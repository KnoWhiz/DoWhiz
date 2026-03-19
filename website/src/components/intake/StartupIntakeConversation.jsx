import { useEffect, useId, useMemo, useRef, useState } from 'react';
import { getDoWhizApiBaseUrl } from '../../analytics';
import { supabase } from '../../app/supabaseClient';
import {
  AUTH_POPUP_SUCCESS_EVENT,
  DASHBOARD_PATH,
  createConversationMessage,
  createInitialConversationMessages,
  mapDraftToIntake,
  openDashboardAuthPopup,
  requestStartupIntakeTurn,
  summarizeToolSelections
} from '../../domain/startupIntake';
import { createValidatedWorkspaceBlueprintFromIntake, saveWorkspaceBlueprint } from '../../domain/workspaceBlueprint';

const DEFAULT_COMPOSER_PLACEHOLDER =
  'Share project details or answer the latest question... (Enter to send, Cmd+Enter for new line)';

function StartupIntakeConversation({
  variant = 'page',
  className = '',
  intakeAriaLabel = 'Conversational workspace intake',
  composerPlaceholder = DEFAULT_COMPOSER_PLACEHOLDER,
  showDraftDetails = true,
  showBlueprintDetails = true,
  footerActions = null,
  onViewed,
  onStarted,
  onSubmitted,
  onHandoff
}) {
  const [messages, setMessages] = useState(() => createInitialConversationMessages());
  const [inputValue, setInputValue] = useState('');
  const [isSending, setIsSending] = useState(false);
  const [errors, setErrors] = useState([]);
  const [blueprint, setBlueprint] = useState(null);
  const [intakeDraft, setIntakeDraft] = useState(null);
  const [missingFields, setMissingFields] = useState([]);
  const [readyForBlueprint, setReadyForBlueprint] = useState(false);
  const [requestError, setRequestError] = useState('');
  const [hasActiveSession, setHasActiveSession] = useState(false);

  const chatFeedRef = useRef(null);
  const startedRef = useRef(false);
  const inputId = useId();

  const draftJson = useMemo(() => (intakeDraft ? JSON.stringify(intakeDraft, null, 2) : ''), [intakeDraft]);
  const blueprintJson = useMemo(() => (blueprint ? JSON.stringify(blueprint, null, 2) : ''), [blueprint]);

  useEffect(() => {
    onViewed?.();
  }, [onViewed]);

  useEffect(() => {
    const node = chatFeedRef.current;
    if (!node) {
      return;
    }

    node.scrollTop = node.scrollHeight;
  }, [messages]);

  useEffect(() => {
    let isMounted = true;

    supabase.auth.getSession().then(({ data }) => {
      if (!isMounted) {
        return;
      }

      setHasActiveSession(Boolean(data?.session));
    });

    const { data: authStateChange } = supabase.auth.onAuthStateChange((_event, session) => {
      if (!isMounted) {
        return;
      }

      setHasActiveSession(Boolean(session));
    });

    return () => {
      isMounted = false;
      authStateChange?.subscription?.unsubscribe();
    };
  }, []);

  useEffect(() => {
    const handlePopupAuthSuccess = (event) => {
      if (event.origin !== window.location.origin) {
        return;
      }

      if (event.data?.type !== AUTH_POPUP_SUCCESS_EVENT) {
        return;
      }

      setHasActiveSession(true);
      setMessages((prev) => [
        ...prev,
        createConversationMessage('assistant', 'Signed in successfully. Opening Team Workspace now.')
      ]);
      window.location.assign(DASHBOARD_PATH);
    };

    window.addEventListener('message', handlePopupAuthSuccess);
    return () => {
      window.removeEventListener('message', handlePopupAuthSuccess);
    };
  }, []);

  const addAssistantMessage = (text) => {
    setMessages((prev) => [...prev, createConversationMessage('assistant', text)]);
  };

  const resetConversation = () => {
    setMessages(createInitialConversationMessages());
    setInputValue('');
    setIsSending(false);
    setErrors([]);
    setBlueprint(null);
    setIntakeDraft(null);
    setMissingFields([]);
    setReadyForBlueprint(false);
    setRequestError('');
    startedRef.current = false;
  };

  const handleTextSubmit = async (event) => {
    event.preventDefault();
    if (isSending) {
      return;
    }

    const value = inputValue.trim();
    if (!value) {
      return;
    }

    if (!startedRef.current) {
      startedRef.current = true;
      onStarted?.();
    }

    const userMessage = createConversationMessage('user', value);
    const nextMessages = [...messages, userMessage];
    setMessages(nextMessages);
    setInputValue('');
    setErrors([]);
    setBlueprint(null);
    setRequestError('');
    setIsSending(true);

    try {
      const payload = await requestStartupIntakeTurn(getDoWhizApiBaseUrl(), nextMessages, intakeDraft);
      const assistantMessage = String(payload.assistant_message || '').trim();
      setIntakeDraft(payload.intake_draft || null);
      setMissingFields(Array.isArray(payload.missing_fields) ? payload.missing_fields : []);
      setReadyForBlueprint(Boolean(payload.ready_for_blueprint));

      if (assistantMessage) {
        addAssistantMessage(assistantMessage);
      } else {
        addAssistantMessage('I updated the intake JSON. Please continue with any missing details.');
      }
    } catch (error) {
      const message =
        error instanceof Error ? error.message : 'Startup intake conversation request failed.';
      setRequestError(message);
      addAssistantMessage('I could not reach the startup intake model right now. Please try again in a few seconds.');
    } finally {
      setIsSending(false);
    }
  };

  const handleChatComposerKeyDown = (event) => {
    if (event.key !== 'Enter' || event.isComposing) {
      return;
    }

    if (event.metaKey || event.ctrlKey || event.shiftKey || event.altKey) {
      return;
    }

    event.preventDefault();
    event.currentTarget.form?.requestSubmit();
  };

  const emitHandoff = (method) => {
    onHandoff?.({
      method,
      has_active_session: hasActiveSession,
      destination: DASHBOARD_PATH
    });
  };

  const handleCreateBlueprint = () => {
    setErrors([]);
    setRequestError('');

    onSubmitted?.({
      has_draft: Boolean(intakeDraft),
      ready_for_blueprint: readyForBlueprint,
      missing_fields: missingFields
    });

    if (!intakeDraft) {
      addAssistantMessage('I need more intake context first. Please describe your project to continue.');
      return;
    }

    if (!readyForBlueprint) {
      addAssistantMessage(
        `I still need a few fields before creating the blueprint:\n- ${
          missingFields.length ? missingFields.join('\n- ') : 'additional details'
        }`
      );
      return;
    }

    const intake = mapDraftToIntake(intakeDraft);
    const result = createValidatedWorkspaceBlueprintFromIntake(intake);

    if (!result.is_valid) {
      setErrors(result.errors);
      setBlueprint(null);
      addAssistantMessage(`Validation still failed:\n- ${result.errors.join('\n- ')}`);
      return;
    }

    saveWorkspaceBlueprint(result.blueprint);
    setBlueprint(result.blueprint);
    setErrors([]);

    if (hasActiveSession) {
      addAssistantMessage('Blueprint saved. Opening Team Workspace now.');
      emitHandoff('direct');
      window.location.assign(DASHBOARD_PATH);
      return;
    }

    const authPopup = openDashboardAuthPopup();
    if (authPopup) {
      authPopup.focus();
      addAssistantMessage(
        'Blueprint saved. Sign in or sign up in the popup. It will close automatically after success.'
      );
      emitHandoff('popup');
      return;
    }

    addAssistantMessage(
      'Blueprint saved. Popup was blocked, so redirecting to sign in. After auth, Team Workspace will reflect your blueprint.'
    );
    emitHandoff('redirect');
    window.location.assign(DASHBOARD_PATH);
  };

  const rootClassName = [
    'startup-intake-conversation',
    `startup-intake-conversation-${variant}`,
    className
  ]
    .filter(Boolean)
    .join(' ');
  const sectionClassName = variant === 'page' ? 'route-section' : 'intake-conversation-section';

  return (
    <div className={rootClassName}>
      <section className={`${sectionClassName} intake-chat-shell`} aria-label={intakeAriaLabel}>
        <div className="intake-chat-feed" ref={chatFeedRef} aria-live="polite" aria-relevant="additions text">
          {messages.map((message) => (
            <article key={message.id} className={`intake-chat-message is-${message.role}`}>
              <p>{message.text}</p>
            </article>
          ))}
        </div>

        <form className="intake-chat-composer" onSubmit={handleTextSubmit}>
          <label className="visually-hidden" htmlFor={inputId}>
            Describe your project
          </label>
          <textarea
            id={inputId}
            className="intake-chat-input"
            value={inputValue}
            onChange={(event) => setInputValue(event.target.value)}
            onKeyDown={handleChatComposerKeyDown}
            placeholder={composerPlaceholder}
            disabled={isSending}
            rows={2}
            aria-busy={isSending}
          />
          <button
            type="submit"
            className="btn btn-primary intake-chat-send-btn"
            disabled={isSending}
            aria-disabled={isSending}
          >
            {isSending ? 'Thinking...' : 'Send'}
          </button>
        </form>
      </section>

      {requestError ? (
        <section className={`${sectionClassName} intake-errors`} aria-live="polite" role="status">
          <h2>Conversation API error</h2>
          <ul>
            <li>{requestError}</li>
          </ul>
        </section>
      ) : null}

      {intakeDraft ? (
        <section className={`${sectionClassName} intake-conversation-state`} aria-live="polite">
          {showDraftDetails ? (
            <>
              <h2>Current JSON Draft</h2>
              <p>The model updates this draft every turn.</p>
            </>
          ) : null}
          <p className="workspace-inline-note">
            {readyForBlueprint
              ? 'Ready to create blueprint.'
              : `Missing fields: ${missingFields.length ? missingFields.join(', ') : 'waiting for more details'}`}
          </p>
          {showDraftDetails ? (
            <details className="intake-advanced intake-conversation-details">
              <summary>Current JSON draft</summary>
              <pre className="intake-conversation-summary">{summarizeToolSelections(intakeDraft)}</pre>
              <pre className="intake-blueprint-preview">{draftJson}</pre>
            </details>
          ) : null}
        </section>
      ) : null}

      {errors.length ? (
        <section className={`${sectionClassName} intake-errors`} aria-live="polite" role="status">
          <h2>Blueprint validation issues</h2>
          <ul>
            {errors.map((error) => (
              <li key={error}>{error}</li>
            ))}
          </ul>
        </section>
      ) : null}

      <div className="route-actions intake-conversation-actions">
        <button type="button" className="btn btn-primary" onClick={handleCreateBlueprint}>
          Create blueprint now
        </button>
        <button type="button" className="btn btn-secondary" onClick={resetConversation}>
          Restart chat
        </button>
        {footerActions}
      </div>

      {showBlueprintDetails && blueprint ? (
        <section className={`${sectionClassName} intake-conversation-state`} aria-live="polite">
          <h2>Blueprint saved</h2>
          <p>Your team blueprint is saved locally and now appears in your dashboard workspace section.</p>
          <details className="intake-advanced">
            <summary>View blueprint JSON</summary>
            <pre className="intake-blueprint-preview">{blueprintJson}</pre>
          </details>
        </section>
      ) : null}
    </div>
  );
}

export default StartupIntakeConversation;

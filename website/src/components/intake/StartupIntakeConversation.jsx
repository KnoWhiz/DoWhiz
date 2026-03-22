import { useEffect, useId, useMemo, useRef, useState } from 'react';
import { getDoWhizApiBaseUrl } from '../../analytics';
import { supabase } from '../../app/supabaseClient';
import {
  AUTH_POPUP_SUCCESS_EVENT,
  DASHBOARD_PATH,
  DEFAULT_INTAKE_CONVERSATION_COPY,
  createConversationMessage,
  createInitialConversationMessages,
  mapDraftToIntake,
  openDashboardAuthPopup,
  requestStartupIntakeTurn,
  summarizeToolSelections
} from '../../domain/startupIntake';
import { createValidatedWorkspaceBlueprintFromIntake, saveWorkspaceBlueprint } from '../../domain/workspaceBlueprint';

function StartupIntakeConversation({
  variant = 'page',
  className = '',
  intakeAriaLabel,
  composerPlaceholder,
  copy = DEFAULT_INTAKE_CONVERSATION_COPY,
  showDraftDetails = true,
  showBlueprintDetails = true,
  footerActions = null,
  onViewed,
  onStarted,
  onSubmitted,
  onHandoff
}) {
  const resolvedCopy = useMemo(
    () => ({
      ...DEFAULT_INTAKE_CONVERSATION_COPY,
      ...copy,
      intakeAriaLabel:
        intakeAriaLabel ??
        copy?.intakeAriaLabel ??
        DEFAULT_INTAKE_CONVERSATION_COPY.intakeAriaLabel,
      composerPlaceholder:
        composerPlaceholder ??
        copy?.composerPlaceholder ??
        DEFAULT_INTAKE_CONVERSATION_COPY.composerPlaceholder
    }),
    [composerPlaceholder, copy, intakeAriaLabel]
  );
  const [messages, setMessages] = useState(() =>
    createInitialConversationMessages(resolvedCopy.initialAssistantPrompt)
  );
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
        createConversationMessage('assistant', resolvedCopy.signedInSuccessOpeningWorkspace)
      ]);
      window.location.assign(DASHBOARD_PATH);
    };

    window.addEventListener('message', handlePopupAuthSuccess);
    return () => {
      window.removeEventListener('message', handlePopupAuthSuccess);
    };
  }, [resolvedCopy.signedInSuccessOpeningWorkspace]);

  const addAssistantMessage = (text) => {
    setMessages((prev) => [...prev, createConversationMessage('assistant', text)]);
  };

  const resetConversation = () => {
    setMessages(createInitialConversationMessages(resolvedCopy.initialAssistantPrompt));
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
        addAssistantMessage(resolvedCopy.intakeJsonUpdatedFallback);
      }
    } catch (error) {
      const message =
        error instanceof Error ? error.message : 'Startup intake conversation request failed.';
      setRequestError(message);
      addAssistantMessage(resolvedCopy.modelUnavailable);
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
      addAssistantMessage(resolvedCopy.needMoreContext);
      return;
    }

    if (!readyForBlueprint) {
      addAssistantMessage(resolvedCopy.missingFieldsBeforeBlueprint(missingFields));
      return;
    }

    const intake = mapDraftToIntake(intakeDraft);
    const result = createValidatedWorkspaceBlueprintFromIntake(intake);

    if (!result.is_valid) {
      setErrors(result.errors);
      setBlueprint(null);
      addAssistantMessage(resolvedCopy.validationFailed(result.errors));
      return;
    }

    saveWorkspaceBlueprint(result.blueprint);
    setBlueprint(result.blueprint);
    setErrors([]);

    if (hasActiveSession) {
      addAssistantMessage(resolvedCopy.blueprintSavedDirect);
      emitHandoff('direct');
      window.location.assign(DASHBOARD_PATH);
      return;
    }

    const authPopup = openDashboardAuthPopup();
    if (authPopup) {
      authPopup.focus();
      addAssistantMessage(resolvedCopy.blueprintSavedPopup);
      emitHandoff('popup');
      return;
    }

    addAssistantMessage(resolvedCopy.blueprintSavedRedirect);
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
      <section className={`${sectionClassName} intake-chat-shell`} aria-label={resolvedCopy.intakeAriaLabel}>
        <div className="intake-chat-feed" ref={chatFeedRef} aria-live="polite" aria-relevant="additions text">
          {messages.map((message) => (
            <article key={message.id} className={`intake-chat-message is-${message.role}`}>
              <p>{message.text}</p>
            </article>
          ))}
        </div>

        <form className="intake-chat-composer" onSubmit={handleTextSubmit}>
          <label className="visually-hidden" htmlFor={inputId}>
            {resolvedCopy.describeProjectLabel}
          </label>
          <textarea
            id={inputId}
            className="intake-chat-input"
            value={inputValue}
            onChange={(event) => setInputValue(event.target.value)}
            onKeyDown={handleChatComposerKeyDown}
            placeholder={resolvedCopy.composerPlaceholder}
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
            {isSending ? resolvedCopy.thinking : resolvedCopy.send}
          </button>
        </form>
      </section>

      {requestError ? (
        <section className={`${sectionClassName} intake-errors`} aria-live="polite" role="status">
          <h2>{resolvedCopy.conversationApiErrorTitle}</h2>
          <ul>
            <li>{requestError}</li>
          </ul>
        </section>
      ) : null}

      {intakeDraft ? (
        <section className={`${sectionClassName} intake-conversation-state`} aria-live="polite">
          {showDraftDetails ? (
            <>
              <h2>{resolvedCopy.currentJsonDraftTitle}</h2>
              <p>{resolvedCopy.currentJsonDraftDescription}</p>
            </>
          ) : null}
          <p className="workspace-inline-note">
            {readyForBlueprint
              ? resolvedCopy.readyToCreateBlueprint
              : resolvedCopy.missingFieldsStatus(missingFields)}
          </p>
          {showDraftDetails ? (
            <details className="intake-advanced intake-conversation-details">
              <summary>{resolvedCopy.currentJsonDraftSummary}</summary>
              <pre className="intake-conversation-summary">{summarizeToolSelections(intakeDraft)}</pre>
              <pre className="intake-blueprint-preview">{draftJson}</pre>
            </details>
          ) : null}
        </section>
      ) : null}

      {errors.length ? (
        <section className={`${sectionClassName} intake-errors`} aria-live="polite" role="status">
          <h2>{resolvedCopy.blueprintValidationIssuesTitle}</h2>
          <ul>
            {errors.map((error) => (
              <li key={error}>{error}</li>
            ))}
          </ul>
        </section>
      ) : null}

      <div className="route-actions intake-conversation-actions">
        <button type="button" className="btn btn-primary" onClick={handleCreateBlueprint}>
          {resolvedCopy.createBlueprintNow}
        </button>
        <button type="button" className="btn btn-secondary" onClick={resetConversation}>
          {resolvedCopy.restartChat}
        </button>
        {footerActions}
      </div>

      {showBlueprintDetails && blueprint ? (
        <section className={`${sectionClassName} intake-conversation-state`} aria-live="polite">
          <h2>{resolvedCopy.blueprintSavedTitle}</h2>
          <p>{resolvedCopy.blueprintSavedDescription}</p>
          <details className="intake-advanced">
            <summary>{resolvedCopy.viewBlueprintJson}</summary>
            <pre className="intake-blueprint-preview">{blueprintJson}</pre>
          </details>
        </section>
      ) : null}
    </div>
  );
}

export default StartupIntakeConversation;

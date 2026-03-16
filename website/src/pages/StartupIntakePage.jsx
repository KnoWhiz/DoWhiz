import { useEffect, useMemo, useRef, useState } from 'react';
import { Link } from 'react-router-dom';
import {
  createFounderIntakeDefaults,
  createValidatedWorkspaceBlueprintFromIntake,
  saveWorkspaceBlueprint
} from '../domain/workspaceBlueprint';

const DASHBOARD_PATH = '/auth/index.html?loggedIn=true#section-workspace';

const DEFAULT_RESOURCE_SELECTIONS = {
  build_system: 'github',
  formal_docs: 'google_docs',
  coordination: 'slack',
  external_execution: 'email'
};

const TOOL_LABELS = {
  github: 'GitHub',
  gitlab: 'GitLab',
  bitbucket: 'Bitbucket',
  google_docs: 'Google Docs',
  notion: 'Notion',
  slack: 'Slack',
  discord: 'Discord',
  email: 'Email'
};

const CHAT_STEPS = {
  PROJECT_DESCRIPTION: 'project_description',
  FOUNDER_NAME: 'founder_name',
  VENTURE_NAME: 'venture_name',
  GOALS: 'goals',
  LAUNCH_MODE: 'launch_mode',
  RESOURCE_CATEGORY: 'resource_category',
  CONFIRM: 'confirm',
  COMPLETED: 'completed'
};

const LAUNCH_MODE_OPTIONS = [
  {
    value: 'default',
    label: 'Use default resource launch',
    description: 'Fastest path: GitHub + Google Docs + Slack + Email.'
  },
  {
    value: 'custom',
    label: 'Choose tools step by step',
    description: 'Pick one tool in each resource category.'
  }
];

const RESOURCE_CATEGORY_FLOW = [
  {
    id: 'build_system',
    label: 'Build System',
    options: [
      { value: 'github', label: 'GitHub' },
      { value: 'gitlab', label: 'GitLab' },
      { value: 'bitbucket', label: 'Bitbucket' }
    ]
  },
  {
    id: 'formal_docs',
    label: 'Formal Docs',
    options: [
      { value: 'google_docs', label: 'Google Docs' },
      { value: 'notion', label: 'Notion' }
    ]
  },
  {
    id: 'coordination',
    label: 'Coordination',
    options: [
      { value: 'slack', label: 'Slack' },
      { value: 'discord', label: 'Discord' },
      { value: 'email', label: 'Email' }
    ]
  },
  {
    id: 'external_execution',
    label: 'External Execution',
    options: [
      { value: 'email', label: 'Email' },
      { value: 'slack', label: 'Slack' },
      { value: 'discord', label: 'Discord' }
    ]
  }
];

const CONFIRM_OPTIONS = [
  {
    value: 'create',
    label: 'Create blueprint now',
    description: 'Save team setup and continue to dashboard.'
  },
  {
    value: 'restart',
    label: 'Start over',
    description: 'Clear this chat and restart intake.'
  }
];

function createMessage(role, text) {
  return {
    id: `${role}-${Date.now()}-${Math.random().toString(36).slice(2, 10)}`,
    role,
    text
  };
}

function createInitialMessages() {
  return [
    createMessage(
      'assistant',
      'Describe the project you want to start. I will build your agent-team workspace setup from that.'
    )
  ];
}

function applyResourceSelectionsToIntake(intake, selections) {
  const next = {
    ...intake
  };

  const channels = [];
  const addChannel = (channel) => {
    if (!channels.includes(channel)) {
      channels.push(channel);
    }
  };

  const buildSystem = selections.build_system || DEFAULT_RESOURCE_SELECTIONS.build_system;
  if (buildSystem === 'github') {
    addChannel('GitHub');
  }

  const formalDocs = selections.formal_docs || DEFAULT_RESOURCE_SELECTIONS.formal_docs;
  if (formalDocs === 'google_docs') {
    addChannel('Google Docs');
  }

  const coordination = selections.coordination || DEFAULT_RESOURCE_SELECTIONS.coordination;
  if (coordination === 'slack') {
    addChannel('Slack');
  }
  if (coordination === 'discord') {
    addChannel('Discord');
  }
  if (coordination === 'email') {
    addChannel('Email');
  }

  const externalExecution = selections.external_execution || DEFAULT_RESOURCE_SELECTIONS.external_execution;
  if (externalExecution === 'email') {
    addChannel('Email');
  }
  if (externalExecution === 'slack') {
    addChannel('Slack');
  }
  if (externalExecution === 'discord') {
    addChannel('Discord');
  }

  if (channels.length === 0) {
    addChannel('Email');
  }

  const hasExistingRepo = buildSystem === 'github' || buildSystem === 'gitlab' || buildSystem === 'bitbucket';

  next.preferred_channels = channels;
  next.has_existing_repo = hasExistingRepo;
  next.primary_repo_provider = hasExistingRepo ? buildSystem : 'github';
  next.has_docs_workspace = formalDocs === 'google_docs' || formalDocs === 'notion';

  return next;
}

function summarizeSelections(selections) {
  return RESOURCE_CATEGORY_FLOW.map((category) => {
    const tool = selections[category.id];
    const label = TOOL_LABELS[tool] || tool || 'Not selected';
    return `${category.label}: ${label}`;
  }).join('\n');
}

function StartupIntakePage() {
  const [intake, setIntake] = useState(() => createFounderIntakeDefaults());
  const [messages, setMessages] = useState(() => createInitialMessages());
  const [inputValue, setInputValue] = useState('');
  const [step, setStep] = useState(CHAT_STEPS.PROJECT_DESCRIPTION);
  const [resourceIndex, setResourceIndex] = useState(0);
  const [launchMode, setLaunchMode] = useState(null);
  const [resourceSelections, setResourceSelections] = useState(() => ({ ...DEFAULT_RESOURCE_SELECTIONS }));
  const [errors, setErrors] = useState([]);
  const [blueprint, setBlueprint] = useState(null);
  const chatFeedRef = useRef(null);

  const blueprintJson = useMemo(
    () => (blueprint ? JSON.stringify(blueprint, null, 2) : ''),
    [blueprint]
  );

  const activeResourceCategory = RESOURCE_CATEGORY_FLOW[resourceIndex] || null;
  const isTextInputStep =
    step === CHAT_STEPS.PROJECT_DESCRIPTION ||
    step === CHAT_STEPS.FOUNDER_NAME ||
    step === CHAT_STEPS.VENTURE_NAME ||
    step === CHAT_STEPS.GOALS;

  const chatPlaceholder = (() => {
    if (step === CHAT_STEPS.PROJECT_DESCRIPTION) {
      return 'Example: AI onboarding copilot for B2B SaaS customer success teams...';
    }
    if (step === CHAT_STEPS.FOUNDER_NAME) {
      return 'Your name';
    }
    if (step === CHAT_STEPS.VENTURE_NAME) {
      return "Project name (or type 'skip')";
    }
    if (step === CHAT_STEPS.GOALS) {
      return 'Ship MVP, close 3 pilots, launch onboarding analytics...';
    }
    return 'Type your answer...';
  })();

  const activeChoiceOptions = useMemo(() => {
    if (step === CHAT_STEPS.LAUNCH_MODE) {
      return LAUNCH_MODE_OPTIONS;
    }
    if (step === CHAT_STEPS.RESOURCE_CATEGORY && activeResourceCategory) {
      return activeResourceCategory.options;
    }
    if (step === CHAT_STEPS.CONFIRM) {
      return CONFIRM_OPTIONS;
    }
    return [];
  }, [activeResourceCategory, step]);

  useEffect(() => {
    const node = chatFeedRef.current;
    if (!node) {
      return;
    }
    node.scrollTop = node.scrollHeight;
  }, [messages]);

  const addAssistantMessage = (text) => {
    setMessages((prev) => [...prev, createMessage('assistant', text)]);
  };

  const addUserMessage = (text) => {
    setMessages((prev) => [...prev, createMessage('user', text)]);
  };

  const resetConversation = () => {
    setIntake(createFounderIntakeDefaults());
    setMessages(createInitialMessages());
    setInputValue('');
    setStep(CHAT_STEPS.PROJECT_DESCRIPTION);
    setResourceIndex(0);
    setLaunchMode(null);
    setResourceSelections({ ...DEFAULT_RESOURCE_SELECTIONS });
    setErrors([]);
    setBlueprint(null);
  };

  const finalizeSelections = (mode, selections) => {
    setLaunchMode(mode);
    setResourceSelections(selections);
    setIntake((prev) => applyResourceSelectionsToIntake(prev, selections));
    addAssistantMessage(
      `Great. I mapped your resource launch to:\n${summarizeSelections(selections)}\n\nReady to create your agent team blueprint?`
    );
    setStep(CHAT_STEPS.CONFIRM);
    setErrors([]);
  };

  const askNextResourceCategory = (nextIndex) => {
    const category = RESOURCE_CATEGORY_FLOW[nextIndex];
    if (!category) {
      return;
    }
    setResourceIndex(nextIndex);
    addAssistantMessage(`Choose one tool for ${category.label}.`);
    setStep(CHAT_STEPS.RESOURCE_CATEGORY);
  };

  const createBlueprintFromConversation = () => {
    const finalIntake = applyResourceSelectionsToIntake(intake, resourceSelections);
    setIntake(finalIntake);

    const result = createValidatedWorkspaceBlueprintFromIntake(finalIntake);
    if (!result.is_valid) {
      setErrors(result.errors);
      setBlueprint(null);
      addAssistantMessage(`I still need a few fields:\n- ${result.errors.join('\n- ')}`);
      return;
    }

    saveWorkspaceBlueprint(result.blueprint);
    setErrors([]);
    setBlueprint(result.blueprint);
    addAssistantMessage(
      'Blueprint saved. You can open your unified dashboard now, or restart this chat to adjust the setup.'
    );
    setStep(CHAT_STEPS.COMPLETED);
  };

  const handleTextSubmit = (event) => {
    event.preventDefault();
    if (!isTextInputStep) {
      return;
    }

    const value = inputValue.trim();
    if (!value) {
      return;
    }

    setInputValue('');
    addUserMessage(value);
    setErrors([]);
    setBlueprint(null);

    if (step === CHAT_STEPS.PROJECT_DESCRIPTION) {
      setIntake((prev) => ({
        ...prev,
        venture_thesis: value
      }));
      addAssistantMessage('Great context. What should I call you?');
      setStep(CHAT_STEPS.FOUNDER_NAME);
      return;
    }

    if (step === CHAT_STEPS.FOUNDER_NAME) {
      if (value.toLowerCase() === 'skip') {
        addAssistantMessage('I need your name to create the team blueprint. What should I call you?');
        return;
      }

      setIntake((prev) => ({
        ...prev,
        founder_name: value
      }));
      addAssistantMessage("What is the project or company name? You can type 'skip' if undecided.");
      setStep(CHAT_STEPS.VENTURE_NAME);
      return;
    }

    if (step === CHAT_STEPS.VENTURE_NAME) {
      if (value.toLowerCase() !== 'skip') {
        setIntake((prev) => ({
          ...prev,
          venture_name: value
        }));
      }
      addAssistantMessage(
        'What are your top goals for the next 30-90 days? You can answer in one line or comma-separated.'
      );
      setStep(CHAT_STEPS.GOALS);
      return;
    }

    if (step === CHAT_STEPS.GOALS) {
      if (value.toLowerCase() === 'skip') {
        addAssistantMessage('I need at least one goal before launch planning. Please share your top goal.');
        return;
      }

      setIntake((prev) => ({
        ...prev,
        goals_text: value
      }));
      addAssistantMessage('How do you want to launch resources?');
      setStep(CHAT_STEPS.LAUNCH_MODE);
    }
  };

  const handleChoiceSelect = (option) => {
    addUserMessage(option.label);
    setErrors([]);
    setBlueprint(null);

    if (step === CHAT_STEPS.LAUNCH_MODE) {
      if (option.value === 'default') {
        finalizeSelections('default', { ...DEFAULT_RESOURCE_SELECTIONS });
        return;
      }

      setLaunchMode('custom');
      setResourceSelections({});
      askNextResourceCategory(0);
      return;
    }

    if (step === CHAT_STEPS.RESOURCE_CATEGORY && activeResourceCategory) {
      const nextSelections = {
        ...resourceSelections,
        [activeResourceCategory.id]: option.value
      };
      const nextIndex = resourceIndex + 1;

      if (nextIndex < RESOURCE_CATEGORY_FLOW.length) {
        setResourceSelections(nextSelections);
        addAssistantMessage(`Noted. ${activeResourceCategory.label}: ${option.label}.`);
        askNextResourceCategory(nextIndex);
        return;
      }

      finalizeSelections('custom', nextSelections);
      return;
    }

    if (step === CHAT_STEPS.CONFIRM) {
      if (option.value === 'restart') {
        resetConversation();
        return;
      }
      createBlueprintFromConversation();
    }
  };

  return (
    <main className="route-shell route-shell-intake">
      <div className="route-card route-card-intake">
        <p className="route-kicker">Conversational Intake</p>
        <h1>Create Your Agent Team</h1>
        <p>
          Start by describing your project in chat. Then pick default launch or configure each resource category one
          step at a time.
        </p>

        <section className="route-section intake-chat-shell" aria-label="Conversational workspace intake">
          <div className="intake-chat-feed" ref={chatFeedRef}>
            {messages.map((message) => (
              <article key={message.id} className={`intake-chat-message is-${message.role}`}>
                <p>{message.text}</p>
              </article>
            ))}
          </div>

          {isTextInputStep ? (
            <form className="intake-chat-composer" onSubmit={handleTextSubmit}>
              <input
                type="text"
                className="intake-chat-input"
                value={inputValue}
                onChange={(event) => setInputValue(event.target.value)}
                placeholder={chatPlaceholder}
              />
              <button type="submit" className="btn btn-primary intake-chat-send-btn">
                Send
              </button>
            </form>
          ) : null}

          {activeChoiceOptions.length ? (
            <div className="intake-chat-options">
              {activeChoiceOptions.map((option) => (
                <button
                  key={option.value}
                  type="button"
                  className="intake-chat-option"
                  onClick={() => handleChoiceSelect(option)}
                >
                  <span>{option.label}</span>
                  {option.description ? <small>{option.description}</small> : null}
                </button>
              ))}
            </div>
          ) : null}
        </section>

        {launchMode ? (
          <section className="route-section">
            <h2>Launch Plan</h2>
            <pre className="intake-conversation-summary">{summarizeSelections(resourceSelections)}</pre>
          </section>
        ) : null}

        {errors.length ? (
          <section className="route-section intake-errors" aria-live="polite">
            <h2>Blueprint validation issues</h2>
            <ul>
              {errors.map((error) => (
                <li key={error}>{error}</li>
              ))}
            </ul>
          </section>
        ) : null}

        <div className="route-actions">
          <button type="button" className="btn btn-secondary" onClick={resetConversation}>
            Restart chat
          </button>
          <a className="btn btn-secondary" href={DASHBOARD_PATH}>
            Open dashboard
          </a>
          <Link className="btn btn-secondary" to="/">
            Back to landing
          </Link>
        </div>

        {blueprint ? (
          <section className="route-section">
            <h2>Blueprint saved</h2>
            <p>
              Your team blueprint is saved locally and now appears in your dashboard workspace section.
            </p>
            <details className="intake-advanced">
              <summary>View blueprint JSON</summary>
              <pre className="intake-blueprint-preview">{blueprintJson}</pre>
            </details>
          </section>
        ) : null}
      </div>
    </main>
  );
}

export default StartupIntakePage;

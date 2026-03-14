import { useState, useEffect, useRef } from 'react';
import {
  getOrCreateSessionId,
  persistAttributionFromLocation,
  trackAnalyticsEvent
} from '../analytics';
import { supabase } from '../app/supabaseClient';
import oliverImg from '../assets/Oliver.jpg';
import miniMouseImg from '../assets/Mini-Mouse.jpg';
import stickyOctopusImg from '../assets/Sticky-Octopus.jpg';
import skyDragonImg from '../assets/Sky-Dragon.jpg';
import cozyLobsterImg from '../assets/Cozy-Lobster.jpg';
import struttonPigeonImg from '../assets/Strutton-Pigeon.jpg';
import fluffyElephantImg from '../assets/Fluffy-Elephant.jpg';
import plushAxolotlImg from '../assets/Plush-Axolotl.jpg';
import MouseField from '../components/landing/MouseField';
import {
  getNextThemeSwitch,
  getThemeForLocalTime,
  shouldEnableMouseField
} from '../components/landing/mouseFieldUtils';

const SITE_URL = 'https://dowhiz.com';
const LOGO_URL = `${SITE_URL}/assets/DoWhiz.svg`;
const SUPPORT_EMAIL = 'admin@dowhiz.com';
const ORG_NAME = 'DoWhiz';
const MAX_TOTAL_UPLOAD_BYTES = 10 * 1024 * 1024;
const CN_PATH_PREFIX = '/cn';
const DEPLOYMENT_FORM_DEFAULTS = {
  full_name: '',
  work_email: '',
  agent_hourly_rate_usd: '',
  team_name: '',
  azure_region: '',
  agent_name: '',
  wallet_id: '',
  contact_channel: '',
  workspace_paths: 'Not provided',
  use_case: ''
};
const DEPLOYMENT_REGIONS = ['eastus', 'westus2', 'centralus', 'westeurope', 'southeastasia', 'other'];

const isCnPath = (pathname = '/') =>
  pathname === CN_PATH_PREFIX || pathname === `${CN_PATH_PREFIX}/` || pathname.startsWith(`${CN_PATH_PREFIX}/`);

const getLocalizedAuthPath = (suffix = '', pathname = typeof window !== 'undefined' ? window.location.pathname : '/') =>
  `${isCnPath(pathname) ? CN_PATH_PREFIX : ''}/auth/index.html${suffix}`;

function LandingPage() {
  const [theme, setTheme] = useState(() => getThemeForLocalTime());
  const [enableMouseField, setEnableMouseField] = useState(false);
  const [user, setUser] = useState(null);
  const [showUserMenu, setShowUserMenu] = useState(false);
  const [navHidden, setNavHidden] = useState(false);
  const [deploymentForm, setDeploymentForm] = useState(DEPLOYMENT_FORM_DEFAULTS);
  const [skillsFiles, setSkillsFiles] = useState([]);
  const [privateFiles, setPrivateFiles] = useState([]);
  const [envRows, setEnvRows] = useState([{ key: '', value: '', show: false }]);
  const [confirmOwnership, setConfirmOwnership] = useState(false);
  const [confirmMvp, setConfirmMvp] = useState(false);
  const [deploymentStatus, setDeploymentStatus] = useState({ message: '', type: '' });
  const [isSubmittingDeployment, setIsSubmittingDeployment] = useState(false);
  const userMenuRef = useRef(null);
  const skillsInputRef = useRef(null);
  const privateInputRef = useRef(null);
  const lastScrollY = useRef(0);

  useEffect(() => {
    if (typeof window === 'undefined') {
      return;
    }
    const { hash, pathname } = window.location;
    if (!hash || pathname.startsWith('/auth')) {
      return;
    }
    const params = new URLSearchParams(hash.substring(1));
    const hasTokens = params.get('access_token') && params.get('refresh_token');
    const hasError = params.get('error') || params.get('error_description');
    if (hasTokens || hasError) {
      window.location.replace(getLocalizedAuthPath(hash, pathname));
    }
  }, []);

  useEffect(() => {
    if (typeof window === 'undefined') {
      return;
    }

    persistAttributionFromLocation();
    const sessionId = getOrCreateSessionId();
    trackAnalyticsEvent(
      'landing_page_view',
      {
        landing_page_variant: 'startup_workspace_phase1',
        landing_page_variant_legacy: 'default',
        language: isCnPath(window.location.pathname) ? 'zh-CN' : 'en-US'
      },
      {
        eventKey: `landing_page_view:${sessionId}:${window.location.pathname}`
      }
    );
  }, []);

  useEffect(() => {
    if (typeof window === 'undefined') {
      return undefined;
    }

    const scrollToHashTarget = () => {
      const hash = window.location.hash;
      if (!hash) {
        return;
      }

      const targetId = decodeURIComponent(hash.replace(/^#/, ''));
      const target = document.getElementById(targetId);
      if (target) {
        target.scrollIntoView({ behavior: 'auto', block: 'start' });
      }
    };

    const timeoutId = window.setTimeout(scrollToHashTarget, 120);
    window.addEventListener('hashchange', scrollToHashTarget);

    return () => {
      window.clearTimeout(timeoutId);
      window.removeEventListener('hashchange', scrollToHashTarget);
    };
  }, []);

  // Close dropdown when clicking outside
  useEffect(() => {
    const handleClickOutside = (event) => {
      if (userMenuRef.current && !userMenuRef.current.contains(event.target)) {
        setShowUserMenu(false);
      }
    };
    document.addEventListener('mousedown', handleClickOutside);
    return () => document.removeEventListener('mousedown', handleClickOutside);
  }, []);

  // Check for Supabase session on load
  useEffect(() => {
    console.log('App: Checking for Supabase session...');
    supabase.auth.getSession().then(({ data: { session } }) => {
      console.log('App: getSession result:', session);
      console.log('App: User:', session?.user);
      setUser(session?.user ?? null);
    });

    const { data: { subscription } } = supabase.auth.onAuthStateChange((event, session) => {
      console.log('App: Auth state change:', event, session?.user);
      setUser(session?.user ?? null);
    });

    return () => subscription.unsubscribe();
  }, []);

  useEffect(() => {
    let timeoutId;

    const updateTheme = () => {
      setTheme(getThemeForLocalTime());
    };

    const scheduleNextSwitch = () => {
      const now = new Date();
      const nextSwitch = getNextThemeSwitch(now);
      const delay = Math.max(nextSwitch.getTime() - now.getTime(), 0);

      timeoutId = window.setTimeout(() => {
        updateTheme();
        scheduleNextSwitch();
      }, delay);
    };

    updateTheme();
    scheduleNextSwitch();

    return () => {
      if (timeoutId) {
        window.clearTimeout(timeoutId);
      }
    };
  }, []);

  useEffect(() => {
    document.documentElement.setAttribute('data-theme', theme);
  }, [theme]);

  // Hide navbar on scroll down, show on scroll up
  useEffect(() => {
    const handleScroll = () => {
      const currentScrollY = window.scrollY;
      const scrollThreshold = 100;

      if (currentScrollY > lastScrollY.current && currentScrollY > scrollThreshold) {
        setNavHidden(true);
      } else {
        setNavHidden(false);
      }
      lastScrollY.current = currentScrollY;
    };

    window.addEventListener('scroll', handleScroll, { passive: true });
    return () => window.removeEventListener('scroll', handleScroll);
  }, []);

  useEffect(() => {
    if (!shouldEnableMouseField()) {
      return undefined;
    }

    let idleId;
    let timeoutId;

    const revealMouseField = () => {
      setEnableMouseField(true);
    };

    if ('requestIdleCallback' in window) {
      idleId = window.requestIdleCallback(revealMouseField, { timeout: 1500 });
      return () => {
        if (typeof window.cancelIdleCallback === 'function') {
          window.cancelIdleCallback(idleId);
        }
      };
    }

    timeoutId = window.setTimeout(revealMouseField, 800);
    return () => {
      if (timeoutId) {
        window.clearTimeout(timeoutId);
      }
    };
  }, []);

  useEffect(() => {
    const contentLayer = document.querySelector('.content-layer');
    const sections = Array.from(
      document.querySelectorAll('.content-layer > .hero-section, .content-layer > .section')
    );

    if (!contentLayer || !sections.length) {
      return undefined;
    }

    const getActiveSection = () => {
      const layerRect = contentLayer.getBoundingClientRect();
      const probeY = layerRect.top + contentLayer.clientHeight * 0.35;

      return (
        sections.find((section) => {
          const rect = section.getBoundingClientRect();
          return rect.top <= probeY && rect.bottom >= probeY;
        }) || sections[0]
      );
    };

    const updateSnapMode = () => {
      const activeSection = getActiveSection();
      const shouldRelaxSnap = activeSection?.classList.contains('snap-free');
      contentLayer.classList.toggle('snap-relaxed', Boolean(shouldRelaxSnap));
    };

    const updateSnapTargets = () => {
      const viewportHeight = window.innerHeight;

      sections.forEach((section) => {
        if (section.classList.contains('hero-section')) {
          section.classList.remove('snap-free');
          return;
        }

        const requiresFreeScroll = section.scrollHeight > viewportHeight * 1.02;
        section.classList.toggle('snap-free', requiresFreeScroll);
      });

      updateSnapMode();
    };

    updateSnapTargets();
    const timeoutId = window.setTimeout(updateSnapTargets, 250);
    window.addEventListener('resize', updateSnapTargets);
    window.addEventListener('load', updateSnapTargets);
    contentLayer.addEventListener('scroll', updateSnapMode, { passive: true });

    const observer = new ResizeObserver(() => {
      updateSnapTargets();
    });

    sections.forEach((section) => observer.observe(section));

    return () => {
      window.clearTimeout(timeoutId);
      window.removeEventListener('resize', updateSnapTargets);
      window.removeEventListener('load', updateSnapTargets);
      contentLayer.removeEventListener('scroll', updateSnapMode);
      contentLayer.classList.remove('snap-relaxed');
      observer.disconnect();
    };
  }, []);

  // Equalize intro and example heights per row within roles grid
  useEffect(() => {
    const syncRoleHeights = () => {
      const cards = Array.from(document.querySelectorAll('.roles-grid .role-card'));
      const descs = Array.from(document.querySelectorAll('.roles-grid .role-desc'));
      const examples = Array.from(document.querySelectorAll('.roles-grid .role-example'));

      // reset first
      descs.forEach((el) => (el.style.minHeight = ''));
      examples.forEach((el) => (el.style.minHeight = ''));

      const rows = [];
      cards.forEach((card) => {
        const top = card.offsetTop;
        let row = rows.find((r) => Math.abs(r.top - top) < 4);
        if (!row) {
          row = { top, cards: [] };
          rows.push(row);
        }
        row.cards.push(card);
      });

      rows.forEach((row) => {
        let maxDesc = 0;
        row.cards.forEach((card) => {
          const desc = card.querySelector('.role-desc');
          if (desc) {
            maxDesc = Math.max(maxDesc, desc.offsetHeight);
          }
        });
        row.cards.forEach((card) => {
          const desc = card.querySelector('.role-desc');
          if (desc && maxDesc) {
            desc.style.minHeight = `${maxDesc}px`;
          }
        });
      });
    };

    syncRoleHeights();
    window.addEventListener('resize', syncRoleHeights);
    window.addEventListener('load', syncRoleHeights);

    const roleGrid = document.querySelector('.roles-grid');
    const resizeObserver = new ResizeObserver(() => syncRoleHeights());
    if (roleGrid) {
      resizeObserver.observe(roleGrid);
      Array.from(roleGrid.children).forEach((child) => resizeObserver.observe(child));
    }

    return () => {
      window.removeEventListener('resize', syncRoleHeights);
      window.removeEventListener('load', syncRoleHeights);
      resizeObserver.disconnect();
    };
  }, []);

  const buildMailtoLink = (email, subject, body) => {
    const encodedSubject = encodeURIComponent(subject);
    const encodedBody = encodeURIComponent(body);
    return `mailto:${email}?subject=${encodedSubject}&body=${encodedBody}`;
  };

  const formatBytes = (bytes) => {
    if (!Number.isFinite(bytes) || bytes < 0) return '0 B';
    const units = ['B', 'KB', 'MB', 'GB'];
    let value = bytes;
    let idx = 0;
    while (value >= 1024 && idx < units.length - 1) {
      value /= 1024;
      idx += 1;
    }
    return `${value.toFixed(value >= 100 || idx === 0 ? 0 : 1)} ${units[idx]}`;
  };

  const getDeploymentApiBaseUrl = () => {
    if (typeof window !== 'undefined' && window.location.hostname === 'localhost') {
      return 'http://localhost:9001';
    }
    return 'https://api.production1.dowhiz.com/service';
  };

  const setDeploymentStatusMessage = (message, type = '') => {
    setDeploymentStatus({ message, type });
  };

  const handleDeploymentFieldChange = (field) => (event) => {
    const value = event.target.value;
    setDeploymentForm((prev) => ({ ...prev, [field]: value }));
  };

  const handleSkillsFilesChange = (event) => {
    const files = Array.from(event.target.files || []);
    setSkillsFiles(files);
  };

  const handlePrivateFilesChange = (event) => {
    const files = Array.from(event.target.files || []);
    setPrivateFiles(files);
  };

  const addEnvRow = () => {
    setEnvRows((prev) => [...prev, { key: '', value: '', show: false }]);
  };

  const updateEnvRow = (index, field, value) => {
    setEnvRows((prev) =>
      prev.map((row, rowIndex) => (rowIndex === index ? { ...row, [field]: value } : row))
    );
  };

  const toggleEnvVisibility = (index) => {
    setEnvRows((prev) =>
      prev.map((row, rowIndex) => (rowIndex === index ? { ...row, show: !row.show } : row))
    );
  };

  const removeEnvRow = (index) => {
    setEnvRows((prev) => {
      if (prev.length <= 1) {
        setDeploymentStatusMessage('At least one env key row is required.', 'error');
        return prev;
      }
      return prev.filter((_, rowIndex) => rowIndex !== index);
    });
  };

  const resetDeploymentForm = () => {
    setDeploymentForm({ ...DEPLOYMENT_FORM_DEFAULTS });
    setSkillsFiles([]);
    setPrivateFiles([]);
    setEnvRows([{ key: '', value: '', show: false }]);
    setConfirmOwnership(false);
    setConfirmMvp(false);
    setDeploymentStatusMessage('Form reset.', 'info');

    if (skillsInputRef.current) {
      skillsInputRef.current.value = '';
    }
    if (privateInputRef.current) {
      privateInputRef.current.value = '';
    }
  };

  const collectEnvEntries = () => {
    const entries = [];
    const seen = new Set();

    for (const row of envRows) {
      const keyRaw = (row.key || '').trim();
      const valueRaw = row.value || '';

      if (!keyRaw && !valueRaw) {
        continue;
      }
      if (!keyRaw || !valueRaw) {
        throw new Error('Each env row needs both key and value.');
      }

      const key = keyRaw.toUpperCase();
      if (!/^[A-Z][A-Z0-9_]*$/.test(key)) {
        throw new Error(`Invalid env key format: ${keyRaw}`);
      }
      if (seen.has(key)) {
        throw new Error(`Duplicate env key: ${key}`);
      }

      seen.add(key);
      entries.push({ key, value: valueRaw });
    }

    if (!entries.length) {
      throw new Error('At least one env key is required.');
    }

    return entries;
  };

  const validateDeploymentForm = () => {
    const requiredFields = [
      ['full_name', 'Full name'],
      ['work_email', 'Work email'],
      ['azure_region', 'Preferred Azure region'],
      ['agent_name', 'Agent name'],
      ['contact_channel', 'Primary contact channel'],
      ['use_case', 'Primary tasks this agent should run']
    ];

    for (const [key, label] of requiredFields) {
      if (!(deploymentForm[key] || '').trim()) {
        setDeploymentStatusMessage(`${label} is required.`, 'error');
        return null;
      }
    }

    if (!/^[^@\s]+@[^@\s]+\.[^@\s]+$/.test(deploymentForm.work_email.trim())) {
      setDeploymentStatusMessage('Please enter a valid work email address.', 'error');
      return null;
    }

    if (!skillsFiles.length) {
      setDeploymentStatusMessage('Upload at least one skills file.', 'error');
      return null;
    }

    if (!privateFiles.length) {
      setDeploymentStatusMessage('Upload at least one private data folder/file.', 'error');
      return null;
    }

    const combinedBytes = [...skillsFiles, ...privateFiles].reduce((sum, file) => sum + file.size, 0);
    if (combinedBytes > MAX_TOTAL_UPLOAD_BYTES) {
      setDeploymentStatusMessage(
        `Total upload size ${formatBytes(combinedBytes)} exceeds limit ${formatBytes(MAX_TOTAL_UPLOAD_BYTES)}.`,
        'error'
      );
      return null;
    }

    if (!confirmOwnership || !confirmMvp) {
      setDeploymentStatusMessage('Please confirm both review checkboxes before submitting.', 'error');
      return null;
    }

    try {
      const envEntries = collectEnvEntries();
      return { envEntries };
    } catch (error) {
      setDeploymentStatusMessage(error instanceof Error ? error.message : 'Invalid environment keys.', 'error');
      return null;
    }
  };

  const handleDeploymentSubmit = async (event) => {
    event.preventDefault();
    setDeploymentStatusMessage('');

    const validation = validateDeploymentForm();
    if (!validation) {
      return;
    }

    setIsSubmittingDeployment(true);
    setDeploymentStatusMessage('Uploading package and starting manual deployment intake...', 'info');

    try {
      const formData = new FormData();
      formData.append('full_name', deploymentForm.full_name.trim());
      formData.append('work_email', deploymentForm.work_email.trim());
      formData.append('agent_hourly_rate_usd', deploymentForm.agent_hourly_rate_usd.trim());
      formData.append('team_name', deploymentForm.team_name.trim());
      formData.append('azure_region', deploymentForm.azure_region.trim());
      formData.append('agent_name', deploymentForm.agent_name.trim());
      formData.append('wallet_id', deploymentForm.wallet_id.trim());
      formData.append('contact_channel', deploymentForm.contact_channel.trim());
      formData.append('workspace_paths', deploymentForm.workspace_paths || 'Not provided');
      formData.append('use_case', deploymentForm.use_case.trim());
      formData.append('env_keys_json', JSON.stringify(validation.envEntries));

      skillsFiles.forEach((file) => {
        formData.append('skills_files', file, file.webkitRelativePath || file.name);
      });

      privateFiles.forEach((file) => {
        formData.append('private_data_files', file, file.webkitRelativePath || file.name);
      });

      const response = await fetch(`${getDeploymentApiBaseUrl()}/api/agent-market/deploy`, {
        method: 'POST',
        body: formData
      });

      let payload = null;
      try {
        payload = await response.json();
      } catch {
        payload = null;
      }

      if (!response.ok) {
        throw new Error(
          payload?.error || 'Manual deployment intake is temporarily unavailable. Please retry in a few minutes.'
        );
      }

      const requestId = payload?.request_id;
      const etaHours = payload?.eta_hours || 24;
      const requestSuffix = requestId ? ` (request: ${requestId})` : '';
      setDeploymentStatusMessage(
        `Manual deployment request accepted${requestSuffix}. Provisioning is in progress and is usually ready within ${etaHours} hours.`,
        'success'
      );
    } catch (error) {
      const message =
        error instanceof Error ? error.message : 'Failed to submit manual deployment request. Please retry.';
      setDeploymentStatusMessage(message, 'error');
    } finally {
      setIsSubmittingDeployment(false);
    }
  };

  const features = [
    {
      tag: '01',
      title: 'Trigger from every surface',
      desc: 'Start work from email, Slack/Discord messages, GitHub issues, or @mentions in shared Google Docs and Notion comments.',
      icon: '/icons/The%20Digital%20Employee%20Stack/trigger.svg'
    },
    {
      tag: '02',
      title: 'Tool-native execution',
      desc: 'Agents work directly in your docs, project boards, repos, and chat spaces so outputs land where your team already works.',
      icon: '/icons/The%20Digital%20Employee%20Stack/execute.svg'
    },
    {
      tag: '03',
      title: 'Shared memory across channels',
      desc: 'Per-user context carries over across email, chat, issues, and comments so follow-ups do not restart from zero.',
      icon: '/icons/The%20Digital%20Employee%20Stack/shared.svg'
    },
    {
      tag: '04',
      title: 'Agent-owned identities',
      desc: 'Each digital employee has their own account identity. You do not hand over personal credentials to get work done.',
      icon: '/icons/The%20Digital%20Employee%20Stack/agent.svg'
    },
    {
      tag: '05',
      title: 'Permissioned workspace access',
      desc: 'Agents only access workspaces and integrations when you explicitly grant access and can be revoked at any time.',
      icon: '/icons/The%20Digital%20Employee%20Stack/permission.svg'
    },
    {
      tag: '06',
      title: 'Reviewable artifacts + approvals',
      desc: 'Each run returns auditable artifacts and explicit human approval points before external delivery or sensitive actions.',
      icon: '/icons/The%20Digital%20Employee%20Stack/collaboration.svg'
    }
  ];

  const howItWorksSteps = [
    {
      id: '01',
      phase: 'Trigger',
      role: 'Users',
      intro: 'Give DoWhiz a task through:',
      points: [
        'Email with attachments, links, and constraints',
        'Slack/Discord message or @mention in a thread',
        'GitHub issue assignment or shared doc comment mention'
      ],
      output: 'A structured brief with requester context, expected output, and delivery target.'
    },
    {
      id: '02',
      phase: 'Execute',
      role: 'Agent',
      intro: 'Works directly in approved tools with scoped permissions and agent-owned identities.',
      points: [
        'No personal credential handoff required',
        'Workspace permissions are explicit and revocable',
        'Cross-agent coordination for multi-step tasks'
      ],
      output: 'Work artifacts are created where your team already collaborates.'
    },
    {
      id: '03',
      phase: 'Deliver',
      role: 'Agent',
      intro: 'Results return to the originating channel, and shared memory keeps continuity across future requests.',
      points: [
        'PRs, docs, action items, and updates delivered in-thread',
        'Per-user context persists across channels',
        'Follow-ups start with history, not from scratch'
      ],
      output: 'Faster iteration with consistent quality across every surface.'
    }
  ];

  const safetyItems = [
    {
      tag: 'A1',
      title: 'Isolated execution environment',
      icon: '/icons/shield_lock.svg',
      desc: 'Every request runs in an isolated runtime boundary so tasks stay contained, reviewable, and predictable.',
      points: [
        'Separate sandbox/VM boundaries per task',
        'Scoped network and file access controls',
        'Execution logs available for auditing'
      ]
    },
    {
      tag: 'A2',
      title: 'No user credential handoff',
      icon: '/icons/lock_person.svg',
      desc: 'You do not share personal passwords or account credentials with DoWhiz agents to get work done.',
      points: [
        'Agents operate with agent-owned identities',
        'No direct login into your personal accounts',
        'Credential exposure risk is minimized by design'
      ]
    },
    {
      tag: 'A3',
      title: 'Explicit access grants',
      icon: '/icons/key.svg',
      desc: 'Agents can only work in workspaces and integrations that you explicitly authorize and can revoke.',
      points: [
        'Granular workspace-level permission model',
        'Access can be revoked at any time',
        'Only authorized resources are in scope'
      ]
    }
  ];

  const accessFlowSteps = [
    {
      title: 'Grant',
      desc: 'Invite or authorize the agent account in the tool you want to use.'
    },
    {
      title: 'Scope',
      desc: 'Define what project, doc, repo, or channel the agent can access.'
    },
    {
      title: 'Operate',
      desc: 'The agent executes work only inside that approved scope and reports results.'
    },
    {
      title: 'Revoke',
      desc: 'Remove workspace permissions at any time when the task is complete.'
    }
  ];

  const deploymentSteps = [
    {
      id: '01',
      title: 'Submit manual runtime package',
      desc: 'Share operator details, skills files, private data folders, and environment keys for managed runtime setup.'
    },
    {
      id: '02',
      title: 'Validation and provisioning review',
      desc: 'DoWhiz validates the package, checks security requirements, and confirms provisioning readiness.'
    },
    {
      id: '03',
      title: 'Handoff with runbook',
      desc: 'Your managed runtime is provisioned with status updates and a clear manual-next-step runbook.'
    }
  ];

  const deploymentChecklist = [
    'Use this path only when you need managed runtime deployment help.',
    'Upload only the minimum secrets and private files required for this request.',
    'Use uppercase env key names like OPENAI_API_KEY to avoid validation failures.',
    'Provisioning is typically ready within 24 hours after submission.'
  ];

  const workflowExamples = [
    {
      id: 'maggie',
      title: 'Meeting Summary and Follow-up Task Assignment',
      owner: 'Maggie',
      avatar: miniMouseImg,
      mediaType: 'video',
      media: '/icons/workflow%20example/maggie.mov',
      trigger: 'Tell Maggie her tasks in a meeting.',
      execution: [
        'Maggie extracts decisions, actions, dependencies, and owners.',
        'Builds due-date follow-ups and milestone checkpoints.',
        'Prepares a status-ready update for your team channel.',
        'Assigns tasks to other agents.'
      ],
      result: 'You get an owner-tracked execution plan with clear follow-up cadence.'
    },
    {
      id: 'devin',
      title: 'Engineering delivery from GitHub',
      owner: 'Devin',
      avatar: stickyOctopusImg,
      mediaType: 'video',
      media: '/icons/workflow%20example/devin.mov',
      trigger: 'Create a GitHub issue, assign Devin, and include acceptance criteria.',
      execution: [
        'Devin breaks work into implementation checkpoints.',
        'Implements the change and opens a pull request.',
        'Runs tests and posts pass/fail notes with review context.'
      ],
      result: 'You get a PR, test status, and a concise summary in the same issue thread.'
    },
    {
      id: 'oliver',
      title: 'Chat Summary and Todo List from Discord',
      owner: 'Oliver',
      avatar: oliverImg,
      mediaType: 'image',
      media: '/icons/workflow%20example/oliver.png',
      trigger: '@mention Oliver in Discord channel and assign him tasks.',
      execution: [
        'Oliver scans the full conversation history in the channel.',
        'Identifies key decisions, technical conclusions, and shared updates.',
        'Extracts concrete action items with clear ownership and priorities.',
        'Organizes them into a structured, execution-ready checklist.'
      ],
      result: 'You get a concise recap of what happened and a clear, owner-aligned action plan for the next step.'
    }
  ];

  const blogPosts = [
    {
      tag: 'SEO Guide',
      title: 'AI workflow automation checklist for lean teams',
      date: 'February 26, 2026',
      excerpt:
        'A practical rollout checklist for trigger design, quality gates, and weekly delivery metrics across channels.',
      link: '/blog/ai-workflow-automation-checklist/'
    },
    {
      tag: 'SEO Guide',
      title: 'GitHub issue automation best practices',
      date: 'February 26, 2026',
      excerpt: 'A repeatable issue-to-PR model with better scoping, validation, and reviewer-ready handoffs.',
      link: '/blog/github-issue-automation-best-practices/'
    },
    {
      tag: 'SEO Guide',
      title: 'Email task automation playbook for operations teams',
      date: 'February 26, 2026',
      excerpt: 'How to convert inbound email threads into structured execution, progress updates, and complete deliverables.',
      link: '/blog/email-task-automation-playbook/'
    },
    {
      tag: 'SEO Guide',
      title: 'AI employee trust, safety, and governance framework',
      date: 'February 26, 2026',
      excerpt: 'Governance essentials for permission scopes, audit trails, and escalation paths for high-confidence execution.',
      link: '/blog/ai-employee-trust-safety-governance/'
    }
  ];

  const faqItems = [
    {
      question: 'What is DoWhiz?',
      answer: 'DoWhiz is a startup workspace operating system where a digital founding team executes tasks across your tools and channels with shared memory.'
    },
    {
      question: 'How do I get started?',
      answer: 'Start with founder intake to create your workspace blueprint, then assign work to agents through email, Slack/Discord, GitHub, or shared docs.'
    },
    {
      question: 'Do the employees remember context?',
      answer: 'Yes. Shared memory keeps key preferences and project context so follow-ups are faster and more consistent. You can always update or reset it.'
    },
    {
      question: 'Do I need to share my credentials?',
      answer: 'No. DoWhiz agents use agent-owned identities. They only access workspaces and integrations you explicitly authorize.'
    },
    {
      question: 'What kinds of tasks can the employees handle?',
      answer: 'Writing, project updates, summaries, research, and tool-native deliverables in docs, spreadsheets, slides, and code, tailored by role.'
    },
    {
      question: 'Where can I reach the team?',
      answer: 'Across email, Slack, Discord, GitHub issues, and shared workspace comments. These channels are execution surfaces while workspace remains your operating home.'
    }
  ];

  const [openFaq, setOpenFaq] = useState(null);
  const toggleFaq = (idx) => setOpenFaq((prev) => (prev === idx ? null : idx));

  useEffect(() => {
    const syncHowHeights = () => {
      const roleCards = Array.from(document.querySelectorAll('.how-column .role-card-variant'));
      const outputCards = Array.from(document.querySelectorAll('.how-column .how-card.output'));

      roleCards.forEach((el) => (el.style.minHeight = ''));
      outputCards.forEach((el) => (el.style.minHeight = ''));

      if (roleCards.length) {
        const maxRole = Math.max(...roleCards.map((el) => el.offsetHeight));
        roleCards.forEach((el) => (el.style.minHeight = `${maxRole}px`));
      }
      if (outputCards.length) {
        const maxOut = Math.max(...outputCards.map((el) => el.offsetHeight));
        outputCards.forEach((el) => (el.style.minHeight = `${maxOut}px`));
      }
    };

    syncHowHeights();
    window.addEventListener('resize', syncHowHeights);
    return () => window.removeEventListener('resize', syncHowHeights);
  }, []);

  const structuredData = {
    '@context': 'https://schema.org',
    '@graph': [
      {
        '@type': 'Organization',
        '@id': `${SITE_URL}/#organization`,
        name: ORG_NAME,
        url: `${SITE_URL}/`,
        logo: LOGO_URL,
        email: `mailto:${SUPPORT_EMAIL}`,
        contactPoint: [
          {
            '@type': 'ContactPoint',
            email: SUPPORT_EMAIL,
            contactType: 'customer support',
            availableLanguage: ['English']
          }
        ],
        sameAs: ['https://github.com/KnoWhiz/DoWhiz']
      },
      {
        '@type': 'FAQPage',
        '@id': `${SITE_URL}/#faq`,
        mainEntity: faqItems.map((item) => ({
          '@type': 'Question',
          name: item.question,
          acceptedAnswer: {
            '@type': 'Answer',
            text: item.answer
          }
        }))
      }
    ]
  };

  const teamMembers = [
    {
      name: 'Oliver',
      email: 'oliver@dowhiz.com',
      pronoun: 'He/Him',
      nickname: 'Little-Bear',
      title: 'Generalist',
      desc: 'All-around work assistant for daily office tasks across Notion, Google Docs, Google Slides, and Google Sheets.',
      example: 'Draft a project update in Notion and summarize it for stakeholders.',
      status: 'Active',
      img: oliverImg,
      imgAlt: 'Illustration of Oliver the Little-Bear, DoWhiz generalist digital employee.',
      subject: 'Office Task Request',
      body: 'Draft a project update in Notion and summarize it for stakeholders.',
      profilePath: '/agents/oliver/'
    },
    {
      name: 'Maggie',
      email: 'maggie@dowhiz.com',
      pronoun: 'She/Her',
      nickname: 'Mini-Mouse',
      title: 'TPM',
      desc: 'TPM who turns meeting notes into action items, follows up with people and agents at milestones, updates the board, and sends daily reports.',
      example: "Summarize today's meeting, update action items, and send a daily report.",
      status: 'Coming Soon',
      img: miniMouseImg,
      imgAlt: 'Illustration of Maggie the Mini-Mouse, DoWhiz TPM digital employee.',
      subject: 'TPM Request',
      body: "Summarize today's meeting, turn notes into action items, update the board, and send a daily report.",
      profilePath: '/agents/maggie/'
    },
    {
      name: 'Devin',
      email: 'devin@dowhiz.com',
      pronoun: 'He/Him',
      nickname: 'Sticky-Octopus',
      title: 'Coder',
      desc: 'Coder handling daily development tasks and feature delivery.',
      example: 'Implement the requested feature and open a PR.',
      status: 'Coming Soon',
      img: stickyOctopusImg,
      imgAlt: 'Illustration of Devin the Sticky-Octopus, DoWhiz coder digital employee.',
      subject: 'Coding Task',
      body: 'Implement the requested feature and open a PR.',
      profilePath: '/agents/devin/'
    },
    {
      name: 'Lumio',
      email: 'lumio@dowhiz.com',
      pronoun: 'He/Him',
      nickname: 'Sky-Dragon',
      title: 'CEO',
      desc: 'CEO focused on strategy, leadership, and decision-making.',
      example: 'Draft a one-page strategy for Q2 goals.',
      status: 'Coming Soon',
      img: skyDragonImg,
      imgAlt: 'Illustration of Lumio the Sky-Dragon, DoWhiz CEO digital employee.',
      subject: 'Strategy Request',
      body: 'Draft a one-page strategy for Q2 goals.',
      profilePath: '/agents/lumio/'
    },
    {
      name: 'Claw',
      email: 'claw@dowhiz.com',
      pronoun: 'She/Her',
      nickname: 'Cozy-Lobster',
      title: 'Workflow Specialist',
      desc: 'Workflow specialist focused on safe and accessible orchestration across chat, docs, and engineering tools.',
      example: 'Route Slack triage into GitHub tasks and send weekly execution digests.',
      status: 'Coming Soon',
      img: cozyLobsterImg,
      imgAlt: 'Illustration of Claw the Cozy-Lobster, DoWhiz workflow specialist.',
      subject: 'Assistant Request',
      body: 'Design a safe, tool-native workflow for this cross-channel request.',
      profilePath: '/agents/claw/'
    },
    {
      name: 'Jeffery',
      email: 'jeffery@dowhiz.com',
      pronoun: 'He/Him',
      nickname: 'Strutton-Pigeon',
      title: 'DeepTutor',
      desc: 'DeepTutor helps you understand and manage documents and papers.',
      example: 'Summarize this paper and extract key takeaways.',
      status: 'Coming Soon',
      img: struttonPigeonImg,
      imgAlt: 'Illustration of Jeffery the Strutton-Pigeon, DoWhiz DeepTutor document helper.',
      subject: 'Document Help',
      body: 'Summarize this paper and extract key takeaways.',
      profilePath: '/agents/jeffery/'
    },
    {
      name: 'Anna',
      email: 'anna@dowhiz.com',
      pronoun: 'She/Her',
      nickname: 'Fluffy-Elephant',
      title: 'TBD',
      desc: 'Role definition in progress.',
      example: 'TBD.',
      status: 'Coming Soon',
      img: fluffyElephantImg,
      imgAlt: 'Illustration of Anna the Fluffy-Elephant, DoWhiz role in progress.',
      subject: 'Role Request',
      body: 'Role definition in progress.',
      profilePath: '/agents/anna/'
    },
    {
      name: 'Rachel',
      email: 'rachel@dowhiz.com',
      pronoun: 'She/Her',
      nickname: 'Plush-Axolotl',
      title: 'GTM Specialist',
      desc: 'GTM specialist tracking team status and product progress, publishing posts to LinkedIn, Xiaohongshu, Reddit, YouTube, X, Medium, Product Hunt, Hacker News, and WeChat groups.',
      example: "Prepare and schedule this week's multi-platform launch posts.",
      status: 'Coming Soon',
      img: plushAxolotlImg,
      imgAlt: 'Illustration of Rachel the Plush-Axolotl, DoWhiz GTM specialist.',
      subject: 'GTM Request',
      body: 'Prepare posts across LinkedIn, Xiaohongshu, Reddit, YouTube, X, Medium, Product Hunt, Hacker News, and WeChat groups.',
      profilePath: '/agents/rachel/'
    }
  ];

  const heroPrimaryCtaHref = '/start';
  const heroSecondaryCtaHref = '/workspace';

  const handleHeroCtaClick = () => {
    trackAnalyticsEvent('secondary_cta_click', {
      cta_location: 'hero_primary',
      cta_text: 'Start workspace',
      cta_text_legacy: 'Try DoWhiz service today'
    });
  };

  const handleSignupCtaClick = () => {
    trackAnalyticsEvent('primary_cta_click', {
      cta_location: 'nav_sign_in',
      cta_text: 'Sign In'
    });
  };

  return (
    <div className="app-container">
      <script
        type="application/ld+json"
        dangerouslySetInnerHTML={{ __html: JSON.stringify(structuredData) }}
      />
      <div className="content-layer">
        {/* Navigation */}
        <nav className={`navbar${navHidden ? ' nav-hidden' : ''}`}>
          <div className="nav-content">
            <a href="/" className="logo">
              <img src="/assets/DoWhiz.svg" alt="" className="brand-mark" aria-hidden="true" />
              <span>Do<span className="text-gradient">Whiz</span></span>
            </a>
            <div className="nav-links">
              <a href="#roles" className="nav-btn">Team</a>
              <a href="#how-it-works" className="nav-btn">How it works</a>
              <a href="#workflows" className="nav-btn">Workflows</a>
              <a href="#safety" className="nav-btn">Safety</a>
              <a href="#features" className="nav-btn">Features</a>
              <a href="#deployment" className="nav-btn">Manual setup</a>
              <a href="#faq" className="nav-btn">FAQ</a>
              <a href="#blog" className="nav-btn">Blog</a>
            </div>
            <div className="nav-actions">
              <div className="social-links">
                <a href="https://github.com/KnoWhiz/DoWhiz" target="_blank" rel="noopener noreferrer" className="btn-small" aria-label="GitHub">
                  <svg viewBox="0 0 24 24" width="20" height="20" stroke="currentColor" strokeWidth="2" fill="none" strokeLinecap="round" strokeLinejoin="round">
                    <path d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22"></path>
                  </svg>
                </a>
                <a href="https://discord.gg/7ucnweCKk8" target="_blank" rel="noopener noreferrer" className="btn-small" aria-label="Discord">
                  <svg viewBox="0 0 24 24" width="20" height="20" stroke="currentColor" strokeWidth="2" fill="none" strokeLinecap="round" strokeLinejoin="round">
                    <path d="M21 11.5a8.38 8.38 0 0 1-.9 3.8 8.5 8.5 0 0 1-7.6 4.7 8.38 8.38 0 0 1-3.8-.9L3 21l1.9-5.7a8.38 8.38 0 0 1-.9-3.8 8.5 8.5 0 0 1 4.7-7.6 8.38 8.38 0 0 1 3.8-.9h.5a8.48 8.48 0 0 1 8 8v.5z"></path>
                  </svg>
                </a>
                <a className="btn-small" href="mailto:admin@dowhiz.com" aria-label="Contact">
                  <svg viewBox="0 0 24 24" width="20" height="20" stroke="currentColor" strokeWidth="2" fill="none" strokeLinecap="round" strokeLinejoin="round">
                    <path d="M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z"></path>
                    <polyline points="22,6 12,13 2,6"></polyline>
                  </svg>
                </a>
                {user ? (
                  <div className="user-menu-container" ref={userMenuRef}>
                    <button
                      type="button"
                      className="user-profile-btn"
                      onClick={() => setShowUserMenu(!showUserMenu)}
                      aria-label={user.user_metadata?.full_name || user.email || 'User menu'}
                      title={user.user_metadata?.full_name || user.email || 'User menu'}
                    >
                      <img
                        src={user.user_metadata?.avatar_url || user.user_metadata?.picture}
                        alt={user.user_metadata?.full_name || user.email}
                        className="user-avatar"
                      />
                    </button>
                    {showUserMenu && (
                      <div className="user-dropdown">
                        <a
                          href={getLocalizedAuthPath('?loggedIn=true')}
                          className="dropdown-item"
                          onClick={async (e) => {
                            e.preventDefault();
                            const { data: { session } } = await supabase.auth.getSession();
                            window.location.href = session
                              ? getLocalizedAuthPath('?loggedIn=true')
                              : getLocalizedAuthPath();
                          }}
                        >
                          <svg viewBox="0 0 24 24" width="16" height="16" stroke="currentColor" strokeWidth="2" fill="none">
                            <path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5"/>
                          </svg>
                          Dashboard
                        </a>
                        <button className="dropdown-item" onClick={async () => {
                          await supabase.auth.signOut();
                          setUser(null);
                          setShowUserMenu(false);
                        }}>
                          <svg viewBox="0 0 24 24" width="16" height="16" stroke="currentColor" strokeWidth="2" fill="none">
                            <path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"/>
                            <polyline points="16 17 21 12 16 7"/>
                            <line x1="21" y1="12" x2="9" y2="12"/>
                          </svg>
                          Sign Out
                        </button>
                      </div>
                    )}
                  </div>
                ) : (
                  <a
                    className="btn-small"
                    href={getLocalizedAuthPath()}
                    aria-label="Sign In"
                    onClick={handleSignupCtaClick}
                  >
                    <svg viewBox="0 0 24 24" width="20" height="20" stroke="currentColor" strokeWidth="2" fill="none" strokeLinecap="round" strokeLinejoin="round">
                      <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"></path>
                      <circle cx="12" cy="7" r="4"></circle>
                    </svg>
                  </a>
                )}
              </div>
            </div>
          </div>
        </nav>

        {/* Hero Section */}
        <section className="hero-section">
          {enableMouseField ? <MouseField theme={theme} /> : null}
          <div className="halo-effect"></div>
          <div className="container hero-content">
            <h1 className="hero-title">
              <span className="hero-title-line hero-title-line-primary">Launch your startup workspace in one click.</span>
              <span className="hero-title-line hero-title-line-secondary">Your digital founding team executes from idea to GTM.</span>
            </h1>
            <p className="hero-subtitle hero-subtitle-desktop">
              Start from a founder brief, generate a workspace, and coordinate <a href="#roles" className="role-link">specialized agents</a> across build, docs, research, and GTM. Email, Slack, Discord, GitHub, and Google Workspace stay execution surfaces while your workspace is the product home.
            </p>
            <p className="hero-subtitle hero-subtitle-mobile">
              Create a workspace first, then run your digital founding team across existing channels with shared memory and approvals.
            </p>
            <div className="hero-cta">
              <a
                className="btn btn-primary"
                href={heroPrimaryCtaHref}
                onClick={handleHeroCtaClick}
              >
                Start your workspace
              </a>
              <a className="btn btn-secondary" href={heroSecondaryCtaHref}>
                Preview workspace home
              </a>
            </div>
          </div>
        </section>

        {/* Roles & Scenarios */}
        <section id="roles" className="section roles-section">
          <div className="container">
            <h2 className="section-title">Meet Your Digital Founding Team</h2>
            <div className="roles-grid">
              {teamMembers.map((member) => {
                const isActive = member.status === 'Active';
                const cardClasses = `role-card ${isActive ? 'active-role' : 'coming-soon'}`;

                return (
                  <div
                    key={member.name}
                    className={cardClasses}
                    title={`${member.name}: view channels and trigger examples`}
                  >
                    <span
                      className={`status-badge role-status ${isActive ? 'status-active' : 'status-soon'}`}
                    >
                      {member.status}
                    </span>
                    <div className="role-header">
                      <div className="role-profile">
                        <img
                          src={member.img}
                          alt={member.imgAlt}
                          className="role-avatar"
                          loading="lazy"
                          decoding="async"
                          fetchPriority="low"
                          width="60"
                          height="60"
                        />
                        <div>
                          <div className="role-row role-name-row">
                            <h3>{member.name}</h3>
                            <span className="nickname-tag">{member.nickname}</span>
                          </div>
                          <div className="role-row role-title-row">
                            <span className="role-title-text">{member.title}</span>
                            <span className="pronoun-tag">{member.pronoun}</span>
                          </div>
                          <div className="role-row role-email-row">
                            {isActive ? (
                              <a
                                className="email-tag role-email"
                                href={buildMailtoLink(member.email, member.subject, member.body)}
                                target="_blank"
                                rel="noopener noreferrer"
                                aria-label={`Email ${member.name}`}
                              >
                                {member.email}
                              </a>
                            ) : (
                              <span className="email-tag role-email" aria-disabled="true">
                                {member.email}
                              </span>
                            )}
                          </div>
                        </div>
                      </div>
                    </div>
                    <p className="role-desc">{member.desc}</p>
                    <div className="role-example">
                      <span className="example-label">Example Task</span>
                      <p>"{member.example}"</p>
                    </div>
                    <div className="role-actions">
                      <a
                        href={member.profilePath}
                        className="profile-link"
                      >
                        View profile
                      </a>
                      <span className="email-hint">
                        {isActive ? 'Channels + triggers' : 'Coming soon'}
                      </span>
                    </div>
                  </div>
                );
              })}
            </div>
          </div>
        </section>

        <section id="how-it-works" className="section workflow-section">
          <div className="container">
            <h2 className="section-title">Workspace Operating Flow</h2>
            <p className="section-intro">
              One model from founder intent to approved delivery: intake, execution, and return with persistent shared memory.
            </p>
            <div className="how-columns">
              {howItWorksSteps.map((step) => {
                const icon =
                  step.role.toLowerCase().includes('user') ? '/icons/user.svg' : '/icons/agent.svg';
                return (
                  <div key={step.id} className="how-column">
                    <div className="how-head-cell">
                      <div className="how-head-badge">{step.id}</div>
                      <div className="how-head-title">{step.phase}</div>
                    </div>

                    <div className="how-stack">
                      <div className="how-card role-card-variant">
                        <div className="how-card-heading">
                          <img src={icon} alt={`${step.role} icon`} className="how-card-icon" />
                          <span className="how-card-title">{step.role}</span>
                        </div>
                        <p className="how-card-intro">{step.intro}</p>
                        <ul className="how-card-list">
                          {step.points.map((point) => (
                            <li key={point}>{point}</li>
                          ))}
                        </ul>
                      </div>
                      <div className="how-connector-wrap" aria-hidden="true">
                        <span className="how-connector-line"></span>
                        <span className="how-connector-dot"></span>
                      </div>
                      <div className="how-output-wrap">
                        <span className="how-output-dot" aria-hidden="true"></span>
                        <div className="how-card output">
                          <div className="how-card-heading">
                            <img src="/icons/output.svg" alt="Output icon" className="how-card-icon" />
                            <span className="how-card-title">Output</span>
                          </div>
                          <p className="how-card-intro">{step.output}</p>
                        </div>
                      </div>
                    </div>
                  </div>
                );
              })}
            </div>
          </div>
        </section>

        <section id="workflows" className="section">
          <div className="container">
            <h2 className="section-title">Founder Workflow Examples</h2>
            <p className="section-intro">
              Concrete, trigger-to-outcome examples across engineering, planning, and GTM.
            </p>
            <div className="workflow-showcase-grid">
              {workflowExamples.map((workflow) => (
                <article key={workflow.id} className="workflow-showcase-card">
                  <div className="workflow-media-frame">
                    {workflow.mediaType === 'video' ? (
                      <video
                        className="workflow-media"
                        src={workflow.media}
                        autoPlay
                        muted
                        loop
                        playsInline
                        controls
                      />
                    ) : (
                      <img className="workflow-media" src={workflow.media} alt={workflow.title} />
                    )}
                  </div>
                  <div className="workflow-body">
                    <div className="workflow-title-row">
                      <img className="workflow-avatar" src={workflow.avatar} alt={`${workflow.owner} avatar`} />
                      <h3>{workflow.title}</h3>
                    </div>
                    <div className="workflow-block">
                      <span className="workflow-label">Trigger</span>
                      <p>{workflow.trigger}</p>
                    </div>
                    <div className="workflow-block">
                      <span className="workflow-label">Execution</span>
                      <ul className="workflow-execution-list">
                        {workflow.execution.map((item) => (
                          <li key={item}>{item}</li>
                        ))}
                      </ul>
                    </div>
                    <div className="workflow-block">
                      <span className="workflow-label">Result</span>
                      <p>{workflow.result}</p>
                    </div>
                  </div>
                </article>
              ))}
            </div>
          </div>
        </section>

        <section id="safety" className="section">
          <div className="container">
            <h2 className="section-title">Safety &amp; Access</h2>
            <p className="section-intro">
              Built for practical operations with explicit permissions and controlled execution.
            </p>
            <div className="safety-access-layout">
                {safetyItems.map((item) => (
                  <article key={item.tag} className="safety-card">
                    <div className="safety-card-iconwrap">
                      <img src={item.icon} alt={item.tag} className="safety-card-icon" />
                    </div>
                    <h3>{item.title}</h3>
                    <p>{item.desc}</p>
                    <ul className="safety-point-list">
                      {item.points.map((point) => (
                        <li key={point}>{point}</li>
                    ))}
                  </ul>
                </article>
              ))}

              <aside className="access-playbook">
                <h3>How access works</h3>
                <p>
                  You stay in control of where each agent can operate. Access is granted, scoped, and revocable per workspace.
                </p>
                <div className="access-playbook-steps">
                  {accessFlowSteps.map((step, index) => (
                    <div key={step.title} className="access-step-item">
                      <span className="access-step-index">{index + 1}</span>
                      <div>
                        <h4>{step.title}</h4>
                        <p>{step.desc}</p>
                      </div>
                    </div>
                  ))}
                </div>
                <a href="/trust-safety/" className="access-playbook-link">
                  Explore Trust &amp; Safety
                </a>
              </aside>
            </div>
          </div>
        </section>

        {/* Features */}
        <section id="features" className="section features-section">
          <div className="container">
            <h2 className="section-title">Startup Workspace System</h2>
            <p className="section-intro">
              Built for founder execution: channel-native triggers, scoped permissions, shared memory, reviewable artifacts, and explicit human approvals.
            </p>
            <div className="features-grid">
              {features.map((feature) => (
                <div key={feature.tag} className="feature-card">
                  <div className="feature-iconwrap">
                    <img src={feature.icon} alt={feature.title} className="feature-icon" />
                  </div>
                  <h3>{feature.title}</h3>
                  <p>{feature.desc}</p>
                </div>
              ))}
            </div>
          </div>
        </section>

        <section id="deployment" className="section deployment-section">
          <div className="container">
            <h2 className="section-title">Manual Runtime Deployment (Optional)</h2>
            <p className="section-intro">
              This is a secondary path for teams that need DoWhiz-managed runtime setup beyond standard workspace onboarding.
            </p>
            <div className="deployment-grid">
              {deploymentSteps.map((step) => (
                <article key={step.id} className="deployment-card">
                  <span className="deployment-step-tag">{step.id}</span>
                  <h3>{step.title}</h3>
                  <p>{step.desc}</p>
                </article>
              ))}
            </div>
            <article className="deployment-cta-card">
              <h3>Before you submit manual setup</h3>
              <ul className="deployment-point-list">
                {deploymentChecklist.map((item) => (
                  <li key={item}>{item}</li>
                ))}
              </ul>
              <div className="deployment-actions">
                <a className="btn btn-secondary" href="#deployment-intake">
                  Open manual deployment intake
                </a>
                <a className="btn btn-secondary" href="/trust-safety/">
                  Review trust &amp; safety
                </a>
              </div>
            </article>
          </div>
        </section>

        <section id="deployment-intake" className="section deployment-intake-section">
          <div className="container">
            <h2 className="section-title">Manual Deployment Intake (Secondary Path)</h2>
            <p className="section-intro">
              Submit manual runtime setup details. Intake starts after validation.
            </p>

            <form className="deployment-intake-form" onSubmit={handleDeploymentSubmit} noValidate>
              <article className="deployment-form-card">
                <h3>Operator Profile</h3>
                <p>Tell us who owns this manual runtime deployment request.</p>
                <div className="deployment-field-grid">
                  <label className="deployment-field">
                    <span>Full name</span>
                    <input
                      type="text"
                      name="full_name"
                      autoComplete="name"
                      value={deploymentForm.full_name}
                      onChange={handleDeploymentFieldChange('full_name')}
                      required
                    />
                  </label>

                  <label className="deployment-field">
                    <span>Work email</span>
                    <input
                      type="email"
                      name="work_email"
                      autoComplete="email"
                      value={deploymentForm.work_email}
                      onChange={handleDeploymentFieldChange('work_email')}
                      required
                    />
                  </label>

                  <label className="deployment-field">
                    <span>Agent piecework hourly rate (USD/hour)</span>
                    <input
                      type="number"
                      name="agent_hourly_rate_usd"
                      min="0"
                      step="0.01"
                      placeholder="e.g. 120"
                      value={deploymentForm.agent_hourly_rate_usd}
                      onChange={handleDeploymentFieldChange('agent_hourly_rate_usd')}
                    />
                  </label>

                  <label className="deployment-field">
                    <span>Company or team</span>
                    <input
                      type="text"
                      name="team_name"
                      value={deploymentForm.team_name}
                      onChange={handleDeploymentFieldChange('team_name')}
                    />
                  </label>

                  <label className="deployment-field">
                    <span>Preferred Azure region</span>
                    <select
                      name="azure_region"
                      value={deploymentForm.azure_region}
                      onChange={handleDeploymentFieldChange('azure_region')}
                      required
                    >
                      <option value="">Select a region</option>
                      {DEPLOYMENT_REGIONS.map((region) => (
                        <option key={region} value={region}>{region}</option>
                      ))}
                    </select>
                  </label>

                  <label className="deployment-field">
                    <span>Agent name</span>
                    <input
                      type="text"
                      name="agent_name"
                      value={deploymentForm.agent_name}
                      onChange={handleDeploymentFieldChange('agent_name')}
                      required
                    />
                  </label>

                  <label className="deployment-field">
                    <span>x402 wallet (optional)</span>
                    <input
                      type="text"
                      name="wallet_id"
                      value={deploymentForm.wallet_id}
                      onChange={handleDeploymentFieldChange('wallet_id')}
                    />
                  </label>

                  <label className="deployment-field">
                    <span>Primary contact channel</span>
                    <input
                      type="text"
                      name="contact_channel"
                      placeholder="Email / Telegram / Slack / Discord"
                      value={deploymentForm.contact_channel}
                      onChange={handleDeploymentFieldChange('contact_channel')}
                      required
                    />
                  </label>

                  <label className="deployment-field deployment-field-full">
                    <span>Primary tasks this agent should run</span>
                    <textarea
                      name="use_case"
                      placeholder="Describe target users, task types, and expected outputs."
                      value={deploymentForm.use_case}
                      onChange={handleDeploymentFieldChange('use_case')}
                      required
                    />
                  </label>
                </div>
              </article>

              <article className="deployment-form-card">
                <h3>Uploads</h3>
                <p>Select skills and private data folders required for manual runtime setup.</p>
                <div className="deployment-upload-grid">
                  <div className="deployment-upload-box">
                    <h4>Skills files</h4>
                    <p>Upload skill files (e.g. .md, .json, .yaml, .zip).</p>
                    <input
                      ref={skillsInputRef}
                      type="file"
                      name="skills_files"
                      multiple
                      onChange={handleSkillsFilesChange}
                      required
                    />
                    <div className="deployment-file-meta">
                      {skillsFiles.length
                        ? `${skillsFiles.length} file(s), total ${formatBytes(
                          skillsFiles.reduce((sum, file) => sum + file.size, 0)
                        )}`
                        : 'No files selected.'}
                    </div>
                    <div className="deployment-file-list">
                      {skillsFiles.length ? (
                        skillsFiles.map((file, index) => (
                          <div key={`${file.name}-${file.size}-${index}`} className="deployment-file-item">
                            {(file.webkitRelativePath || file.name)} ({formatBytes(file.size)})
                          </div>
                        ))
                      ) : (
                        <span className="deployment-file-empty">No files selected.</span>
                      )}
                    </div>
                  </div>

                  <div className="deployment-upload-box">
                    <h4>Private data folders</h4>
                    <p>Choose one or more folders. Relative paths are retained for intake.</p>
                    <input
                      ref={privateInputRef}
                      type="file"
                      name="private_data_files"
                      webkitdirectory=""
                      directory=""
                      multiple
                      onChange={handlePrivateFilesChange}
                      required
                    />
                    <div className="deployment-file-meta">
                      {privateFiles.length
                        ? `${privateFiles.length} file(s), total ${formatBytes(
                          privateFiles.reduce((sum, file) => sum + file.size, 0)
                        )}`
                        : 'No files selected.'}
                    </div>
                    <div className="deployment-file-list">
                      {privateFiles.length ? (
                        privateFiles.map((file, index) => (
                          <div key={`${file.name}-${file.size}-${file.lastModified}-${index}`} className="deployment-file-item">
                            {(file.webkitRelativePath || file.name)} ({formatBytes(file.size)})
                          </div>
                        ))
                      ) : (
                        <span className="deployment-file-empty">No files selected.</span>
                      )}
                    </div>
                  </div>
                </div>
                <p className="deployment-upload-total">
                  Current total upload size: {formatBytes(
                    [...skillsFiles, ...privateFiles].reduce((sum, file) => sum + file.size, 0)
                  )} / {formatBytes(MAX_TOTAL_UPLOAD_BYTES)}
                </p>
              </article>

              <article className="deployment-form-card">
                <div className="deployment-env-header">
                  <div>
                    <h3>Environment Keys</h3>
                    <p>Add env key name + value pairs for the requested manual runtime setup.</p>
                  </div>
                  <button type="button" className="btn btn-secondary deployment-mini-btn" onClick={addEnvRow}>
                    Add env key
                  </button>
                </div>
                <div className="deployment-env-list">
                  {envRows.map((row, index) => (
                    <div key={`env-row-${index}`} className="deployment-env-row">
                      <input
                        type="text"
                        placeholder="KEY_NAME"
                        value={row.key}
                        onChange={(event) => updateEnvRow(index, 'key', event.target.value)}
                      />
                      <input
                        type={row.show ? 'text' : 'password'}
                        placeholder="Secret value"
                        value={row.value}
                        onChange={(event) => updateEnvRow(index, 'value', event.target.value)}
                      />
                      <button
                        type="button"
                        className="btn btn-secondary deployment-mini-btn"
                        onClick={() => toggleEnvVisibility(index)}
                      >
                        {row.show ? 'Hide' : 'Show'}
                      </button>
                      <button
                        type="button"
                        className="btn btn-secondary deployment-mini-btn"
                        onClick={() => removeEnvRow(index)}
                      >
                        Remove
                      </button>
                    </div>
                  ))}
                </div>
                <p className="deployment-env-help">
                  Format rule: key must match <code>[A-Z][A-Z0-9_]*</code>. Example: <code>OPENAI_API_KEY</code>.
                </p>
              </article>

              <article className="deployment-form-card">
                <h3>Review &amp; Send</h3>
                <p>Submitting starts manual deployment intake after validation.</p>
                <div className="deployment-checklist">
                  <label className="deployment-check">
                    <input
                      type="checkbox"
                      checked={confirmOwnership}
                      onChange={(event) => setConfirmOwnership(event.target.checked)}
                      required
                    />
                    <span>I confirm I am allowed to share these files and secrets for manual runtime setup.</span>
                  </label>
                  <label className="deployment-check">
                    <input
                      type="checkbox"
                      checked={confirmMvp}
                      onChange={(event) => setConfirmMvp(event.target.checked)}
                      required
                    />
                    <span>I understand provisioning may take up to 24 hours after submission.</span>
                  </label>
                </div>

                <div className="deployment-submit-row">
                  <button type="submit" className="btn btn-primary" disabled={isSubmittingDeployment}>
                    {isSubmittingDeployment ? 'Submitting...' : 'Start Manual Runtime Deployment'}
                  </button>
                  <button type="button" className="btn btn-secondary" onClick={resetDeploymentForm}>
                    Reset form
                  </button>
                </div>

                {deploymentStatus.message ? (
                  <p className={`deployment-status deployment-status-${deploymentStatus.type || 'info'}`}>
                    {deploymentStatus.message}
                  </p>
                ) : null}
              </article>
            </form>
          </div>
        </section>

        {/* FAQ */}
        <section id="faq" className="section faq-section">
          <div className="container">
            <h2 className="section-title">Frequently Asked Questions</h2>
            <p className="section-intro">
              Quick answers to the most common questions about the DoWhiz digital employee team.
            </p>
            <div className="faq-accordion">
              {faqItems.map((item, idx) => {
                const isOpen = openFaq === idx;
                return (
                  <article key={item.question} className={`faq-accordion-item ${isOpen ? 'open' : ''}`}>
                    <button
                      className="faq-accordion-header"
                      onClick={() => toggleFaq(idx)}
                      aria-expanded={isOpen}
                      aria-controls={`faq-panel-${idx}`}
                    >
                      <span className="faq-question">{item.question}</span>
                      <span className="faq-toggle" aria-hidden="true">
                        {isOpen ? '−' : '+'}
                      </span>
                    </button>
                    <div
                      id={`faq-panel-${idx}`}
                      className="faq-accordion-panel"
                      style={{ display: isOpen ? 'block' : 'none' }}
                    >
                      <p>{item.answer}</p>
                    </div>
                  </article>
                );
              })}
            </div>
            <div className="faq-cta">
              <a className="btn btn-secondary" href="https://www.dowhiz.com/help-center/">
                View the full Help Center with top 20 questions
              </a>
            </div>
          </div>
        </section>

        {/* Blog */}
        <section id="blog" className="section blog-section">
          <div className="container">
            <div className="blog-header">
              <div>
                <span className="blog-eyebrow">From the blog</span>
                <h2 className="blog-title">Stories from the workflow graph</h2>
                <p className="blog-intro">
                  Notes on building multi-channel digital employees, shipping integrations, and improving handoffs.
                </p>
              </div>
              <a className="btn btn-secondary blog-header-btn" href="/blog/">View all posts</a>
            </div>
            <div className="blog-grid">
              {blogPosts.map((post) => (
                <article
                  key={post.title}
                  className="blog-card"
                  role="article"
                >
                  <div className="blog-meta">
                    <span className="blog-tag">{post.tag}</span>
                    <span className="blog-date">{post.date}</span>
                  </div>
                  <h3>{post.title}</h3>
                  <p>{post.excerpt}</p>
                  <a className="blog-link" href={post.link}>
                    Read on the blog
                    <span aria-hidden="true" className="blog-link-icon"></span>
                  </a>
                </article>
              ))}
            </div>
          </div>
        </section>

        {/* Footer */}
        <footer className="site-footer">
          <div className="container footer-content">
            <div className="footer-brand">
              <a href="#" className="footer-logo">
                <img src="/assets/DoWhiz.svg" alt="" className="footer-brand-mark" aria-hidden="true" />
                <span>Do<span className="text-gradient">Whiz</span></span>
              </a>
              <p className="footer-tagline">
                Tool-native digital employees that turn messages into finished work with shared memory.
              </p>
              <div className="footer-pill">Multi-channel triggers. Agent-owned identities. Shared memory built-in.</div>
            </div>
            <div className="footer-links">
              <span className="footer-title">Essentials</span>
              <div className="footer-link-grid">
                <a href="/privacy/" className="footer-link">Privacy</a>
                <a href="/terms/" className="footer-link">Terms of Service</a>
                <a href="/trust-safety/" className="footer-link">Trust &amp; Safety</a>
                <a href="/integrations/" className="footer-link">Integrations</a>
                <a href="/user-guide/" className="footer-link">User Guide</a>
                <a href="https://www.dowhiz.com/help-center/" className="footer-link">Help Center</a>
                <a href="/solutions/ai-workflow-automation/" className="footer-link">AI Workflow Automation</a>
                <a href="/solutions/github-issue-automation/" className="footer-link">GitHub Issue Automation</a>
                <a href="/solutions/slack-task-automation/" className="footer-link">Slack Task Automation</a>
                <a href="/solutions/email-task-automation/" className="footer-link">Email Task Automation</a>
                <a href="/solutions/google-docs-automation/" className="footer-link">Google Docs Automation</a>
                <a href="mailto:admin@dowhiz.com" className="footer-link">Contact</a>
              </div>
            </div>
          </div>
          <div className="container footer-bottom">
            <span>&copy; {new Date().getFullYear()} DoWhiz. All rights reserved.</span>
            <span>Built for teams that move across channels.</span>
          </div>
        </footer>

      </div>
    </div>
  );
}

export default LandingPage;

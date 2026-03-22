import { DEFAULT_INTAKE_CONVERSATION_COPY } from '../domain/startupIntake';

const EN_LANDING_CONTENT = {
  metadata: {
    title: 'DoWhiz | Multi-Channel Tool-Native Digital Employees',
    description:
      'Multi-channel, tool-native digital employee team for daily operations. Trigger work from email, chat, GitHub, and shared docs with shared memory across platforms.',
    canonicalUrl: 'https://dowhiz.com/',
    ogLocale: 'en_US',
    themeColor: '#2C2C2E',
    htmlLang: 'en'
  },
  nav: {
    homePath: '/',
    links: [
      { href: '#roles', label: 'Team' },
      { href: '#how-it-works', label: 'How it works' },
      { href: '#workflows', label: 'Workflows' },
      { href: '#safety', label: 'Safety' },
      { href: '#features', label: 'Features' },
      { href: '#faq', label: 'FAQ' },
      { href: '#blog', label: 'Blog' }
    ],
    signIn: 'Sign In',
    dashboard: 'Dashboard',
    signOut: 'Sign Out',
    githubAriaLabel: 'GitHub',
    discordAriaLabel: 'Discord',
    contactAriaLabel: 'Contact Oliver'
  },
  hero: {
    eyebrow: '',
    title: 'One-click setup of a one-person company',
    subtitle: 'Tell us your goals, and we build the right team for you.',
    note: '',
    chips: [],
    pillars: [],
    intakeAriaLabel: 'Start your team brief intake conversation',
    intakeKicker: 'Live intake',
    intakeTitle: 'Tell DoWhiz about your project',
    intakeDescription: 'Run the same conversational Team Brief flow directly from the homepage.',
    secondaryCta: 'Try our agent Oliver',
    contactSubject: 'Office Task Request',
    contactBody:
      'Hi Oliver,\n\nI want to learn more about using DoWhiz for the following workflow:\n-\n-\n-\n\nThanks!'
  },
  sections: {
    rolesTitle: 'Meet Your Digital Founding Team',
    rolesIntro: '',
    howTitle: 'Workspace Operating Flow',
    howIntro:
      'One model from founder intent to approved delivery: intake, execution, and return with persistent shared memory.',
    workflowsTitle: 'Founder Workflow Examples',
    workflowsIntro: 'Concrete, trigger-to-outcome examples across engineering, planning, and GTM.',
    safetyTitle: 'Safety & Access',
    safetyIntro: 'Built for practical operations with explicit permissions and controlled execution.',
    featuresTitle: 'Startup Workspace System',
    featuresIntro:
      'Built for founder execution: channel-native triggers, scoped permissions, shared memory, reviewable artifacts, and explicit human approvals.',
    faqTitle: 'Frequently Asked Questions',
    faqIntro: 'Quick answers to the most common questions about the DoWhiz digital employee team.'
  },
  labels: {
    exampleTask: 'Example Task',
    viewProfile: 'View profile',
    viewProfileEnglish: 'View profile',
    activeHint: 'Channels + triggers',
    soonHint: 'Coming soon',
    trigger: 'Trigger',
    execution: 'Execution',
    result: 'Result',
    output: 'Output',
    accessPlaybookTitle: 'How access works',
    accessPlaybookDescription:
      'You stay in control of where each agent can operate. Access is granted, scoped, and revocable per workspace.',
    accessPlaybookLink: 'Explore Trust & Safety',
    faqCta: 'View the full Help Center with top 20 questions',
    blogEyebrow: 'From the blog',
    blogTitle: 'Stories from the workflow graph',
    blogIntro:
      'Notes on building multi-channel digital employees, shipping integrations, and improving handoffs.',
    blogHeaderButton: 'View all posts',
    blogLinkLabel: 'Read on the blog',
    footerTitle: 'Essentials',
    footerTagline:
      'Tool-native digital employees that turn messages into finished work with shared memory.',
    footerPill: 'Multi-channel triggers. Agent-owned identities. Shared memory built-in.',
    footerBottomSecondary: 'Built for teams that move across channels.'
  },
  features: [
    {
      tag: '01',
      title: 'Trigger from every surface',
      desc:
        'Start work from email, Slack/Discord messages, GitHub issues, or @mentions in shared Google Docs and Notion comments.',
      icon: '/icons/The%20Digital%20Employee%20Stack/trigger.svg'
    },
    {
      tag: '02',
      title: 'Tool-native execution',
      desc:
        'Agents work directly in your docs, project boards, repos, and chat spaces so outputs land where your team already works.',
      icon: '/icons/The%20Digital%20Employee%20Stack/execute.svg'
    },
    {
      tag: '03',
      title: 'Shared memory across channels',
      desc:
        'Per-user context carries over across email, chat, issues, and comments so follow-ups do not restart from zero.',
      icon: '/icons/The%20Digital%20Employee%20Stack/shared.svg'
    },
    {
      tag: '04',
      title: 'Agent-owned identities',
      desc:
        'Each digital employee has their own account identity. You do not hand over personal credentials to get work done.',
      icon: '/icons/The%20Digital%20Employee%20Stack/agent.svg'
    },
    {
      tag: '05',
      title: 'Permissioned workspace access',
      desc:
        'Agents only access workspaces and integrations when you explicitly grant access and can be revoked at any time.',
      icon: '/icons/The%20Digital%20Employee%20Stack/permission.svg'
    },
    {
      tag: '06',
      title: 'Reviewable artifacts + approvals',
      desc:
        'Each run returns auditable artifacts and explicit human approval points before external delivery or sensitive actions.',
      icon: '/icons/The%20Digital%20Employee%20Stack/collaboration.svg'
    }
  ],
  howItWorksSteps: [
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
  ],
  safetyItems: [
    {
      tag: 'A1',
      title: 'Isolated execution environment',
      icon: '/icons/shield_lock.svg',
      desc:
        'Every request runs in an isolated runtime boundary so tasks stay contained, reviewable, and predictable.',
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
      desc:
        'You do not share personal passwords or account credentials with DoWhiz agents to get work done.',
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
      desc:
        'Agents can only work in workspaces and integrations that you explicitly authorize and can revoke.',
      points: [
        'Granular workspace-level permission model',
        'Access can be revoked at any time',
        'Only authorized resources are in scope'
      ]
    }
  ],
  accessFlowSteps: [
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
  ],
  workflowExamples: [
    {
      id: 'maggie',
      title: 'Meeting Summary and Follow-up Task Assignment',
      owner: 'Maggie',
      avatarKey: 'miniMouse',
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
      avatarKey: 'stickyOctopus',
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
      avatarKey: 'oliver',
      mediaType: 'image',
      media: '/icons/workflow%20example/oliver.png',
      trigger: '@mention Oliver in Discord channel and assign him tasks.',
      execution: [
        'Oliver scans the full conversation history in the channel.',
        'Identifies key decisions, technical conclusions, and shared updates.',
        'Extracts concrete action items with clear ownership and priorities.',
        'Organizes them into a structured, execution-ready checklist.'
      ],
      result:
        'You get a concise recap of what happened and a clear, owner-aligned action plan for the next step.'
    }
  ],
  blogPosts: [
    {
      tag: 'Product Update',
      title: 'Startup workspace for founders: March 2026 product update',
      date: 'March 19, 2026',
      excerpt:
        'The new product focus: Team Brief onboarding, Team Workspace, and AI chief of staff recommendations for founders.',
      link: '/blog/startup-workspace-for-founders-product-update/'
    },
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
      excerpt:
        'How to convert inbound email threads into structured execution, progress updates, and complete deliverables.',
      link: '/blog/email-task-automation-playbook/'
    }
  ],
  faqItems: [
    {
      question: 'What is DoWhiz?',
      answer:
        'DoWhiz is a startup workspace operating system where a digital founding team executes tasks across your tools and channels with shared memory.'
    },
    {
      question: 'How do I get started?',
      answer:
        'Start with founder intake to create your workspace blueprint, then assign work to agents through email, Slack/Discord, GitHub, or shared docs.'
    },
    {
      question: 'Do the employees remember context?',
      answer:
        'Yes. Shared memory keeps key preferences and project context so follow-ups are faster and more consistent. You can always update or reset it.'
    },
    {
      question: 'Do I need to share my credentials?',
      answer:
        'No. DoWhiz agents use agent-owned identities. They only access workspaces and integrations you explicitly authorize.'
    },
    {
      question: 'What kinds of tasks can the employees handle?',
      answer:
        'Writing, project updates, summaries, research, and tool-native deliverables in docs, spreadsheets, slides, and code, tailored by role.'
    },
    {
      question: 'Where can I reach the team?',
      answer:
        'Across email, Slack, Discord, GitHub issues, and shared workspace comments. These channels are execution surfaces while workspace remains your operating home.'
    }
  ],
  footerLinks: [
    { href: '/privacy/', label: 'Privacy' },
    { href: '/terms/', label: 'Terms of Service' },
    { href: '/trust-safety/', label: 'Trust & Safety' },
    { href: '/integrations/', label: 'Integrations' },
    { href: '/user-guide/', label: 'User Guide' },
    { href: 'https://www.dowhiz.com/help-center/', label: 'Help Center' },
    { href: '/solutions/ai-workflow-automation/', label: 'AI Workflow Automation' },
    { href: '/solutions/github-issue-automation/', label: 'GitHub Issue Automation' },
    { href: '/solutions/slack-task-automation/', label: 'Slack Task Automation' },
    { href: '/solutions/email-task-automation/', label: 'Email Task Automation' },
    { href: '/solutions/google-docs-automation/', label: 'Google Docs Automation' }
  ],
  teamMembers: [
    {
      name: 'Oliver',
      email: 'oliver@dowhiz.com',
      pronoun: 'He/Him',
      nickname: 'Little-Bear',
      title: 'Generalist',
      desc:
        'All-around work assistant for daily office tasks across Notion, Google Docs, Google Slides, and Google Sheets.',
      example: 'Draft a project update in Notion and summarize it for stakeholders.',
      status: 'Active',
      statusKey: 'active',
      imageKey: 'oliver',
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
      desc:
        'TPM who turns meeting notes into action items, follows up with people and agents at milestones, updates the board, and sends daily reports.',
      example: "Summarize today's meeting, update action items, and send a daily report.",
      status: 'Coming Soon',
      statusKey: 'soon',
      imageKey: 'miniMouse',
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
      statusKey: 'soon',
      imageKey: 'stickyOctopus',
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
      statusKey: 'soon',
      imageKey: 'skyDragon',
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
      desc:
        'Workflow specialist focused on safe and accessible orchestration across chat, docs, and engineering tools.',
      example: 'Route Slack triage into GitHub tasks and send weekly execution digests.',
      status: 'Coming Soon',
      statusKey: 'soon',
      imageKey: 'cozyLobster',
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
      statusKey: 'soon',
      imageKey: 'struttonPigeon',
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
      statusKey: 'soon',
      imageKey: 'fluffyElephant',
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
      desc:
        'GTM specialist tracking team status and product progress, publishing posts to LinkedIn, Xiaohongshu, Reddit, YouTube, X, Medium, Product Hunt, Hacker News, and WeChat groups.',
      example: "Prepare and schedule this week's multi-platform launch posts.",
      status: 'Coming Soon',
      statusKey: 'soon',
      imageKey: 'plushAxolotl',
      imgAlt: 'Illustration of Rachel the Plush-Axolotl, DoWhiz GTM specialist.',
      subject: 'GTM Request',
      body: 'Prepare posts across LinkedIn, Xiaohongshu, Reddit, YouTube, X, Medium, Product Hunt, Hacker News, and WeChat groups.',
      profilePath: '/agents/rachel/'
    }
  ],
  intakeConversation: DEFAULT_INTAKE_CONVERSATION_COPY
};

const ZH_INTAKE_CONVERSATION_COPY = {
  intakeAriaLabel: '中文版 Team Brief 对话',
  composerPlaceholder: '先说一下项目或问题。（Enter 发送）',
  initialAssistantPrompt: '先说你在做什么、到哪一步、希望我们帮什么。',
  send: '发送',
  thinking: '处理中...',
  conversationApiErrorTitle: '接口错误',
  currentJsonDraftTitle: '当前 JSON 草稿',
  currentJsonDraftDescription: '每轮都会更新。',
  readyToCreateBlueprint: '可以生成蓝图了。',
  missingFieldsStatus: (missingFields) => `还缺：${missingFields.length ? missingFields.join('、') : '更多信息'}`,
  currentJsonDraftSummary: '当前草稿',
  blueprintValidationIssuesTitle: '蓝图校验问题',
  createBlueprintNow: '生成蓝图',
  restartChat: '重新开始',
  blueprintSavedTitle: '蓝图已保存',
  blueprintSavedDescription: '蓝图已保存在本地，会出现在 dashboard 里。',
  viewBlueprintJson: '查看蓝图 JSON',
  describeProjectLabel: '描述你的项目',
  signedInSuccessOpeningWorkspace: '登录成功，正在打开 Team Workspace。',
  intakeJsonUpdatedFallback: '我已更新草稿，请继续补充。',
  modelUnavailable: '暂时连不上 intake 模型，请稍后再试。',
  needMoreContext: '还需要更多信息，请先说一下你的项目。',
  missingFieldsBeforeBlueprint: (missingFields) =>
    `生成前还缺：\n- ${missingFields.length ? missingFields.join('\n- ') : '更多信息'}`,
  validationFailed: (errors) => `校验未通过：\n- ${errors.join('\n- ')}`,
  blueprintSavedDirect: '蓝图已保存，正在打开 Team Workspace。',
  blueprintSavedPopup: '蓝图已保存。请在弹窗里登录或注册。',
  blueprintSavedRedirect: '蓝图已保存。弹窗被拦截，我将跳转到登录页。'
};

const ZH_LANDING_CONTENT = {
  metadata: {
    title: 'DoWhiz 中文 | 一人公司，一键开工',
    description:
      '中文用户可以直接在邮件、群聊、GitHub 和共享文档里调用 DoWhiz，完成写作、研发和跟进工作。',
    canonicalUrl: 'https://dowhiz.com/cn',
    ogLocale: 'zh_CN',
    themeColor: '#2C2C2E',
    htmlLang: 'zh-CN'
  },
  nav: {
    homePath: '/cn',
    links: [
      { href: '#roles', label: '团队' },
      { href: '#how-it-works', label: '流程' },
      { href: '#workflows', label: '案例' },
      { href: '#safety', label: '安全' },
      { href: '#features', label: '能力' },
      { href: '#faq', label: '问答' },
      { href: '#blog', label: '博客' }
    ],
    signIn: '登录',
    dashboard: '工作台',
    signOut: '退出登录',
    githubAriaLabel: 'GitHub',
    discordAriaLabel: 'Discord',
    contactAriaLabel: '联系 Oliver'
  },
  hero: {
    eyebrow: '',
    title: '一人公司，一键开工',
    subtitle: '说清要做什么，DoWhiz 会把团队和流程配好。',
    note: '',
    chips: [],
    pillars: [],
    intakeAriaLabel: '开始中文版 Team Brief 对话',
    intakeKicker: '在线需求采集',
    intakeTitle: '先说你的项目',
    intakeDescription: '用几句话说明目标、阶段和要接入的工具。',
    secondaryCta: '联系 Oliver',
    contactSubject: '咨询 DoWhiz 中文版',
    contactBody:
      '你好 Oliver，\n\n我想了解 DoWhiz 是否适合下面这个场景：\n-\n-\n-\n\n谢谢。'
  },
  sections: {
    rolesTitle: '数字团队',
    rolesIntro: '',
    howTitle: '怎么工作',
    howIntro: '从需求到交付，直接在你现有的工具里完成。',
    workflowsTitle: '案例',
    workflowsIntro: '几个最常见的用法。',
    safetyTitle: '安全与权限',
    safetyIntro: '先授权，再执行；随时可撤销。',
    featuresTitle: '系统能力',
    featuresIntro: '少切换，少重复说，少手工跟进。',
    faqTitle: '常见问题',
    faqIntro: '先回答最常见的几个问题。'
  },
  labels: {
    exampleTask: '示例',
    viewProfile: '查看角色详情',
    viewProfileEnglish: '英文详情',
    activeHint: '可直接联系',
    soonHint: '筹备中',
    trigger: '触发',
    execution: '执行',
    result: '结果',
    output: '输出',
    accessPlaybookTitle: '权限怎么管',
    accessPlaybookDescription: '你决定员工能进哪些 workspace、能用哪些工具。',
    accessPlaybookLink: 'Trust & Safety',
    faqCta: '查看帮助中心',
    blogEyebrow: '博客',
    blogTitle: '最近更新',
    blogIntro: '产品和工作流更新。',
    blogHeaderButton: '全部文章',
    blogLinkLabel: '查看原文',
    footerTitle: '更多',
    footerTagline: '让邮件、群聊、代码库和文档里的任务直接落地。',
    footerPill: '多渠道触发 · 共享记忆',
    footerBottomSecondary: '给跨渠道协作的团队。'
  },
  features: [
    {
      tag: '01',
      title: '多入口触发',
      desc: '邮件、群聊、GitHub issue 和共享文档评论里都能发任务。',
      icon: '/icons/The%20Digital%20Employee%20Stack/trigger.svg'
    },
    {
      tag: '02',
      title: '原地执行',
      desc: '结果直接写回文档、项目板、代码库和频道。',
      icon: '/icons/The%20Digital%20Employee%20Stack/execute.svg'
    },
    {
      tag: '03',
      title: '共享上下文',
      desc: '后续跟进不用每次从头解释。',
      icon: '/icons/The%20Digital%20Employee%20Stack/shared.svg'
    },
    {
      tag: '04',
      title: '员工自有身份',
      desc: '不用把个人账号密码交给系统。',
      icon: '/icons/The%20Digital%20Employee%20Stack/agent.svg'
    },
    {
      tag: '05',
      title: '按范围授权',
      desc: '只在你允许的 workspace 和工具里执行。',
      icon: '/icons/The%20Digital%20Employee%20Stack/permission.svg'
    },
    {
      tag: '06',
      title: '结果可审阅',
      desc: '重要动作前保留确认点，过程也可追踪。',
      icon: '/icons/The%20Digital%20Employee%20Stack/collaboration.svg'
    }
  ],
  howItWorksSteps: [
    {
      id: '01',
      phase: '触发',
      role: '用户',
      intro: '任务可以从这些地方发起：',
      points: [
        '邮件',
        'Slack / Discord',
        'GitHub issue 或共享文档评论'
      ],
      output: '系统先整理需求和交付目标。'
    },
    {
      id: '02',
      phase: '执行',
      role: '数字员工',
      intro: '数字员工在获准的工具里直接工作。',
      points: [
        '不需要交接个人账号密码',
        '权限按 workspace 控制',
        '需要时可多角色协作'
      ],
      output: '结果直接写回文档、代码库或频道。'
    },
    {
      id: '03',
      phase: '交付',
      role: '数字员工',
      intro: '结果回到原渠道，后续继续沿用上下文。',
      points: [
        'PR、文档和行动项回到原线程',
        '后续跟进不用重讲一遍'
      ],
      output: '推进更快，也更少断档。'
    }
  ],
  safetyItems: [
    {
      tag: 'A1',
      title: '隔离式执行环境',
      icon: '/icons/shield_lock.svg',
      desc: '每个请求都在独立环境中运行。',
      points: [
        '任务隔离',
        '日志可追踪'
      ]
    },
    {
      tag: 'A2',
      title: '无需交出个人账号密码',
      icon: '/icons/lock_person.svg',
      desc: '不用把你的个人账号密码交给 DoWhiz。',
      points: [
        '员工用自己的身份工作',
        '减少凭证暴露'
      ]
    },
    {
      tag: 'A3',
      title: '明确授权，随时撤销',
      icon: '/icons/key.svg',
      desc: '只在你授权的 workspace 和工具里执行。',
      points: [
        '范围明确',
        '可随时撤销'
      ]
    }
  ],
  accessFlowSteps: [
    {
      title: '授权',
      desc: '先开放入口。'
    },
    {
      title: '划定范围',
      desc: '说明能访问哪里。'
    },
    {
      title: '执行',
      desc: '只在这个范围里工作。'
    },
    {
      title: '撤销',
      desc: '做完就能收回权限。'
    }
  ],
  workflowExamples: [
    {
      id: 'maggie',
      title: '会议总结与跟进',
      owner: 'Maggie',
      avatarKey: 'miniMouse',
      mediaType: 'video',
      media: '/icons/workflow%20example/maggie.mov',
      trigger: '把会议里的任务交给 Maggie。',
      execution: [
        '提炼决策、行动项和负责人',
        '整理跟进节奏和截止时间',
        '把结果发回团队频道'
      ],
      result: '你会得到带负责人和节奏的计划。'
    },
    {
      id: 'devin',
      title: 'Issue 到 PR',
      owner: 'Devin',
      avatarKey: 'stickyOctopus',
      mediaType: 'video',
      media: '/icons/workflow%20example/devin.mov',
      trigger: '创建一个 GitHub issue，分配给 Devin，并写清验收标准。',
      execution: [
        '先拆出实现步骤',
        '完成修改并发起 PR',
        '把测试结果贴回 issue'
      ],
      result: '同一线程里拿到 PR 和测试结果。'
    },
    {
      id: 'oliver',
      title: 'Discord 讨论整理',
      owner: 'Oliver',
      avatarKey: 'oliver',
      mediaType: 'image',
      media: '/icons/workflow%20example/oliver.png',
      trigger: '在 Discord 频道里 @Oliver，并把任务交给他。',
      execution: [
        '读取频道上下文',
        '提炼决策和行动项',
        '整理成可执行清单'
      ],
      result: '拿到简短回顾和清晰待办。'
    }
  ],
  blogPosts: [
    {
      tag: '产品更新',
      title: '3 月产品更新',
      date: '2026 年 3 月 19 日',
      excerpt: '这次更新主要是 Team Brief、Team Workspace 和推荐能力。',
      link: '/blog/startup-workspace-for-founders-product-update/'
    },
    {
      tag: 'SEO 指南',
      title: 'AI 工作流检查清单',
      date: '2026 年 2 月 26 日',
      excerpt: '关于触发器、质量门槛和交付指标的实操清单。',
      link: '/blog/ai-workflow-automation-checklist/'
    },
    {
      tag: 'SEO 指南',
      title: 'GitHub issue 自动化',
      date: '2026 年 2 月 26 日',
      excerpt: '把 issue 更稳地推进到 PR 的一套做法。',
      link: '/blog/github-issue-automation-best-practices/'
    },
    {
      tag: 'SEO 指南',
      title: '邮件自动化手册',
      date: '2026 年 2 月 26 日',
      excerpt: '把收件箱里的请求转成结构化执行。',
      link: '/blog/email-task-automation-playbook/'
    }
  ],
  faqItems: [
    {
      question: 'DoWhiz 到底是什么？',
      answer: 'DoWhiz 是一套让数字员工在现有工具里执行任务的 workspace。'
    },
    {
      question: '我应该怎么开始？',
      answer: '先做 Team Brief，再从邮件、群聊、GitHub 或共享文档发任务。'
    },
    {
      question: '这些员工会记住上下文吗？',
      answer: '会。共享记忆会保留项目上下文，你也可以随时重置。'
    },
    {
      question: '我需要把自己的账号密码交出去吗？',
      answer: '不需要。只在你授权的 workspace 里工作。'
    },
    {
      question: '他们能处理哪些类型的任务？',
      answer: '写作、跟进、总结、研究、文档、表格、幻灯片和代码都可以。'
    },
    {
      question: '我可以通过哪些渠道联系他们？',
      answer: '邮件、Slack、Discord、GitHub issue 和共享文档评论都可以。'
    }
  ],
  footerLinks: [
    { href: '/privacy/', label: '隐私政策' },
    { href: '/terms/', label: '服务条款' },
    { href: '/trust-safety/', label: 'Trust & Safety' },
    { href: '/integrations/', label: '集成' },
    { href: '/user-guide/', label: '使用指南' },
    { href: 'https://www.dowhiz.com/help-center/', label: '帮助中心' },
    { href: '/solutions/ai-workflow-automation/', label: 'AI 工作流自动化' },
    { href: '/solutions/github-issue-automation/', label: 'GitHub Issue 自动化' },
    { href: '/solutions/slack-task-automation/', label: 'Slack 自动化' },
    { href: '/solutions/email-task-automation/', label: '邮件自动化' },
    { href: '/solutions/google-docs-automation/', label: 'Google Docs 自动化' }
  ],
  teamMembers: [
    {
      name: 'Oliver',
      email: 'oliver@dowhiz.com',
      pronoun: '他',
      nickname: 'Little-Bear',
      title: '通才助理',
      desc: '通用型助手，处理文档、表格、幻灯片和日常同步。',
      example: '整理本周进展，并发给相关人。',
      status: '已上线',
      statusKey: 'active',
      imageKey: 'oliver',
      imgAlt: 'DoWhiz 通才数字员工 Oliver 的插画形象。',
      subject: '通才任务请求',
      body: '请帮我把本周项目进展整理成一页更新，并同步给相关人。',
      profilePath: '/agents/oliver/'
    },
    {
      name: 'Maggie',
      email: 'maggie@dowhiz.com',
      pronoun: '她',
      nickname: 'Mini-Mouse',
      title: 'TPM',
      desc: '把会议变成行动项、跟进和日报。',
      example: '总结会议，更新行动项并发日报。',
      status: '筹备中',
      statusKey: 'soon',
      imageKey: 'miniMouse',
      imgAlt: 'DoWhiz TPM 数字员工 Maggie 的插画形象。',
      subject: 'TPM 任务请求',
      body: '请总结今天的会议，生成行动项，更新项目板，并发送一份日报。',
      profilePath: '/agents/maggie/'
    },
    {
      name: 'Devin',
      email: 'devin@dowhiz.com',
      pronoun: '他',
      nickname: 'Sticky-Octopus',
      title: '工程师',
      desc: '负责研发任务、修 bug 和提 PR。',
      example: '实现功能，补测试并提 PR。',
      status: '筹备中',
      statusKey: 'soon',
      imageKey: 'stickyOctopus',
      imgAlt: 'DoWhiz 工程数字员工 Devin 的插画形象。',
      subject: '研发任务请求',
      body: '请实现这个功能，补上测试，并提交一个可评审的 PR。',
      profilePath: '/agents/devin/'
    },
    {
      name: 'Lumio',
      email: 'lumio@dowhiz.com',
      pronoun: '他',
      nickname: 'Sky-Dragon',
      title: 'CEO',
      desc: '做策略判断、优先级和取舍。',
      example: '写一页 Q2 计划，并给出建议。',
      status: '筹备中',
      statusKey: 'soon',
      imageKey: 'skyDragon',
      imgAlt: 'DoWhiz CEO 数字员工 Lumio 的插画形象。',
      subject: '策略任务请求',
      body: '请写一页 Q2 战略说明，对几个选项做取舍并给出建议。',
      profilePath: '/agents/lumio/'
    },
    {
      name: 'Claw',
      email: 'claw@dowhiz.com',
      pronoun: '她',
      nickname: 'Cozy-Lobster',
      title: '工作流专家',
      desc: '把聊天、文档和工程工具串成清晰工作流。',
      example: '把 Slack 分诊接到 GitHub 任务。',
      status: '筹备中',
      statusKey: 'soon',
      imageKey: 'cozyLobster',
      imgAlt: 'DoWhiz 工作流数字员工 Claw 的插画形象。',
      subject: '工作流任务请求',
      body: '请为这条跨渠道请求设计一个安全、工具原生的工作流。',
      profilePath: '/agents/claw/'
    },
    {
      name: 'Jeffery',
      email: 'jeffery@dowhiz.com',
      pronoun: '他',
      nickname: 'Strutton-Pigeon',
      title: 'DeepTutor',
      desc: '整理文档和论文，提炼重点。',
      example: '总结论文，并列出关键问题。',
      status: '筹备中',
      statusKey: 'soon',
      imageKey: 'struttonPigeon',
      imgAlt: 'DoWhiz 文档与论文助手 Jeffery 的插画形象。',
      subject: '文档任务请求',
      body: '请总结这篇论文，并提炼出关键结论和待确认问题。',
      profilePath: '/agents/jeffery/'
    },
    {
      name: 'Anna',
      email: 'anna@dowhiz.com',
      pronoun: '她',
      nickname: 'Fluffy-Elephant',
      title: '角色待定',
      desc: '这个角色还在定义中。',
      example: '角色定义中。',
      status: '筹备中',
      statusKey: 'soon',
      imageKey: 'fluffyElephant',
      imgAlt: 'DoWhiz 角色设计中的数字员工 Anna 的插画形象。',
      subject: '角色任务请求',
      body: '角色定义中。',
      profilePath: '/agents/anna/'
    },
    {
      name: 'Rachel',
      email: 'rachel@dowhiz.com',
      pronoun: '她',
      nickname: 'Plush-Axolotl',
      title: 'GTM 增长专家',
      desc: '负责发布节奏和多平台内容。',
      example: '准备本周发布文案。',
      status: '筹备中',
      statusKey: 'soon',
      imageKey: 'plushAxolotl',
      imgAlt: 'DoWhiz GTM 数字员工 Rachel 的插画形象。',
      subject: 'GTM 任务请求',
      body: '请准备面向多个平台的发布内容，并排好本周的节奏。',
      profilePath: '/agents/rachel/'
    }
  ],
  intakeConversation: ZH_INTAKE_CONVERSATION_COPY
};

export function getLandingContent(locale = 'en-US') {
  return locale === 'zh-CN' ? ZH_LANDING_CONTENT : EN_LANDING_CONTENT;
}

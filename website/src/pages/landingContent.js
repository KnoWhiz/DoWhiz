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
  composerPlaceholder: '先描述一下你的项目、目标或当前卡点...（Enter 发送，Cmd/Ctrl+Enter 换行）',
  initialAssistantPrompt:
    '先告诉我你正在做什么项目、现在处在什么阶段，以及你希望 DoWhiz 帮你完成什么。我会继续追问，并一起整理成可执行的 workspace blueprint。',
  send: '发送',
  thinking: '思考中...',
  conversationApiErrorTitle: '需求对话接口错误',
  currentJsonDraftTitle: '当前 JSON 草稿',
  currentJsonDraftDescription: '模型会在每一轮对话后更新这份草稿。',
  readyToCreateBlueprint: '已满足生成蓝图的条件。',
  missingFieldsStatus: (missingFields) =>
    `仍缺少字段：${missingFields.length ? missingFields.join('、') : '等待更多信息'}`,
  currentJsonDraftSummary: '当前 JSON 草稿',
  blueprintValidationIssuesTitle: '蓝图校验问题',
  createBlueprintNow: '立即生成蓝图',
  restartChat: '重新开始',
  blueprintSavedTitle: '蓝图已保存',
  blueprintSavedDescription: '你的团队蓝图已经保存在本地，并会显示在 dashboard 的 workspace 区域。',
  viewBlueprintJson: '查看蓝图 JSON',
  describeProjectLabel: '描述你的项目',
  signedInSuccessOpeningWorkspace: '登录成功，正在打开 Team Workspace。',
  intakeJsonUpdatedFallback: '我已经更新了 intake JSON，请继续补充缺失的信息。',
  modelUnavailable: '暂时无法连接到 startup intake 模型，请稍后再试。',
  needMoreContext: '我还需要更多 intake 信息。请先描述一下你的项目。',
  missingFieldsBeforeBlueprint: (missingFields) =>
    `生成蓝图前还需要补齐这些字段：\n- ${missingFields.length ? missingFields.join('\n- ') : '更多细节'}`,
  validationFailed: (errors) => `蓝图校验仍未通过：\n- ${errors.join('\n- ')}`,
  blueprintSavedDirect: '蓝图已保存，正在打开 Team Workspace。',
  blueprintSavedPopup: '蓝图已保存。请在弹出的窗口中登录或注册，成功后会自动关闭。',
  blueprintSavedRedirect:
    '蓝图已保存。由于弹窗被拦截，我将直接跳转到登录页；完成认证后，Team Workspace 会带上你的蓝图。'
};

const ZH_LANDING_CONTENT = {
  metadata: {
    title: 'DoWhiz 中文 | 面向创始人的数字员工工作台',
    description:
      'DoWhiz 为创始人提供可在邮件、群聊、GitHub 和共享文档中直接工作的数字员工团队，把零散需求转成可追踪、可交付、可延续的结果。',
    canonicalUrl: 'https://dowhiz.com/cn',
    ogLocale: 'zh_CN',
    themeColor: '#9A3412',
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
    eyebrow: '面向中文创始人的数字员工工作台',
    title: '把一个人的创业，变成一支能落地的数字团队',
    subtitle:
      '你只需要说清目标、约束和节奏，DoWhiz 就会把团队、工具和交付流程一起拉起来。',
    note:
      '适合高速试错中的创始人：需求可以直接从邮件、群聊、GitHub issue、共享文档评论里发起，结果也会回到你原来工作的地方。',
    chips: ['邮件', 'Slack / Discord', 'GitHub', 'Google Docs / Notion', '共享记忆'],
    pillars: [
      {
        label: '触发',
        value: '在现有沟通面里直接开工',
        desc: '不用切系统，在邮件、评论、群聊里交代一句就能发起任务。'
      },
      {
        label: '执行',
        value: '在工具里原地完成',
        desc: '数字员工直接写文档、提 PR、整理任务板，而不是把结果困在黑盒对话里。'
      },
      {
        label: '交付',
        value: '结果可追踪，后续不断档',
        desc: '共享记忆会保留上下文，让跟进不必从头解释。'
      }
    ],
    intakeAriaLabel: '开始中文版 Team Brief 对话',
    intakeKicker: '在线需求采集',
    intakeTitle: '先把你的项目说清楚',
    intakeDescription:
      '直接在首页完成中文版 Team Brief，对话里把目标、阶段、约束和要接入的工具讲明白。',
    secondaryCta: '先和 Oliver 聊聊',
    contactSubject: '想了解 DoWhiz 的中文使用场景',
    contactBody:
      '你好 Oliver，\n\n我想了解 DoWhiz 是否适合以下场景：\n-\n-\n-\n\n希望你给我一些建议，谢谢。'
  },
  sections: {
    rolesTitle: '认识你的数字创业班底',
    rolesIntro:
      '先把角色分工搭起来，再让任务通过合适的渠道落到合适的数字员工手里。',
    howTitle: 'DoWhiz 的工作方式',
    howIntro:
      '从创始人意图到交付回收，只有一条连续链路：收集需求、在原工具里执行、回到原渠道交付，并把上下文记住。',
    workflowsTitle: '创始人工作流示例',
    workflowsIntro: '看一条需求如何从触发走到结果，而不是停在一个聊天窗口里。',
    safetyTitle: '安全与权限',
    safetyIntro: '不是把账号密码交出去，而是先定义边界，再让数字员工在边界内执行。',
    featuresTitle: '创业团队真正需要的系统能力',
    featuresIntro:
      '为创始人执行而建：原渠道触发、带边界的权限、共享记忆、可审核的产物，以及明确的人类确认点。',
    faqTitle: '常见问题',
    faqIntro: '快速回答大家最常问的 DoWhiz 中文使用问题。'
  },
  labels: {
    exampleTask: '示例任务',
    viewProfile: '查看角色详情',
    viewProfileEnglish: '查看角色详情（英文）',
    activeHint: '渠道 + 触发方式',
    soonHint: '即将开放',
    trigger: '触发',
    execution: '执行',
    result: '结果',
    output: '输出',
    accessPlaybookTitle: '权限是怎么工作的',
    accessPlaybookDescription:
      '你始终掌握每个数字员工能进入哪些工作区、能使用哪些工具。授权、范围和撤销都按 workspace 管理。',
    accessPlaybookLink: '查看 Trust & Safety（英文）',
    faqCta: '查看帮助中心 Top 20 问题（英文）',
    blogEyebrow: '博客更新',
    blogTitle: '关于数字员工工作流的最新记录',
    blogIntro: '我们把多渠道数字员工、集成建设和交付 handoff 的经验持续写出来。',
    blogHeaderButton: '查看全部文章（英文）',
    blogLinkLabel: '阅读英文原文',
    footerTitle: '常用入口',
    footerTagline:
      '在邮件、群聊、代码库和文档里原地工作的数字员工，把消息变成交付，并把上下文延续下去。',
    footerPill: '多渠道触发 · 独立身份 · 共享记忆',
    footerBottomSecondary: '为跨渠道高频协作的团队而建。'
  },
  features: [
    {
      tag: '01',
      title: '从任何工作界面触发',
      desc:
        '邮件、Slack/Discord 消息、GitHub issue，或共享 Google Docs / Notion 评论里的 @mention，都可以直接发起任务。',
      icon: '/icons/The%20Digital%20Employee%20Stack/trigger.svg'
    },
    {
      tag: '02',
      title: '在原工具里直接执行',
      desc:
        '数字员工直接在文档、项目板、代码库和聊天空间里工作，让结果落回团队本来就在用的地方。',
      icon: '/icons/The%20Digital%20Employee%20Stack/execute.svg'
    },
    {
      tag: '03',
      title: '跨渠道共享记忆',
      desc:
        '同一位用户的上下文可以在邮件、群聊、issue 和评论之间延续，后续跟进不需要重新讲一遍。',
      icon: '/icons/The%20Digital%20Employee%20Stack/shared.svg'
    },
    {
      tag: '04',
      title: '员工自有身份',
      desc:
        '每位数字员工都有自己的账号身份。为了完成工作，你不需要把个人账号直接交给系统。',
      icon: '/icons/The%20Digital%20Employee%20Stack/agent.svg'
    },
    {
      tag: '05',
      title: '按工作区授予权限',
      desc:
        '只有在你明确授权之后，数字员工才会进入对应 workspace 或集成，而且随时可以撤销。',
      icon: '/icons/The%20Digital%20Employee%20Stack/permission.svg'
    },
    {
      tag: '06',
      title: '产物可审阅，也可审批',
      desc:
        '每次执行都会留下可审计的工作产物，并在对外发送或敏感动作前保留明确的人类确认点。',
      icon: '/icons/The%20Digital%20Employee%20Stack/collaboration.svg'
    }
  ],
  howItWorksSteps: [
    {
      id: '01',
      phase: '触发',
      role: '用户',
      intro: '你可以这样把任务交给 DoWhiz：',
      points: [
        '发一封带附件、链接和约束的邮件',
        '在 Slack / Discord 线程里发消息或 @mention',
        '在 GitHub issue、共享文档评论中直接点名'
      ],
      output: '系统会整理出包含发起人上下文、预期结果和交付目标的结构化 brief。'
    },
    {
      id: '02',
      phase: '执行',
      role: '数字员工',
      intro: '数字员工会在获准的工具里直接工作，并使用独立身份和受限权限。',
      points: [
        '不需要交接个人账号密码',
        '工作区权限是明确授权、随时可撤销的',
        '多步骤任务可以跨角色协作完成'
      ],
      output: '工作产物会被创建在你的团队本来就在协作的位置。'
    },
    {
      id: '03',
      phase: '交付',
      role: '数字员工',
      intro: '结果会回到原始渠道，而共享记忆会让后续请求保持连续性。',
      points: [
        'PR、文档、行动项和更新可以直接回到原线程',
        '同一位用户的上下文能在多个渠道间延续',
        '后续跟进从历史开始，而不是重新从零解释'
      ],
      output: '你得到的是更快的迭代速度，以及在每个工作面上更稳定的交付质量。'
    }
  ],
  safetyItems: [
    {
      tag: 'A1',
      title: '隔离式执行环境',
      icon: '/icons/shield_lock.svg',
      desc:
        '每一次请求都会在隔离的运行边界中执行，让任务过程更可控、可审阅，也更可预期。',
      points: [
        '每个任务都拥有独立的 sandbox / VM 边界',
        '网络与文件访问会按范围受控',
        '执行日志可供审计与追溯'
      ]
    },
    {
      tag: 'A2',
      title: '无需交出个人账号密码',
      icon: '/icons/lock_person.svg',
      desc: '为了让 DoWhiz 完成工作，你不需要把个人账号密码或长期凭证交给数字员工。',
      points: [
        '数字员工使用自己的账号身份',
        '不需要直接登录你的个人账户',
        '凭证暴露风险在设计上被压低'
      ]
    },
    {
      tag: 'A3',
      title: '明确授权，随时撤销',
      icon: '/icons/key.svg',
      desc: '数字员工只能进入你明确授权的 workspace 与集成，而且可以随时撤销访问。',
      points: [
        '权限粒度可以下沉到具体 workspace',
        '任务结束后随时可以回收授权',
        '只有被允许的资源会进入执行范围'
      ]
    }
  ],
  accessFlowSteps: [
    {
      title: '授权',
      desc: '先在你要使用的工具里邀请或授权数字员工账号。'
    },
    {
      title: '划定范围',
      desc: '明确这次任务允许访问的项目、文档、代码库或频道。'
    },
    {
      title: '执行',
      desc: '数字员工只会在你批准的范围内工作，并把结果回报出来。'
    },
    {
      title: '撤销',
      desc: '任务完成后，你可以随时移除对应 workspace 的权限。'
    }
  ],
  workflowExamples: [
    {
      id: 'maggie',
      title: '会议总结 + 跟进任务分派',
      owner: 'Maggie',
      avatarKey: 'miniMouse',
      mediaType: 'video',
      media: '/icons/workflow%20example/maggie.mov',
      trigger: '把会议里要做的事直接告诉 Maggie。',
      execution: [
        'Maggie 会抽取决策、行动项、依赖关系和负责人。',
        '把后续跟进、截止时间和关键里程碑整理出来。',
        '准备一份适合发回团队频道的状态更新。',
        '需要时再把子任务分派给其他数字员工。'
      ],
      result: '你会得到一份有人负责、带节奏、可继续推进的执行计划。'
    },
    {
      id: 'devin',
      title: '从 GitHub issue 到代码交付',
      owner: 'Devin',
      avatarKey: 'stickyOctopus',
      mediaType: 'video',
      media: '/icons/workflow%20example/devin.mov',
      trigger: '创建一个 GitHub issue，分配给 Devin，并写清验收标准。',
      execution: [
        'Devin 会先把工作拆成可执行的实现检查点。',
        '完成修改后直接发起 pull request。',
        '跑测试，并把通过/失败结论和评审上下文一并贴回去。'
      ],
      result: '你会在同一个 issue 线程里拿到 PR、测试状态和简明总结。'
    },
    {
      id: 'oliver',
      title: '把 Discord 讨论整理成待办清单',
      owner: 'Oliver',
      avatarKey: 'oliver',
      mediaType: 'image',
      media: '/icons/workflow%20example/oliver.png',
      trigger: '在 Discord 频道里 @Oliver，并把任务交给他。',
      execution: [
        'Oliver 会阅读频道中的完整上下文。',
        '识别关键决策、技术结论和需要同步的更新。',
        '提炼出明确负责人和优先级的行动项。',
        '把它们组织成一份可以立即执行的清单。'
      ],
      result: '你会得到一份简明回顾，以及面向下一步的清晰执行计划。'
    }
  ],
  blogPosts: [
    {
      tag: '产品更新',
      title: '创始人工作台：2026 年 3 月产品更新',
      date: '2026 年 3 月 19 日',
      excerpt: '这次更新聚焦 Team Brief onboarding、Team Workspace，以及面向创始人的 AI chief of staff 建议。',
      link: '/blog/startup-workspace-for-founders-product-update/'
    },
    {
      tag: 'SEO 指南',
      title: '精简团队的 AI 工作流自动化检查清单',
      date: '2026 年 2 月 26 日',
      excerpt: '一份关于触发器设计、质量门槛，以及跨渠道每周交付指标的实操清单。',
      link: '/blog/ai-workflow-automation-checklist/'
    },
    {
      tag: 'SEO 指南',
      title: 'GitHub issue 自动化最佳实践',
      date: '2026 年 2 月 26 日',
      excerpt: '一套更清晰地定义范围、完成验证并把结果顺利交给 reviewer 的 issue-to-PR 模型。',
      link: '/blog/github-issue-automation-best-practices/'
    },
    {
      tag: 'SEO 指南',
      title: '运营团队的邮件任务自动化手册',
      date: '2026 年 2 月 26 日',
      excerpt: '如何把收件箱里的邮件线程转成结构化执行、进度回报和完整交付。',
      link: '/blog/email-task-automation-playbook/'
    }
  ],
  faqItems: [
    {
      question: 'DoWhiz 到底是什么？',
      answer:
        'DoWhiz 是一套面向创业团队的 workspace operating system。它让一支数字创业班底在你已有的工具和渠道里协同执行任务，并共享上下文。'
    },
    {
      question: '我应该怎么开始？',
      answer:
        '先通过 founder intake / Team Brief 生成你的 workspace blueprint，然后再通过邮件、Slack/Discord、GitHub 或共享文档把工作交给数字员工。'
    },
    {
      question: '这些员工会记住上下文吗？',
      answer:
        '会。共享记忆会保留关键偏好和项目上下文，让后续跟进更快、更稳定；你也可以随时更新或重置它。'
    },
    {
      question: '我需要把自己的账号密码交出去吗？',
      answer:
        '不需要。DoWhiz 数字员工使用自己的身份工作，只会访问你明确授权的 workspace 与集成。'
    },
    {
      question: '他们能处理哪些类型的任务？',
      answer:
        '从写作、项目更新、总结、研究，到文档、表格、幻灯片和代码中的工具原生交付，都可以按不同角色分工处理。'
    },
    {
      question: '我可以通过哪些渠道联系他们？',
      answer:
        '邮件、Slack、Discord、GitHub issue，以及共享工作区里的评论都可以。这些都是触发与交付的工作面，而 workspace 是整体操作中枢。'
    }
  ],
  footerLinks: [
    { href: '/privacy/', label: '隐私政策（英文）' },
    { href: '/terms/', label: '服务条款（英文）' },
    { href: '/trust-safety/', label: '信任与安全（英文）' },
    { href: '/integrations/', label: '集成能力（英文）' },
    { href: '/user-guide/', label: '使用指南（英文）' },
    { href: 'https://www.dowhiz.com/help-center/', label: '帮助中心（英文）' },
    { href: '/solutions/ai-workflow-automation/', label: 'AI 工作流自动化（英文）' },
    { href: '/solutions/github-issue-automation/', label: 'GitHub Issue 自动化（英文）' },
    { href: '/solutions/slack-task-automation/', label: 'Slack 任务自动化（英文）' },
    { href: '/solutions/email-task-automation/', label: '邮件任务自动化（英文）' },
    { href: '/solutions/google-docs-automation/', label: 'Google Docs 自动化（英文）' }
  ],
  teamMembers: [
    {
      name: 'Oliver',
      email: 'oliver@dowhiz.com',
      pronoun: '他',
      nickname: 'Little-Bear',
      title: '通才助理',
      desc:
        '负责 Notion、Google Docs、Google Slides 和 Google Sheets 等日常办公室工作，是通用型的第一响应角色。',
      example: '帮我把本周项目进展整理成一页更新，并同步给相关人。',
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
      desc:
        '把会议纪要转成行动项，在关键节点跟进人和代理，更新项目板，并发送节奏清晰的日报。',
      example: '总结今天的会议，生成行动项，更新项目板，再发一份日报。',
      status: '即将上线',
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
      desc: '负责日常研发任务、缺陷修复与功能交付，把 issue 推进到可评审的代码变更。',
      example: '实现这个功能，补上测试，并提交一个可评审的 PR。',
      status: '即将上线',
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
      desc: '聚焦策略、优先级和关键取舍，帮助创始人把分散想法收敛成可执行判断。',
      example: '写一页 Q2 战略说明，对几个选项做取舍并给出建议。',
      status: '即将上线',
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
      desc:
        '专注于把聊天、文档和工程工具串成安全、清晰、可维护的工作流，减少重复协调成本。',
      example: '把 Slack 里的分诊流程接到 GitHub 任务，并生成每周执行摘要。',
      status: '即将上线',
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
      desc: '帮助你理解、整理和管理文档与论文，把长材料转成可吸收、可行动的结论。',
      example: '总结这篇论文，并提炼出关键结论和待确认问题。',
      status: '即将上线',
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
      desc: '这个角色还在定义中，未来会补足目前团队里仍然空缺的能力带。',
      example: '角色定义中。',
      status: '即将上线',
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
      desc:
        '跟踪团队状态和产品进展，并把内容分发到 LinkedIn、小红书、Reddit、YouTube、X、Medium、Product Hunt、Hacker News 和微信群。',
      example: '准备并排期本周多平台发布文案。',
      status: '即将上线',
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

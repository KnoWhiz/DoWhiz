import { useState, useEffect, useRef } from 'react';
import oliverImg from './assets/Oliver.jpg';
import miniMouseImg from './assets/Mini-Mouse.jpg';
import stickyOctopusImg from './assets/Sticky-Octopus.jpg';
import skyDragonImg from './assets/Sky-Dragon.jpg';
import cozyLobsterImg from './assets/Cozy-Lobster.jpg';
import struttonPigeonImg from './assets/Strutton-Pigeon.jpg';
import fluffyElephantImg from './assets/Fluffy-Elephant.jpg';
import plushAxolotlImg from './assets/Plush-Axolotl.jpg';
import workflowMessage from './assets/Messages.jpg';
import workflowTriage from './assets/Triage.jpg';
import workflowExecution from './assets/Execution.jpg';
import workflowReturn from './assets/Return.jpg';

const lerp = (start, end, t) => start + (end - start) * t;
const clamp = (value, min, max) => Math.min(max, Math.max(min, value));

const palettes = {
  dark: [
    { r: 56, g: 189, b: 248 },
    { r: 99, g: 102, b: 241 },
    { r: 20, g: 184, b: 166 }
  ],
  light: [
    { r: 14, g: 116, b: 144 },
    { r: 56, g: 189, b: 248 },
    { r: 245, g: 158, b: 11 }
  ]
};

const blendColor = (from, to, t) => ({
  r: Math.round(lerp(from.r, to.r, t)),
  g: Math.round(lerp(from.g, to.g, t)),
  b: Math.round(lerp(from.b, to.b, t))
});

const pickColor = (t, palette) => {
  const scaled = clamp(t, 0, 1);
  if (scaled < 0.5) {
    return blendColor(palette[0], palette[1], scaled * 2);
  }
  return blendColor(palette[1], palette[2], (scaled - 0.5) * 2);
};

const createParticles = (count, width, height) => {
  return Array.from({ length: count }, () => {
    const x = Math.random() * width;
    const y = Math.random() * height;
    return {
      x,
      y,
      baseX: x,
      baseY: y,
      vx: 0,
      vy: 0,
      size: 0.6 + Math.random() * 1.8,
      glow: 6 + Math.random() * 14,
      alpha: 0.2 + Math.random() * 0.6,
      seed: Math.random() * Math.PI * 2,
      drift: 6 + Math.random() * 26
    };
  });
};

function MouseField({ theme }) {
  const canvasRef = useRef(null);
  const particlesRef = useRef([]);
  const pointerRef = useRef({
    x: 0,
    y: 0,
    smoothX: 0,
    smoothY: 0,
    active: false
  });
  const sizeRef = useRef({ width: 0, height: 0, dpr: 1 });
  const themeRef = useRef(theme);
  const reduceMotionRef = useRef(false);
  const rafRef = useRef(0);

  useEffect(() => {
    themeRef.current = theme;
  }, [theme]);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) {
      return;
    }
    const context = canvas.getContext('2d');
    if (!context) {
      return;
    }

    const reduceMotionQuery = window.matchMedia('(prefers-reduced-motion: reduce)');
    reduceMotionRef.current = reduceMotionQuery.matches;

    const handleReduceMotion = (event) => {
      reduceMotionRef.current = event.matches;
    };

    if (reduceMotionQuery.addEventListener) {
      reduceMotionQuery.addEventListener('change', handleReduceMotion);
    } else {
      reduceMotionQuery.addListener(handleReduceMotion);
    }

    const setSize = () => {
      const width = window.innerWidth;
      const height = window.innerHeight;
      const dpr = Math.min(window.devicePixelRatio || 1, 2);
      canvas.width = Math.floor(width * dpr);
      canvas.height = Math.floor(height * dpr);
      canvas.style.width = `${width}px`;
      canvas.style.height = `${height}px`;
      context.setTransform(dpr, 0, 0, dpr, 0, 0);
      sizeRef.current = { width, height, dpr };

      const density = width * height > 800000 ? 12000 : 16000;
      const count = Math.min(180, Math.max(70, Math.floor((width * height) / density)));
      particlesRef.current = createParticles(count, width, height);

      pointerRef.current.x = width / 2;
      pointerRef.current.y = height / 2;
      pointerRef.current.smoothX = width / 2;
      pointerRef.current.smoothY = height / 2;
    };

    setSize();
    window.addEventListener('resize', setSize);

    const handlePointerMove = (event) => {
      pointerRef.current.x = event.clientX;
      pointerRef.current.y = event.clientY;
      pointerRef.current.active = true;
    };

    const handlePointerLeave = () => {
      pointerRef.current.active = false;
    };

    window.addEventListener('pointermove', handlePointerMove, { passive: true });
    window.addEventListener('pointerdown', handlePointerMove, { passive: true });
    window.addEventListener('pointerleave', handlePointerLeave);
    window.addEventListener('blur', handlePointerLeave);

    const drawFrame = (timestamp) => {
      const { width, height } = sizeRef.current;
      if (!width || !height) {
        rafRef.current = requestAnimationFrame(drawFrame);
        return;
      }

      if (reduceMotionRef.current) {
        context.clearRect(0, 0, width, height);
        rafRef.current = requestAnimationFrame(drawFrame);
        return;
      }

      context.clearRect(0, 0, width, height);
      context.globalCompositeOperation = themeRef.current === 'dark' ? 'lighter' : 'source-over';

      const palette = palettes[themeRef.current] || palettes.dark;
      const pointer = pointerRef.current;
      pointer.smoothX = lerp(pointer.smoothX, pointer.x, 0.1);
      pointer.smoothY = lerp(pointer.smoothY, pointer.y, 0.1);

      const influence = Math.min(width, height) * (pointer.active ? 0.22 : 0.12);
      const strength = pointer.active ? 0.45 : 0.2;

      particlesRef.current.forEach((particle) => {
        const driftX = Math.sin(timestamp * 0.00025 + particle.seed) * particle.drift;
        const driftY = Math.cos(timestamp * 0.0003 + particle.seed) * particle.drift;
        const targetX = particle.baseX + driftX;
        const targetY = particle.baseY + driftY;

        const dx = particle.x - pointer.smoothX;
        const dy = particle.y - pointer.smoothY;
        const distance = Math.hypot(dx, dy);

        if (distance < influence && distance > 0.001) {
          const force = (1 - distance / influence) * strength;
          particle.vx += (dx / distance) * force;
          particle.vy += (dy / distance) * force;
        }

        particle.vx += (targetX - particle.x) * 0.0024;
        particle.vy += (targetY - particle.y) * 0.0024;
        particle.vx *= 0.9;
        particle.vy *= 0.9;
        particle.x += particle.vx;
        particle.y += particle.vy;

        const color = pickColor(particle.y / height, palette);
        const coreAlpha = particle.alpha * (pointer.active ? 0.95 : 0.65);
        const glowAlpha = particle.alpha * (pointer.active ? 0.45 : 0.3);

        context.beginPath();
        context.fillStyle = `rgba(${color.r}, ${color.g}, ${color.b}, ${coreAlpha})`;
        context.arc(particle.x, particle.y, particle.size, 0, Math.PI * 2);
        context.fill();

        context.beginPath();
        context.fillStyle = `rgba(${color.r}, ${color.g}, ${color.b}, ${glowAlpha})`;
        context.arc(particle.x, particle.y, particle.glow, 0, Math.PI * 2);
        context.fill();
      });

      rafRef.current = requestAnimationFrame(drawFrame);
    };

    rafRef.current = requestAnimationFrame(drawFrame);

    return () => {
      window.removeEventListener('resize', setSize);
      window.removeEventListener('pointermove', handlePointerMove);
      window.removeEventListener('pointerdown', handlePointerMove);
      window.removeEventListener('pointerleave', handlePointerLeave);
      window.removeEventListener('blur', handlePointerLeave);

      if (reduceMotionQuery.removeEventListener) {
        reduceMotionQuery.removeEventListener('change', handleReduceMotion);
      } else {
        reduceMotionQuery.removeListener(handleReduceMotion);
      }

      if (rafRef.current) {
        cancelAnimationFrame(rafRef.current);
      }
    };
  }, []);

  return <canvas className="mouse-field" ref={canvasRef} aria-hidden="true" />;
}

function App() {
  const [theme, setTheme] = useState('dark');

  useEffect(() => {
    document.documentElement.setAttribute('data-theme', theme);
  }, [theme]);

  const handleEmailClick = (email, subject, body) => {
    const encodedSubject = encodeURIComponent(subject);
    const encodedBody = encodeURIComponent(body);
    window.location.href = `mailto:${email}?subject=${encodedSubject}&body=${encodedBody}`;
  };

  const teamMembers = [
    {
      name: 'Oliver',
      email: 'oliver@dowhiz.com',
      pronoun: 'He/Him',
      nickname: 'Little-Bear',
      title: 'Writer',
      desc: 'Writer for daily office work across Notion, Overleaf, Google Docs, Google Slides, and Google Sheets.',
      example: 'Draft a project update in Notion and summarize it for stakeholders.',
      status: 'Active',
      img: oliverImg,
      subject: 'Office Task Request',
      body: 'Draft a project update in Notion and summarize it for stakeholders.'
    },
    {
      name: 'Maggie',
      email: 'maggie@dowhiz.com',
      pronoun: 'She/Her',
      nickname: 'Mini-Mouse',
      title: 'TPM',
      desc: 'TPM who turns meeting notes into action items, follows up with people and agents at milestones, updates the board, and sends daily reports.',
      example: "Summarize today's meeting, update action items, and send a daily report.",
      status: 'Active',
      img: miniMouseImg,
      subject: 'TPM Request',
      body: "Summarize today's meeting, turn notes into action items, update the board, and send a daily report."
    },
    {
      name: 'Devin',
      email: 'devin@dowhiz.com',
      pronoun: 'He/Him',
      nickname: 'Sticky-Octopus',
      title: 'Coder',
      desc: 'Coder handling daily development tasks and feature delivery.',
      example: 'Implement the requested feature and open a PR.',
      status: 'Coming',
      img: stickyOctopusImg,
      subject: 'Coding Task',
      body: 'Implement the requested feature and open a PR.'
    },
    {
      name: 'Lumio',
      email: 'lumio@dowhiz.com',
      pronoun: 'He/Him',
      nickname: 'Sky-Dragon',
      title: 'CEO',
      desc: 'CEO focused on strategy, leadership, and decision-making.',
      example: 'Draft a one-page strategy for Q2 goals.',
      status: 'Coming',
      img: skyDragonImg,
      subject: 'Strategy Request',
      body: 'Draft a one-page strategy for Q2 goals.'
    },
    {
      name: 'Claw',
      email: 'claw@dowhiz.com',
      pronoun: 'She/Her',
      nickname: 'Cozy-Lobster',
      title: 'OpenClaw',
      desc: 'OpenClaw: your personal AI assistant on any OS or platform. The lobster way.',
      example: 'Set up a cross-platform workflow for these tasks.',
      status: 'Coming',
      img: cozyLobsterImg,
      subject: 'Assistant Request',
      body: 'Set up a cross-platform workflow for these tasks.'
    },
    {
      name: 'Jeffery',
      email: 'jeffery@dowhiz.com',
      pronoun: 'He/Him',
      nickname: 'Strutton-Pigeon',
      title: 'DeepTutor',
      desc: 'DeepTutor helps you understand and manage documents and papers.',
      example: 'Summarize this paper and extract key takeaways.',
      status: 'Coming',
      img: struttonPigeonImg,
      subject: 'Document Help',
      body: 'Summarize this paper and extract key takeaways.'
    },
    {
      name: 'Anna',
      email: 'anna@dowhiz.com',
      pronoun: 'She/Her',
      nickname: 'Fluffy-Elephant',
      title: 'TBD',
      desc: 'Role definition in progress.',
      example: 'TBD.',
      status: 'Coming',
      img: fluffyElephantImg,
      subject: 'Role Request',
      body: 'Role definition in progress.'
    },
    {
      name: 'Rachel',
      email: 'rachel@dowhiz.com',
      pronoun: 'She/Her',
      nickname: 'Plush-Axolotl',
      title: 'GTM Specialist',
      desc: 'GTM specialist tracking team status and product progress, publishing posts to LinkedIn, Xiaohongshu, Reddit, YouTube, X, Medium, Product Hunt, Hacker News, and WeChat groups.',
      example: "Prepare and schedule this week's multi-platform launch posts.",
      status: 'Coming',
      img: plushAxolotlImg,
      subject: 'GTM Request',
      body: 'Prepare posts across LinkedIn, Xiaohongshu, Reddit, YouTube, X, Medium, Product Hunt, Hacker News, and WeChat groups.'
    }
  ];

  return (
    <div className="app-container">
      <MouseField theme={theme} />
      <div className="content-layer">
        {/* Navigation */}
        <nav className="navbar">
          <div className="container nav-content">
            <a href="#" className="logo">Do<span className="text-gradient">Whiz</span></a>
            <div className="nav-links">
              <a href="#roles" className="nav-btn">Team</a>
              <a href="#features" className="nav-btn">Features</a>
              <a href="#security" className="nav-btn">Security</a>
            </div>
            <div className="nav-actions">
              <div className="social-links">
                <a href="https://github.com/KnoWhiz/DoWhiz" target="_blank" rel="noopener noreferrer" className="btn-small" aria-label="GitHub">
                  <svg viewBox="0 0 24 24" width="20" height="20" stroke="currentColor" strokeWidth="2" fill="none" strokeLinecap="round" strokeLinejoin="round">
                    <path d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22"></path>
                  </svg>
                  <span>GitHub</span>
                </a>
                <a href="https://discord.gg/7ucnweCKk8" target="_blank" rel="noopener noreferrer" className="btn-small" aria-label="Discord">
                  <svg viewBox="0 0 24 24" width="20" height="20" stroke="currentColor" strokeWidth="2" fill="none" strokeLinecap="round" strokeLinejoin="round">
                    <path d="M21 11.5a8.38 8.38 0 0 1-.9 3.8 8.5 8.5 0 0 1-7.6 4.7 8.38 8.38 0 0 1-3.8-.9L3 21l1.9-5.7a8.38 8.38 0 0 1-.9-3.8 8.5 8.5 0 0 1 4.7-7.6 8.38 8.38 0 0 1 3.8-.9h.5a8.48 8.48 0 0 1 8 8v.5z"></path>
                  </svg>
                  <span>Discord</span>
                </a>
                <a className="btn-small" href="mailto:admin@dowhiz.com" aria-label="Contact">
                  <svg viewBox="0 0 24 24" width="20" height="20" stroke="currentColor" strokeWidth="2" fill="none" strokeLinecap="round" strokeLinejoin="round">
                    <path d="M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z"></path>
                    <polyline points="22,6 12,13 2,6"></polyline>
                  </svg>
                  <span>Contact</span>
                </a>
              </div>
            </div>
          </div>
        </nav>

        {/* Hero Section */}
        <section className="hero-section">
          <div className="halo-effect"></div>
          <div className="container hero-content">
            <h1 className="hero-title">
              Empower Everyone<br />
              <span className="text-gradient">with A Digital Employee Team</span>
            </h1>
            <p className="hero-subtitle">
              Seamlessly collaborate with <a href="#roles" className="role-link">Oliver</a>, <a href="#roles" className="role-link">Maggie</a>, and your entire digital workforce—directly from your email inbox. Soon you will also reach them by phone, Slack, Discord, WhatsApp, and more.
            </p>
            <div className="hero-cta">
              <a className="btn btn-primary" href="mailto:oliver@dowhiz.com">
                Start Free Trial
              </a>
            </div>
          </div>
        </section>

        {/* Roles & Scenarios */}
        <section id="roles" className="section roles-section">
          <div className="container">
            <h2 className="section-title">Meet Your Digital Employee Team</h2>
            <div className="roles-grid">
              {teamMembers.map((member) => {
                const isActive = member.status === 'Active';
                const cardClasses = `role-card ${isActive ? 'active-role clickable-card' : 'coming-soon'}`;

                return (
                  <div
                    key={member.name}
                    className={cardClasses}
                    onClick={isActive ? () => handleEmailClick(member.email, member.subject, member.body) : undefined}
                    title={isActive ? `Click to email ${member.name}` : `${member.name} is coming soon`}
                  >
                    <div className="role-header">
                      <div className="role-profile">
                        <img src={member.img} alt={member.name} className="role-avatar" />
                        <div>
                          <h3>{member.name}</h3>
                          <div className="role-title">
                            <span className="role-title-text">{member.title}</span>
                            <span className="pronoun-tag">{member.pronoun}</span>
                          </div>
                          <code className="email-tag">{member.email}</code>
                          <div className="nickname-tag">{member.nickname}</div>
                        </div>
                      </div>
                      <span className={`status-badge ${isActive ? 'status-active' : 'status-soon'}`}>{member.status}</span>
                    </div>
                    <p className="role-desc">{member.desc}</p>
                    <div className="role-example">
                      <span className="example-label">Example Task</span>
                      <p>"{member.example}"</p>
                    </div>
                  </div>
                );
              })}
            </div>
          </div>
        </section>

        {/* Core Experience */}
        <section id="features" className="section features-section">
          <div className="container">
            <h2 className="section-title">Core Features</h2>
            <div className="features-grid">
              {[
                { title: "Multi-Channel Access", desc: "Start with email today, and soon connect via phone, Slack, Discord, WhatsApp, and other channels." },
                { title: "Autonomous Tools", desc: "Agents intelligently select and utilize the right tools for each request without manual configuration." },
                { title: "Direct Delivery", desc: "Completed work is delivered straight back to your inbox. Focus on results, not the process." }
              ].map((item, idx) => (
                <div key={idx} className="feature-card">
                  <h3>{item.title}</h3>
                  <p>{item.desc}</p>
                </div>
              ))}
            </div>
          </div>
        </section>

        {/* Workflow */}
        <section className="section workflow-section">
          <div className="container">
            <h2 className="section-title">How It Works</h2>
            <div className="workflow-container">
              <div className="workflow-line"></div>
              {[
                { step: "1", label: "Message", img: workflowMessage },
                { step: "2", label: "Triage", img: workflowTriage },
                { step: "3", label: "Execution", img: workflowExecution },
                { step: "4", label: "Return", img: workflowReturn },
              ].map((s, i) => (
                <div key={i} className="workflow-step" style={{ animationDelay: `${i * 0.2}s` }}>
                  <div className="step-icon-wrapper">
                    <div className="step-icon">
                      <img src={s.img} alt={s.label} />
                    </div>
                  </div>
                  <h4>{s.label}</h4>
                </div>
              ))}
            </div>
          </div>
        </section>

        {/* Security */}
        <section id="security" className="section security-section">
          <div className="container">
            <h2 className="section-title">Security First Design</h2>
            <div className="security-grid">
              <div className="security-card">
                <h3>Isolation</h3>
                <p>
                  User data and agent memories are strictly isolated. Your data never leaks to other tenants.
                </p>
              </div>
              <div className="security-card">
                <h3>Auditability & Trust</h3>
                <p>
                  All actions are logged and reversible. You maintain full control over your digital workforce.
                </p>
              </div>
            </div>
          </div>
        </section>

        {/* Footer */}
        <footer>
          <p>&copy; {new Date().getFullYear()} DoWhiz. All rights reserved.</p>
        </footer>

        {/* Theme Toggle */}
        <button
          className="theme-toggle"
          onClick={() => setTheme(theme === 'dark' ? 'light' : 'dark')}
          aria-label="Toggle theme"
        >
          {theme === 'dark' ? (
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeLinecap="round" strokeLinejoin="round">
              <circle cx="12" cy="12" r="5"></circle>
              <line x1="12" y1="1" x2="12" y2="3"></line>
              <line x1="12" y1="21" x2="12" y2="23"></line>
              <line x1="4.22" y1="4.22" x2="5.64" y2="5.64"></line>
              <line x1="18.36" y1="18.36" x2="19.78" y2="19.78"></line>
              <line x1="1" y1="12" x2="3" y2="12"></line>
              <line x1="21" y1="12" x2="23" y2="12"></line>
              <line x1="4.22" y1="19.78" x2="5.64" y2="18.36"></line>
              <line x1="18.36" y1="5.64" x2="19.78" y2="4.22"></line>
            </svg>
          ) : (
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeLinecap="round" strokeLinejoin="round">
              <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path>
            </svg>
          )}
        </button>
      </div>
    </div>
  );
}

export default App;

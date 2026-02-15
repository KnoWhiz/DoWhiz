import { useState, useEffect, useRef } from 'react';
import oliverImg from './assets/Oliver.jpg';
import miniMouseImg from './assets/Mini-Mouse.jpg';
import stickyOctopusImg from './assets/Sticky-Octopus.jpg';
import skyDragonImg from './assets/Sky-Dragon.jpg';
import cozyLobsterImg from './assets/Cozy-Lobster.jpg';
import struttonPigeonImg from './assets/Strutton-Pigeon.jpg';
import fluffyElephantImg from './assets/Fluffy-Elephant.jpg';
import plushAxolotlImg from './assets/Plush-Axolotl.jpg';

const WAITLIST_FORM_URL = 'https://docs.google.com/forms/d/1UgZpFgYxq0uSjmVdai1mpjbfj2GxcWakFt3YKL8by34/viewform';
const DAY_START_HOUR = 7;
const NIGHT_START_HOUR = 19;

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

const getThemeForLocalTime = (date = new Date()) => {
  const hour = date.getHours();
  return hour >= DAY_START_HOUR && hour < NIGHT_START_HOUR ? 'light' : 'dark';
};

const getNextThemeSwitch = (date = new Date()) => {
  const next = new Date(date);
  const hour = date.getHours();

  if (hour >= DAY_START_HOUR && hour < NIGHT_START_HOUR) {
    next.setHours(NIGHT_START_HOUR, 0, 0, 0);
    return next;
  }

  next.setHours(DAY_START_HOUR, 0, 0, 0);
  if (hour >= NIGHT_START_HOUR) {
    next.setDate(next.getDate() + 1);
  }
  return next;
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
  const [theme, setTheme] = useState(() => getThemeForLocalTime());

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

  const buildMailtoLink = (email, subject, body) => {
    const encodedSubject = encodeURIComponent(subject);
    const encodedBody = encodeURIComponent(body);
    return `mailto:${email}?subject=${encodedSubject}&body=${encodedBody}`;
  };

  const features = [
    {
      tag: '01',
      title: 'Inbox-native delegation',
      desc: 'Send requests the way you already work: email a digital employee, attach files, and get finished results back in-thread.'
    },
    {
      tag: '02',
      title: 'Specialized agent playbooks',
      desc: 'Each employee is trained for a role (Writer, TPM, Coder, CEO, and more) so outputs are tailored, not generic.'
    },
    {
      tag: '03',
      title: 'Visible, step-by-step delivery',
      desc: 'Expect a clear flow: brief intake, execution, and a tidy handoff with next steps for your team.'
    },
    {
      tag: '04',
      title: 'Multi-format outputs',
      desc: 'Documents, spreadsheets, summaries, posts, and code—delivered in formats your team can immediately reuse.'
    },
    {
      tag: '05',
      title: 'Multi-channel roadmap',
      desc: 'Email first today. Slack, phone, Discord, WhatsApp, and more are coming as we expand access.'
    },
    {
      tag: '06',
      title: 'Privacy-first foundation',
      desc: 'Clear data boundaries with a focus on practical, secure workflows for real business tasks.'
    }
  ];

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
      imgAlt: 'Illustration of Oliver the Little-Bear, DoWhiz writer digital employee.',
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
      status: 'Active',
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
      status: 'Coming',
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
      status: 'Coming',
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
      title: 'OpenClaw',
      desc: 'OpenClaw: your personal AI assistant on any OS or platform. The lobster way.',
      example: 'Set up a cross-platform workflow for these tasks.',
      status: 'Coming',
      img: cozyLobsterImg,
      imgAlt: 'Illustration of Claw the Cozy-Lobster, DoWhiz OpenClaw assistant.',
      subject: 'Assistant Request',
      body: 'Set up a cross-platform workflow for these tasks.',
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
      status: 'Coming',
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
      status: 'Coming',
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
      status: 'Coming',
      img: plushAxolotlImg,
      imgAlt: 'Illustration of Rachel the Plush-Axolotl, DoWhiz GTM specialist.',
      subject: 'GTM Request',
      body: 'Prepare posts across LinkedIn, Xiaohongshu, Reddit, YouTube, X, Medium, Product Hunt, Hacker News, and WeChat groups.',
      profilePath: '/agents/rachel/'
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
              <a href="#features" className="nav-btn">Features</a>
              <a href="#roles" className="nav-btn">Team</a>
              <a href="/user-guide/" className="nav-btn">User Guide</a>
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
              Seamlessly collaborate with <a href="#roles" className="role-link">Oliver 🧸</a> (Writer), <a href="#roles" className="role-link">Maggie 🐭</a> (TPM), <a href="#roles" className="role-link">Devin 🐙</a> (Coder), <a href="#roles" className="role-link">Lumio 🐉</a> (CEO), <a href="#roles" className="role-link">Claw 🦞</a> (OpenClaw assistant), <a href="#roles" className="role-link">Jeffery 🐦</a> (DeepTutor), <a href="#roles" className="role-link">Anna 🐘</a> (role in progress), and <a href="#roles" className="role-link">Rachel 👾</a> (GTM Specialist)—directly from your email inbox. Soon you will also reach them by phone, Slack, Discord, WhatsApp, and more.
            </p>
            <div className="hero-cta">
              <a className="btn btn-primary" href={WAITLIST_FORM_URL} target="_blank" rel="noopener noreferrer">
                Join waitlist
              </a>
            </div>
          </div>
        </section>

        {/* Features */}
        <section id="features" className="section features-section">
          <div className="container">
            <h2 className="section-title">The Digital Employee Stack</h2>
            <p className="section-intro">
              Built for real teams that live in their inbox. Pick an employee, send a request, and receive finished work with clear next steps.
            </p>
            <div className="features-grid">
              {features.map((feature) => (
                <div key={feature.tag} className="feature-card">
                  <span className="feature-tag">{feature.tag}</span>
                  <h3>{feature.title}</h3>
                  <p>{feature.desc}</p>
                </div>
              ))}
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
                const cardClasses = `role-card ${isActive ? 'active-role' : 'coming-soon'}`;

                return (
                  <div
                    key={member.name}
                    className={cardClasses}
                    title={isActive ? `Email ${member.name} at ${member.email}` : `${member.name} is coming soon`}
                  >
                    <div className="role-header">
                      <div className="role-profile">
                        <img src={member.img} alt={member.imgAlt} className="role-avatar" />
                        <div>
                          <h3>{member.name}</h3>
                          <div className="role-title">
                            <span className="role-title-text">{member.title}</span>
                            <span className="pronoun-tag">{member.pronoun}</span>
                          </div>
                          {isActive ? (
                            <a
                              className="email-tag"
                              href={buildMailtoLink(member.email, member.subject, member.body)}
                              target="_blank"
                              rel="noopener noreferrer"
                              aria-label={`Email ${member.name}`}
                            >
                              {member.email}
                            </a>
                          ) : (
                            <span className="email-tag" aria-disabled="true">
                              {member.email}
                            </span>
                          )}
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
                    <div className="role-actions">
                      <a
                        href={member.profilePath}
                        className="profile-link"
                      >
                        View profile
                      </a>
                      <span className="email-hint">
                        {isActive ? 'Click email to send' : 'Coming soon'}
                      </span>
                    </div>
                  </div>
                );
              })}
            </div>
          </div>
        </section>

        {/* Footer */}
        <footer className="site-footer">
          <div className="container footer-content">
            <div className="footer-brand">
              <a href="#" className="footer-logo">
                Do<span className="text-gradient">Whiz</span>
              </a>
              <p className="footer-tagline">
                Digital employees that turn messages into finished work, delivered back to your inbox.
              </p>
              <div className="footer-pill">Email-first today. Multi-channel soon.</div>
            </div>
            <div className="footer-links">
              <span className="footer-title">Essentials</span>
              <div className="footer-link-grid">
                <a href="/privacy/" className="footer-link">Privacy</a>
                <a href="/terms/" className="footer-link">Terms of Service</a>
                <a href="/user-guide/" className="footer-link">User Guide</a>
                <a href="mailto:admin@dowhiz.com" className="footer-link">Contact</a>
              </div>
            </div>
          </div>
          <div className="container footer-bottom">
            <span>&copy; {new Date().getFullYear()} DoWhiz. All rights reserved.</span>
            <span>Built for teams that live in their inbox.</span>
          </div>
        </footer>

      </div>
    </div>
  );
}

export default App;

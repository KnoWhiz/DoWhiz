import { useCallback, useEffect, useRef, useState } from 'react';
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
import StartupIntakeConversation from '../components/intake/StartupIntakeConversation';
import { getLandingContent } from './landingContent';

const SITE_URL = 'https://dowhiz.com';
const LOGO_URL = `${SITE_URL}/assets/DoWhiz.svg`;
const SUPPORT_EMAIL = 'admin@dowhiz.com';
const ORG_NAME = 'DoWhiz';
const CN_PATH_PREFIX = '/cn';

const isCnPath = (pathname = '/') =>
  pathname === CN_PATH_PREFIX || pathname === `${CN_PATH_PREFIX}/` || pathname.startsWith(`${CN_PATH_PREFIX}/`);

const getLocalizedAuthPath = (suffix = '', pathname = typeof window !== 'undefined' ? window.location.pathname : '/') =>
  `${isCnPath(pathname) ? CN_PATH_PREFIX : ''}/auth/index.html${suffix}`;

const updateMetaContent = (selector, content) => {
  if (typeof document === 'undefined' || !content) {
    return;
  }

  const node = document.querySelector(selector);
  if (node) {
    node.setAttribute('content', content);
  }
};

const updateLinkHref = (selector, href) => {
  if (typeof document === 'undefined' || !href) {
    return;
  }

  const node = document.querySelector(selector);
  if (node) {
    node.setAttribute('href', href);
  }
};

function LandingPage({ locale }) {
  const pathname = typeof window !== 'undefined' ? window.location.pathname : '/';
  const pageLocale = locale || (isCnPath(pathname) ? 'zh-CN' : 'en-US');
  const isChinesePage = pageLocale === 'zh-CN';
  const content = getLandingContent(pageLocale);
  const localizedHomePath = content.nav.homePath;
  const [theme, setTheme] = useState(() => getThemeForLocalTime());
  const [enableMouseField, setEnableMouseField] = useState(false);
  const [user, setUser] = useState(null);
  const [showUserMenu, setShowUserMenu] = useState(false);
  const [navHidden, setNavHidden] = useState(false);
  const userMenuRef = useRef(null);
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
        language: pageLocale
      },
      {
        eventKey: `landing_page_view:${sessionId}:${window.location.pathname}`
      }
    );
  }, [pageLocale]);

  useEffect(() => {
    if (typeof document === 'undefined') {
      return;
    }

    document.documentElement.lang = content.metadata.htmlLang;
    document.title = content.metadata.title;

    updateMetaContent('meta[name="description"]', content.metadata.description);
    updateMetaContent('meta[property="og:title"]', content.metadata.title);
    updateMetaContent('meta[property="og:description"]', content.metadata.description);
    updateMetaContent('meta[property="og:url"]', content.metadata.canonicalUrl);
    updateMetaContent('meta[property="og:locale"]', content.metadata.ogLocale);
    updateMetaContent('meta[name="twitter:title"]', content.metadata.title);
    updateMetaContent('meta[name="twitter:description"]', content.metadata.description);
    updateMetaContent('meta[name="theme-color"]', content.metadata.themeColor);
    updateLinkHref('link[rel="canonical"]', content.metadata.canonicalUrl);
  }, [content.metadata]);

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

  const features = content.features;
  const howItWorksSteps = content.howItWorksSteps;
  const safetyItems = content.safetyItems;
  const accessFlowSteps = content.accessFlowSteps;
  const blogPosts = content.blogPosts;
  const faqItems = content.faqItems;
  const assetByKey = {
    oliver: oliverImg,
    miniMouse: miniMouseImg,
    stickyOctopus: stickyOctopusImg,
    skyDragon: skyDragonImg,
    cozyLobster: cozyLobsterImg,
    struttonPigeon: struttonPigeonImg,
    fluffyElephant: fluffyElephantImg,
    plushAxolotl: plushAxolotlImg
  };
  const workflowExamples = content.workflowExamples.map((workflow) => ({
    ...workflow,
    avatar: assetByKey[workflow.avatarKey] || oliverImg
  }));
  const teamMembers = content.teamMembers.map((member) => ({
    ...member,
    img: assetByKey[member.imageKey] || oliverImg
  }));

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
            availableLanguage: isChinesePage ? ['Chinese', 'English'] : ['English']
          }
        ],
        sameAs: ['https://github.com/KnoWhiz/DoWhiz']
      },
      {
        '@type': 'FAQPage',
        '@id': `${content.metadata.canonicalUrl}#faq`,
        url: content.metadata.canonicalUrl,
        inLanguage: content.metadata.htmlLang,
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
  const oliverContactHref = buildMailtoLink('oliver@dowhiz.com', content.hero.contactSubject, content.hero.contactBody);

  const handleHeroIntakeViewed = useCallback(() => {
    const sessionId = getOrCreateSessionId();
    trackAnalyticsEvent(
      'hero_intake_viewed',
      {
        intake_location: 'homepage_hero'
      },
      {
        eventKey: `hero_intake_viewed:${sessionId}:${window.location.pathname}`
      }
    );
  }, []);

  const handleHeroIntakeStarted = useCallback(() => {
    trackAnalyticsEvent('hero_intake_started', {
      intake_location: 'homepage_hero'
    });
  }, []);

  const handleHeroIntakeSubmitted = useCallback((properties = {}) => {
    trackAnalyticsEvent('hero_intake_submitted', {
      intake_location: 'homepage_hero',
      ...properties
    });
  }, []);

  const handleHeroIntakeHandoff = useCallback((properties = {}) => {
    trackAnalyticsEvent('hero_intake_handoff', {
      intake_location: 'homepage_hero',
      ...properties
    });
  }, []);

  const handleHeroContactCtaClick = () => {
    trackAnalyticsEvent('hero_secondary_cta_clicked', {
      cta_location: 'hero_secondary',
      cta_text: content.hero.secondaryCta,
      cta_destination: oliverContactHref
    });
    trackAnalyticsEvent('secondary_cta_click', {
      cta_location: 'hero_secondary',
      cta_text: content.hero.secondaryCta
    });
  };

  const handleSignupCtaClick = () => {
    trackAnalyticsEvent('primary_cta_click', {
      cta_location: 'nav_sign_in',
      cta_text: content.nav.signIn
    });
  };

  return (
    <div className={`app-container landing-page${isChinesePage ? ' landing-page-cn' : ''}`}>
      <script
        type="application/ld+json"
        dangerouslySetInnerHTML={{ __html: JSON.stringify(structuredData) }}
      />
      <div className="content-layer">
        {/* Navigation */}
        <nav className={`navbar${navHidden ? ' nav-hidden' : ''}`}>
          <div className="nav-content">
            <a href={localizedHomePath} className="logo">
              <img src="/assets/DoWhiz.svg" alt="" className="brand-mark" aria-hidden="true" />
              <span>Do<span className="text-gradient">Whiz</span></span>
            </a>
            <div className="nav-links">
              {content.nav.links.map((link) => (
                <a key={link.href} href={link.href} className="nav-btn">
                  {link.label}
                </a>
              ))}
            </div>
            <div className="nav-actions">
              <div className="social-links">
                <a
                  href="https://github.com/KnoWhiz/DoWhiz"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="btn-small"
                  aria-label={content.nav.githubAriaLabel}
                >
                  <svg viewBox="0 0 24 24" width="20" height="20" stroke="currentColor" strokeWidth="2" fill="none" strokeLinecap="round" strokeLinejoin="round">
                    <path d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22"></path>
                  </svg>
                </a>
                <a
                  href="https://discord.gg/7ucnweCKk8"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="btn-small"
                  aria-label={content.nav.discordAriaLabel}
                >
                  <svg viewBox="0 0 24 24" width="20" height="20" stroke="currentColor" strokeWidth="2" fill="none" strokeLinecap="round" strokeLinejoin="round">
                    <path d="M21 11.5a8.38 8.38 0 0 1-.9 3.8 8.5 8.5 0 0 1-7.6 4.7 8.38 8.38 0 0 1-3.8-.9L3 21l1.9-5.7a8.38 8.38 0 0 1-.9-3.8 8.5 8.5 0 0 1 4.7-7.6 8.38 8.38 0 0 1 3.8-.9h.5a8.48 8.48 0 0 1 8 8v.5z"></path>
                  </svg>
                </a>
                <a className="btn-small" href={oliverContactHref} aria-label={content.nav.contactAriaLabel}>
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
                      aria-label={user.user_metadata?.full_name || user.email || (isChinesePage ? '用户菜单' : 'User menu')}
                      title={user.user_metadata?.full_name || user.email || (isChinesePage ? '用户菜单' : 'User menu')}
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
                          {content.nav.dashboard}
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
                          {content.nav.signOut}
                        </button>
                      </div>
                    )}
                  </div>
                ) : (
                  <a
                    className="btn-small"
                    href={getLocalizedAuthPath()}
                    aria-label={content.nav.signIn}
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
            <div className="hero-copy">
              {content.hero.eyebrow ? <p className="hero-eyebrow">{content.hero.eyebrow}</p> : null}
              <h1 className="hero-title">{content.hero.title}</h1>
              <p className="hero-subtitle">{content.hero.subtitle}</p>
              {content.hero.note ? <p className="hero-note">{content.hero.note}</p> : null}
              {content.hero.chips.length ? (
                <div className="hero-chip-row" aria-label={isChinesePage ? '支持的触发表面' : 'Supported trigger surfaces'}>
                  {content.hero.chips.map((chip) => (
                    <span key={chip} className="hero-chip">
                      {chip}
                    </span>
                  ))}
                </div>
              ) : null}
              {content.hero.pillars.length ? (
                <div className="hero-pillar-grid">
                  {content.hero.pillars.map((pillar) => (
                    <article key={pillar.label} className="hero-pillar-card">
                      <span className="hero-pillar-label">{pillar.label}</span>
                      <h3 className="hero-pillar-title">{pillar.value}</h3>
                      <p className="hero-pillar-desc">{pillar.desc}</p>
                    </article>
                  ))}
                </div>
              ) : null}
            </div>
            <aside className="hero-intake-panel" aria-label={content.hero.intakeAriaLabel}>
              <div className="hero-intake-header">
                <p className="hero-intake-kicker">{content.hero.intakeKicker}</p>
                <h2>{content.hero.intakeTitle}</h2>
                <p>{content.hero.intakeDescription}</p>
              </div>
              <StartupIntakeConversation
                variant="hero"
                copy={content.intakeConversation}
                showDraftDetails={false}
                showBlueprintDetails={false}
                onViewed={handleHeroIntakeViewed}
                onStarted={handleHeroIntakeStarted}
                onSubmitted={handleHeroIntakeSubmitted}
                onHandoff={handleHeroIntakeHandoff}
              />
            </aside>
            <div className="hero-secondary-actions">
              <a
                className="btn btn-secondary hero-secondary-cta"
                href={oliverContactHref}
                onClick={handleHeroContactCtaClick}
              >
                {content.hero.secondaryCta}
              </a>
            </div>
          </div>
        </section>

        {/* Roles & Scenarios */}
        <section id="roles" className="section roles-section">
          <div className="container">
            <h2 className="section-title">{content.sections.rolesTitle}</h2>
            {content.sections.rolesIntro ? (
              <p className="section-intro roles-intro">{content.sections.rolesIntro}</p>
            ) : null}
            <div className="roles-grid">
              {teamMembers.map((member) => {
                const isActive = member.statusKey === 'active';
                const cardClasses = `role-card ${isActive ? 'active-role' : 'coming-soon'}`;

                return (
                  <div
                    key={member.name}
                    className={cardClasses}
                    title={
                      isChinesePage
                        ? `${member.name}：查看可用渠道与触发方式`
                        : `${member.name}: view channels and trigger examples`
                    }
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
                                aria-label={isChinesePage ? `给 ${member.name} 发邮件` : `Email ${member.name}`}
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
                      <span className="example-label">{content.labels.exampleTask}</span>
                      <p>"{member.example}"</p>
                    </div>
                    <div className="role-actions">
                      <a
                        href={member.profilePath}
                        className="profile-link"
                      >
                        {isChinesePage ? content.labels.viewProfileEnglish : content.labels.viewProfile}
                      </a>
                      <span className="email-hint">
                        {isActive ? content.labels.activeHint : content.labels.soonHint}
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
            <h2 className="section-title">{content.sections.howTitle}</h2>
            <p className="section-intro">{content.sections.howIntro}</p>
            <div className="how-columns">
              {howItWorksSteps.map((step) => {
                const isUserStep = isChinesePage ? step.role.includes('用户') : step.role.toLowerCase().includes('user');
                const icon = isUserStep ? '/icons/user.svg' : '/icons/agent.svg';
                return (
                  <div key={step.id} className="how-column">
                    <div className="how-head-cell">
                      <div className="how-head-badge">{step.id}</div>
                      <div className="how-head-title">{step.phase}</div>
                    </div>

                    <div className="how-stack">
                      <div className="how-card role-card-variant">
                        <div className="how-card-heading">
                          <img src={icon} alt={isChinesePage ? `${step.role} 图标` : `${step.role} icon`} className="how-card-icon" />
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
                            <img
                              src="/icons/output.svg"
                              alt={isChinesePage ? '输出图标' : 'Output icon'}
                              className="how-card-icon"
                            />
                            <span className="how-card-title">{content.labels.output}</span>
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
            <h2 className="section-title">{content.sections.workflowsTitle}</h2>
            <p className="section-intro">{content.sections.workflowsIntro}</p>
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
                      <img
                        className="workflow-avatar"
                        src={workflow.avatar}
                        alt={isChinesePage ? `${workflow.owner} 头像` : `${workflow.owner} avatar`}
                      />
                      <h3>{workflow.title}</h3>
                    </div>
                    <div className="workflow-block">
                      <span className="workflow-label">{content.labels.trigger}</span>
                      <p>{workflow.trigger}</p>
                    </div>
                    <div className="workflow-block">
                      <span className="workflow-label">{content.labels.execution}</span>
                      <ul className="workflow-execution-list">
                        {workflow.execution.map((item) => (
                          <li key={item}>{item}</li>
                        ))}
                      </ul>
                    </div>
                    <div className="workflow-block">
                      <span className="workflow-label">{content.labels.result}</span>
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
            <h2 className="section-title">{content.sections.safetyTitle}</h2>
            <p className="section-intro">{content.sections.safetyIntro}</p>
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
                <h3>{content.labels.accessPlaybookTitle}</h3>
                <p>{content.labels.accessPlaybookDescription}</p>
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
                  {content.labels.accessPlaybookLink}
                </a>
              </aside>
            </div>
          </div>
        </section>

        {/* Features */}
        <section id="features" className="section features-section">
          <div className="container">
            <h2 className="section-title">{content.sections.featuresTitle}</h2>
            <p className="section-intro">{content.sections.featuresIntro}</p>
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

        {/* FAQ */}
        <section id="faq" className="section faq-section">
          <div className="container">
            <h2 className="section-title">{content.sections.faqTitle}</h2>
            <p className="section-intro">{content.sections.faqIntro}</p>
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
                {content.labels.faqCta}
              </a>
            </div>
          </div>
        </section>

        {/* Blog */}
        <section id="blog" className="section blog-section">
          <div className="container">
            <div className="blog-header">
              <div>
                <span className="blog-eyebrow">{content.labels.blogEyebrow}</span>
                <h2 className="blog-title">{content.labels.blogTitle}</h2>
                <p className="blog-intro">{content.labels.blogIntro}</p>
              </div>
              <a className="btn btn-secondary blog-header-btn" href="/blog/">{content.labels.blogHeaderButton}</a>
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
                    {content.labels.blogLinkLabel}
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
              <a href={localizedHomePath} className="footer-logo">
                <img src="/assets/DoWhiz.svg" alt="" className="footer-brand-mark" aria-hidden="true" />
                <span>Do<span className="text-gradient">Whiz</span></span>
              </a>
              <p className="footer-tagline">{content.labels.footerTagline}</p>
              <div className="footer-pill">{content.labels.footerPill}</div>
            </div>
            <div className="footer-links">
              <span className="footer-title">{content.labels.footerTitle}</span>
              <div className="footer-link-grid">
                {content.footerLinks.map((link) => (
                  <a key={link.href} href={link.href} className="footer-link">
                    {link.label}
                  </a>
                ))}
                <a href={`mailto:${SUPPORT_EMAIL}`} className="footer-link">
                  {isChinesePage ? '联系邮箱' : 'Contact'}
                </a>
              </div>
            </div>
          </div>
          <div className="container footer-bottom">
            <span>
              &copy; {new Date().getFullYear()} DoWhiz. {isChinesePage ? '保留所有权利。' : 'All rights reserved.'}
            </span>
            <span>{content.labels.footerBottomSecondary}</span>
          </div>
        </footer>

      </div>
    </div>
  );
}

export default LandingPage;

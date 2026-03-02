(function () {
  const DAY_START_HOUR = 7;
  const NIGHT_START_HOUR = 19;

  function getThemeForLocalTime() {
    const hour = new Date().getHours();
    return hour >= DAY_START_HOUR && hour < NIGHT_START_HOUR ? 'light' : 'dark';
  }

  function applyTheme() {
    document.documentElement.setAttribute('data-theme', getThemeForLocalTime());
  }

  function scheduleNextThemeSwitch() {
    const now = new Date();
    const hour = now.getHours();
    const next = new Date(now);

    if (hour >= DAY_START_HOUR && hour < NIGHT_START_HOUR) {
      next.setHours(NIGHT_START_HOUR, 0, 0, 0);
    } else {
      next.setHours(DAY_START_HOUR, 0, 0, 0);
      if (hour >= NIGHT_START_HOUR) {
        next.setDate(next.getDate() + 1);
      }
    }

    const delay = Math.max(next.getTime() - now.getTime(), 0);
    setTimeout(function () {
      applyTheme();
      scheduleNextThemeSwitch();
    }, delay);
  }

  function ensureSharedNavStyles() {
    if (document.getElementById('dw-shared-nav-styles')) {
      return;
    }

    const link = document.createElement('link');
    link.id = 'dw-shared-nav-styles';
    link.rel = 'stylesheet';
    link.href = '/shared-nav.css';
    document.head.appendChild(link);
  }

  function shouldMountSharedNav() {
    const pathname = window.location.pathname;
    return pathname !== '/' && pathname !== '/index.html';
  }

  function getActiveNavHref(pathname) {
    if (pathname.startsWith('/agents/')) {
      return '/#roles';
    }

    if (
      pathname.startsWith('/solutions/') ||
      pathname.startsWith('/demo-videos/') ||
      pathname.startsWith('/integrations/')
    ) {
      return '/#workflows';
    }

    if (pathname.startsWith('/trust-safety/')) {
      return '/#safety';
    }

    if (pathname.startsWith('/help-center/')) {
      return '/#faq';
    }

    if (pathname.startsWith('/agent-market/')) {
      return '/#deployment';
    }

    if (pathname.startsWith('/blog/')) {
      return '/#blog';
    }

    if (pathname.startsWith('/user-guide/')) {
      return '/#how-it-works';
    }

    if (
      pathname.startsWith('/privacy/') ||
      pathname.startsWith('/terms/') ||
      pathname.startsWith('/auth/')
    ) {
      return '/#features';
    }

    return '';
  }

  function buildSharedNav() {
    return [
      '<div class="nav-content">',
      '  <a href="/" class="logo" aria-label="DoWhiz homepage">',
      '    <img src="/assets/DoWhiz.jpeg" alt="" class="brand-mark" aria-hidden="true" />',
      '    <span>Do<span class="text-gradient">Whiz</span></span>',
      '  </a>',
      '  <div class="nav-links">',
      '    <a href="/#roles" class="nav-btn">Team</a>',
      '    <a href="/#how-it-works" class="nav-btn">How it works</a>',
      '    <a href="/#workflows" class="nav-btn">Workflows</a>',
      '    <a href="/#safety" class="nav-btn">Safety</a>',
      '    <a href="/#features" class="nav-btn">Features</a>',
      '    <a href="/#deployment" class="nav-btn">Deployment</a>',
      '    <a href="/#faq" class="nav-btn">FAQ</a>',
      '    <a href="/#blog" class="nav-btn">Blog</a>',
      '  </div>',
      '  <div class="nav-actions">',
      '    <div class="social-links">',
      '      <a href="https://github.com/KnoWhiz/DoWhiz" target="_blank" rel="noopener noreferrer" class="btn-small" aria-label="GitHub">',
      '        <svg viewBox="0 0 24 24" width="20" height="20" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round">',
      '          <path d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22"></path>',
      '        </svg>',
      '      </a>',
      '      <a href="https://discord.gg/7ucnweCKk8" target="_blank" rel="noopener noreferrer" class="btn-small" aria-label="Discord">',
      '        <svg viewBox="0 0 24 24" width="20" height="20" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round">',
      '          <path d="M21 11.5a8.38 8.38 0 0 1-.9 3.8 8.5 8.5 0 0 1-7.6 4.7 8.38 8.38 0 0 1-3.8-.9L3 21l1.9-5.7a8.38 8.38 0 0 1-.9-3.8 8.5 8.5 0 0 1 4.7-7.6 8.38 8.38 0 0 1 3.8-.9h.5a8.48 8.48 0 0 1 8 8v.5z"></path>',
      '        </svg>',
      '      </a>',
      '      <a class="btn-small" href="mailto:admin@dowhiz.com" aria-label="Contact">',
      '        <svg viewBox="0 0 24 24" width="20" height="20" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round">',
      '          <path d="M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z"></path>',
      '          <polyline points="22,6 12,13 2,6"></polyline>',
      '        </svg>',
      '      </a>',
      '      <a class="btn-small" href="/auth/index.html" aria-label="Sign In">',
      '        <svg viewBox="0 0 24 24" width="20" height="20" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round">',
      '          <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"></path>',
      '          <circle cx="12" cy="7" r="4"></circle>',
      '        </svg>',
      '      </a>',
      '    </div>',
      '  </div>',
      '</div>'
    ].join('');
  }

  function mountSharedNav() {
    if (!shouldMountSharedNav()) {
      return;
    }

    if (!document.body || document.body.dataset.dwSharedNavMounted === '1') {
      return;
    }

    ensureSharedNavStyles();

    const existingHeader = document.querySelector('nav.navbar, header.page-header');
    const nav = document.createElement('nav');
    nav.className = 'navbar';
    nav.innerHTML = buildSharedNav();

    if (existingHeader) {
      existingHeader.replaceWith(nav);
    } else {
      document.body.insertBefore(nav, document.body.firstChild);
    }

    const activeHref = getActiveNavHref(window.location.pathname);
    if (activeHref) {
      const activeLink = nav.querySelector('.nav-links a[href="' + activeHref + '"]');
      if (activeLink) {
        activeLink.setAttribute('aria-current', 'page');
      }
    }

    document.body.classList.add('dw-shared-nav');
    document.body.dataset.dwSharedNavMounted = '1';
  }

  applyTheme();

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', function () {
      scheduleNextThemeSwitch();
      mountSharedNav();
    });
  } else {
    scheduleNextThemeSwitch();
    mountSharedNav();
  }
})();

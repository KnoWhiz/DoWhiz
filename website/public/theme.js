(function () {
  const DAY_START_HOUR = 7;
  const NIGHT_START_HOUR = 19;
  const EN_ORIGIN = 'https://www.dowhiz.com';
  const EN_ORIGIN_ALT = 'https://dowhiz.com';
  const CN_ORIGIN = 'https://www.cn.dowhiz.com';
  const CN_HOSTS = new Set(['www.cn.dowhiz.com', 'cn.dowhiz.com']);
  const LOCALE_OVERRIDE_VALUES = new Set(['zh', 'zh-cn', 'cn']);
  const NAV_LABELS = {
    en: {
      home: 'DoWhiz homepage',
      team: 'Team',
      how: 'How it works',
      workflows: 'Workflows',
      safety: 'Safety',
      features: 'Features',
      deployment: 'Deployment',
      faq: 'FAQ',
      blog: 'Blog',
      contact: 'Contact',
      signIn: 'Sign In'
    },
    zh: {
      home: 'DoWhiz 首页',
      team: '团队',
      how: '工作方式',
      workflows: '工作流',
      safety: '安全',
      features: '功能',
      deployment: '部署',
      faq: '常见问题',
      blog: '博客',
      contact: '联系',
      signIn: '登录'
    }
  };

  let observer = null;
  let translationsPromise = null;
  let isApplyingLocalization = false;

  function normalizeText(value) {
    return (value || '').replace(/\s+/g, ' ').trim();
  }

  function sanitizeLocalizedText(value) {
    return (value || '')
      .replaceAll('多威兹', 'DoWhiz')
      .replaceAll('多惠兹', 'DoWhiz')
      .replaceAll('多维兹', 'DoWhiz')
      .replaceAll('多奇才', 'DoWhiz')
      .replaceAll('Open Claw', 'OpenClaw');
  }

  function getLocale() {
    if (typeof window === 'undefined') {
      return 'en';
    }

    const override = new URLSearchParams(window.location.search).get('dwLocale');
    if (override && LOCALE_OVERRIDE_VALUES.has(override.toLowerCase())) {
      return 'zh-CN';
    }

    return CN_HOSTS.has(window.location.host) ? 'zh-CN' : 'en';
  }

  function isChineseLocale() {
    return getLocale() === 'zh-CN';
  }

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
    const labels = isChineseLocale() ? NAV_LABELS.zh : NAV_LABELS.en;

    return [
      '<div class="nav-content">',
      '  <a href="/" class="logo" aria-label="' + labels.home + '">',
      '    <img src="/assets/DoWhiz.jpeg" alt="" class="brand-mark" aria-hidden="true" />',
      '    <span>Do<span class="text-gradient">Whiz</span></span>',
      '  </a>',
      '  <div class="nav-links">',
      '    <a href="/#roles" class="nav-btn">' + labels.team + '</a>',
      '    <a href="/#how-it-works" class="nav-btn">' + labels.how + '</a>',
      '    <a href="/#workflows" class="nav-btn">' + labels.workflows + '</a>',
      '    <a href="/#safety" class="nav-btn">' + labels.safety + '</a>',
      '    <a href="/#features" class="nav-btn">' + labels.features + '</a>',
      '    <a href="/#deployment" class="nav-btn">' + labels.deployment + '</a>',
      '    <a href="/#faq" class="nav-btn">' + labels.faq + '</a>',
      '    <a href="/#blog" class="nav-btn">' + labels.blog + '</a>',
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
      '      <a class="btn-small" href="mailto:admin@dowhiz.com" aria-label="' + labels.contact + '">',
      '        <svg viewBox="0 0 24 24" width="20" height="20" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round">',
      '          <path d="M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z"></path>',
      '          <polyline points="22,6 12,13 2,6"></polyline>',
      '        </svg>',
      '      </a>',
      '      <a class="btn-small" href="/auth/index.html" aria-label="' + labels.signIn + '">',
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

  function translateString(raw, translations) {
    const normalized = normalizeText(raw);
    if (!normalized) {
      return raw;
    }

    if (Object.prototype.hasOwnProperty.call(translations, normalized)) {
      return sanitizeLocalizedText(translations[normalized]);
    }

    return raw;
  }

  function localizeTextNodes(root, translations) {
    if (!root) {
      return;
    }

    const walker = document.createTreeWalker(root, NodeFilter.SHOW_TEXT);
    let node = walker.nextNode();

    while (node) {
      const parent = node.parentElement;
      const raw = node.textContent;
      const normalized = normalizeText(raw);

      if (
        parent &&
        parent.tagName !== 'SCRIPT' &&
        parent.tagName !== 'STYLE' &&
        parent.tagName !== 'SVG' &&
        parent.tagName !== 'PATH' &&
        normalized &&
        Object.prototype.hasOwnProperty.call(translations, normalized)
      ) {
        const leading = (raw.match(/^\s*/) || [''])[0];
        const trailing = (raw.match(/\s*$/) || [''])[0];
        node.textContent = leading + translations[normalized] + trailing;
      }

      node = walker.nextNode();
    }
  }

  function localizeAttributes(root, translations) {
    if (!root || !root.querySelectorAll) {
      return;
    }

    const attrs = ['placeholder', 'title', 'aria-label', 'alt', 'value'];
    const nodes = [root].concat(Array.from(root.querySelectorAll('*')));

    nodes.forEach(function (node) {
      if (!node.getAttribute) {
        return;
      }

      attrs.forEach(function (attr) {
        const raw = node.getAttribute(attr);
        const translated = translateString(raw, translations);
        if (raw && translated !== raw) {
          node.setAttribute(attr, translated);
        }
      });
    });
  }

  function rewriteInternalLinks(root) {
    if (!root || !root.querySelectorAll) {
      return;
    }

    const origin = CN_ORIGIN;
    const nodes = [root].concat(Array.from(root.querySelectorAll('a[href], link[href]')));

    nodes.forEach(function (node) {
      if (!node.getAttribute) {
        return;
      }

      const href = node.getAttribute('href');
      if (!href) {
        return;
      }

      if (href.startsWith(EN_ORIGIN_ALT)) {
        node.setAttribute('href', origin + href.slice(EN_ORIGIN_ALT.length));
      } else if (href.startsWith(EN_ORIGIN)) {
        node.setAttribute('href', origin + href.slice(EN_ORIGIN.length));
      }
    });
  }

  function localizeJsonLd(translations) {
    const scripts = document.querySelectorAll('script[type="application/ld+json"]');

    function translateValue(value) {
      if (typeof value === 'string') {
        const localized = translateString(value, translations);
        return localized
          .replaceAll(EN_ORIGIN_ALT, CN_ORIGIN)
          .replaceAll(EN_ORIGIN, CN_ORIGIN)
          .replaceAll('en_US', 'zh_CN')
          .replaceAll('English', 'Chinese');
      }

      if (Array.isArray(value)) {
        return value.map(translateValue);
      }

      if (value && typeof value === 'object') {
        const next = {};
        Object.keys(value).forEach(function (key) {
          next[key] = translateValue(value[key]);
        });
        return next;
      }

      return value;
    }

    scripts.forEach(function (script) {
      try {
        const payload = JSON.parse(script.textContent);
        script.textContent = JSON.stringify(translateValue(payload));
      } catch (error) {
        // Ignore malformed inline JSON-LD blocks.
      }
    });
  }

  function ensureAlternateLink(hreflang, href) {
    const selector = 'link[rel="alternate"][hreflang="' + hreflang + '"]';
    let link = document.querySelector(selector);

    if (!link) {
      link = document.createElement('link');
      link.rel = 'alternate';
      link.hreflang = hreflang;
      document.head.appendChild(link);
    }

    link.href = href;
  }

  function localizeHead(translations) {
    document.documentElement.lang = 'zh-CN';
    document.documentElement.setAttribute('data-locale', 'zh-CN');

    const pathname = window.location.pathname;
    const search = window.location.search;
    const canonicalHref = CN_ORIGIN + pathname;

    if (document.title) {
      const translatedTitle = translateString(document.title, translations);
      if (translatedTitle !== document.title) {
        document.title = translatedTitle;
      }
    }

    const metaSelectors = [
      'meta[name="description"]',
      'meta[property="og:title"]',
      'meta[property="og:description"]',
      'meta[name="twitter:title"]',
      'meta[name="twitter:description"]',
      'meta[property="og:image:alt"]'
    ];

    metaSelectors.forEach(function (selector) {
      const element = document.querySelector(selector);
      if (!element) {
        return;
      }

      const content = element.getAttribute('content');
      const translated = translateString(content, translations);
      if (content && translated !== content) {
        element.setAttribute('content', translated);
      }
    });

    const canonical = document.querySelector('link[rel="canonical"]');
    if (canonical) {
      canonical.href = canonicalHref;
    }

    const ogUrl = document.querySelector('meta[property="og:url"]');
    if (ogUrl) {
      ogUrl.setAttribute('content', CN_ORIGIN + pathname + search);
    }

    const ogLocale = document.querySelector('meta[property="og:locale"]');
    if (ogLocale) {
      ogLocale.setAttribute('content', 'zh_CN');
    }

    ensureAlternateLink('zh-CN', canonicalHref);
    ensureAlternateLink('en', EN_ORIGIN + pathname);
    ensureAlternateLink('x-default', EN_ORIGIN + pathname);
  }

  function applyLocalization(translations) {
    if (isApplyingLocalization) {
      return;
    }

    isApplyingLocalization = true;
    try {
      localizeHead(translations);
      rewriteInternalLinks(document);
      localizeTextNodes(document.body, translations);
      localizeAttributes(document.body, translations);
      localizeJsonLd(translations);
    } finally {
      isApplyingLocalization = false;
    }
  }

  function watchForMutations(translations) {
    if (observer) {
      observer.disconnect();
    }

    let scheduled = false;
    const scheduleApply = function () {
      if (scheduled) {
        return;
      }
      scheduled = true;
      window.requestAnimationFrame(function () {
        scheduled = false;
        applyLocalization(translations);
      });
    };

    observer = new MutationObserver(function () {
      scheduleApply();
    });

    observer.observe(document.documentElement, {
      childList: true,
      subtree: true,
      characterData: true
    });
  }

  function loadTranslations() {
    if (!translationsPromise) {
      translationsPromise = fetch('/cn-translations.json')
        .then(function (response) {
          if (!response.ok) {
            throw new Error('Failed to load cn translations.');
          }
          return response.json();
        })
        .then(function (payload) {
          const rawTranslations = payload && payload.translations ? payload.translations : {};
          const sanitized = {};
          Object.keys(rawTranslations).forEach(function (key) {
            sanitized[key] = sanitizeLocalizedText(rawTranslations[key]);
          });
          return sanitized;
        })
        .catch(function () {
          return {};
        });
    }

    return translationsPromise;
  }

  function startCnLocalization() {
    if (!isChineseLocale()) {
      return;
    }

    document.documentElement.lang = 'zh-CN';
    document.documentElement.setAttribute('data-locale', 'zh-CN');

    loadTranslations().then(function (translations) {
      applyLocalization(translations);
      watchForMutations(translations);
    });
  }

  applyTheme();

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', function () {
      scheduleNextThemeSwitch();
      mountSharedNav();
      startCnLocalization();
    });
  } else {
    scheduleNextThemeSwitch();
    mountSharedNav();
    startCnLocalization();
  }
})();

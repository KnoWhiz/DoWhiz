const ANON_ID_KEY = 'dowhiz_analytics_anonymous_id';
const SESSION_ID_KEY = 'dowhiz_analytics_session_id';
const ATTRIBUTION_KEY = 'dowhiz_analytics_attribution';
const WEBSITE_VERSION = 'website-analytics-v1';

function safeStorage(kind) {
  if (typeof window === 'undefined') {
    return null;
  }
  try {
    return kind === 'session' ? window.sessionStorage : window.localStorage;
  } catch {
    return null;
  }
}

function generateId(prefix) {
  if (typeof crypto !== 'undefined' && typeof crypto.randomUUID === 'function') {
    return `${prefix}_${crypto.randomUUID()}`;
  }
  return `${prefix}_${Date.now().toString(36)}_${Math.random().toString(36).slice(2, 11)}`;
}

export function getOrCreateAnonymousId() {
  const storage = safeStorage('local');
  if (!storage) {
    return generateId('anon');
  }
  const existing = storage.getItem(ANON_ID_KEY);
  if (existing) {
    return existing;
  }
  const created = generateId('anon');
  storage.setItem(ANON_ID_KEY, created);
  return created;
}

export function getOrCreateSessionId() {
  const session = safeStorage('session');
  if (session) {
    const existing = session.getItem(SESSION_ID_KEY);
    if (existing) {
      return existing;
    }
    const created = generateId('sess');
    session.setItem(SESSION_ID_KEY, created);
    return created;
  }

  const local = safeStorage('local');
  if (local) {
    const fallback = local.getItem(SESSION_ID_KEY);
    if (fallback) {
      return fallback;
    }
    const created = generateId('sess');
    local.setItem(SESSION_ID_KEY, created);
    return created;
  }

  return generateId('sess');
}

function normalizeOptional(value) {
  if (value === undefined || value === null) {
    return null;
  }
  const trimmed = String(value).trim();
  return trimmed.length > 0 ? trimmed : null;
}

function detectEnvironment() {
  if (typeof window === 'undefined') {
    return 'production';
  }
  const host = window.location.hostname.toLowerCase();
  if (host === 'localhost' || host.endsWith('.local')) {
    return 'local';
  }
  if (host.includes('staging') || host.includes('dowhizstaging')) {
    return 'staging';
  }
  return 'production';
}

export function getDoWhizApiBaseUrl() {
  if (typeof window === 'undefined') {
    return 'https://api.production1.dowhiz.com/service';
  }
  const host = window.location.hostname.toLowerCase();
  if (host === 'localhost') {
    return 'http://localhost:9001';
  }
  if (host.includes('staging') || host.includes('dowhizstaging')) {
    return 'https://api.staging.dowhiz.com/service';
  }
  return 'https://api.production1.dowhiz.com/service';
}

function parseAttributionFromLocation() {
  if (typeof window === 'undefined') {
    return null;
  }
  const params = new URLSearchParams(window.location.search);
  const parsed = {
    utm_source: normalizeOptional(params.get('utm_source')),
    utm_medium: normalizeOptional(params.get('utm_medium')),
    utm_campaign: normalizeOptional(params.get('utm_campaign')),
    utm_term: normalizeOptional(params.get('utm_term')),
    utm_content: normalizeOptional(params.get('utm_content')),
    captured_at: new Date().toISOString()
  };
  const hasAnyUtm =
    parsed.utm_source || parsed.utm_medium || parsed.utm_campaign || parsed.utm_term || parsed.utm_content;
  return hasAnyUtm ? parsed : null;
}

function loadStoredAttribution() {
  const storage = safeStorage('local');
  if (!storage) {
    return null;
  }
  const raw = storage.getItem(ATTRIBUTION_KEY);
  if (!raw) {
    return null;
  }
  try {
    return JSON.parse(raw);
  } catch {
    return null;
  }
}

export function persistAttributionFromLocation() {
  const storage = safeStorage('local');
  if (!storage) {
    return null;
  }
  const parsed = parseAttributionFromLocation();
  if (!parsed) {
    return loadStoredAttribution();
  }

  const existing = loadStoredAttribution();
  const merged = {
    utm_source: parsed.utm_source ?? existing?.utm_source ?? null,
    utm_medium: parsed.utm_medium ?? existing?.utm_medium ?? null,
    utm_campaign: parsed.utm_campaign ?? existing?.utm_campaign ?? null,
    utm_term: parsed.utm_term ?? existing?.utm_term ?? null,
    utm_content: parsed.utm_content ?? existing?.utm_content ?? null,
    captured_at: parsed.captured_at
  };
  storage.setItem(ATTRIBUTION_KEY, JSON.stringify(merged));
  return merged;
}

function inferDeviceType(ua) {
  if (!ua) return 'unknown';
  if (/mobile|iphone|ipod|android.*mobile|windows phone/i.test(ua)) return 'mobile';
  if (/ipad|tablet|android(?!.*mobile)/i.test(ua)) return 'tablet';
  return 'desktop';
}

function inferBrowser(ua) {
  if (!ua) return 'unknown';
  if (/edg\//i.test(ua)) return 'edge';
  if (/chrome\//i.test(ua) && !/edg\//i.test(ua)) return 'chrome';
  if (/safari\//i.test(ua) && !/chrome\//i.test(ua)) return 'safari';
  if (/firefox\//i.test(ua)) return 'firefox';
  if (/opr\//i.test(ua) || /opera\//i.test(ua)) return 'opera';
  return 'unknown';
}

function inferOs(ua) {
  if (!ua) return 'unknown';
  if (/windows/i.test(ua)) return 'windows';
  if (/macintosh|mac os x/i.test(ua)) return 'macos';
  if (/android/i.test(ua)) return 'android';
  if (/iphone|ipad|ios/i.test(ua)) return 'ios';
  if (/linux/i.test(ua)) return 'linux';
  return 'unknown';
}

function currentPagePath() {
  if (typeof window === 'undefined') {
    return '/';
  }
  return `${window.location.pathname}${window.location.search}`;
}

export function currentAnalyticsContext() {
  const attribution = persistAttributionFromLocation() || loadStoredAttribution() || {};
  const userAgent = typeof navigator !== 'undefined' ? navigator.userAgent || '' : '';

  return {
    anonymous_id: getOrCreateAnonymousId(),
    session_id: getOrCreateSessionId(),
    environment: detectEnvironment(),
    app_version: WEBSITE_VERSION,
    page_path: currentPagePath(),
    route_path: typeof window !== 'undefined' ? window.location.pathname : '/',
    referrer:
      typeof document !== 'undefined' && typeof document.referrer === 'string' && document.referrer
        ? document.referrer
        : null,
    utm_source: attribution.utm_source || null,
    utm_medium: attribution.utm_medium || null,
    utm_campaign: attribution.utm_campaign || null,
    utm_term: attribution.utm_term || null,
    utm_content: attribution.utm_content || null,
    device_type: inferDeviceType(userAgent),
    browser: inferBrowser(userAgent),
    os: inferOs(userAgent)
  };
}

export async function trackAnalyticsEvent(eventName, properties = {}, options = {}) {
  const normalizedEventName = normalizeOptional(eventName);
  if (!normalizedEventName || typeof window === 'undefined') {
    return false;
  }

  const payload = {
    event_name: normalizedEventName,
    event_timestamp: new Date().toISOString(),
    source: options.source || 'client',
    ...currentAnalyticsContext(),
    event_key: normalizeOptional(options.eventKey),
    properties
  };

  if (options.pagePath) {
    payload.page_path = options.pagePath;
  }
  if (options.routePath) {
    payload.route_path = options.routePath;
  }

  const headers = {
    'Content-Type': 'application/json'
  };
  if (options.accessToken) {
    headers.Authorization = `Bearer ${options.accessToken}`;
  }

  try {
    const res = await fetch(`${getDoWhizApiBaseUrl()}/analytics/track`, {
      method: 'POST',
      headers,
      body: JSON.stringify(payload),
      keepalive: true
    });
    return res.ok;
  } catch (error) {
    console.warn(`analytics track failed for ${normalizedEventName}:`, error);
    return false;
  }
}

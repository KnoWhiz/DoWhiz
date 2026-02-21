// Time-based theme switching (7 AM - 7 PM = light, 7 PM - 7 AM = dark)
(function() {
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
    setTimeout(function() {
      applyTheme();
      scheduleNextThemeSwitch();
    }, delay);
  }

  // Apply theme immediately
  applyTheme();

  // Schedule next switch when DOM is ready
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', scheduleNextThemeSwitch);
  } else {
    scheduleNextThemeSwitch();
  }
})();

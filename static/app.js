(() => {
  const reduce = window.matchMedia('(prefers-reduced-motion: reduce)').matches;

  // ── Tema claro / oscuro ───────────────────────────────────────────────────
  const root = document.documentElement;
  const toggle = document.querySelector('[data-theme-toggle]');

  const applyTheme = (dark) => {
    root.classList.toggle('dark', dark);
    if (!toggle) return;
    toggle.setAttribute('aria-pressed', String(dark));
    toggle.setAttribute('aria-label', dark ? 'Activar modo claro' : 'Activar modo oscuro');
  };

  applyTheme(root.classList.contains('dark'));

  if (toggle) {
    toggle.addEventListener('click', () => {
      const dark = !root.classList.contains('dark');
      applyTheme(dark);
      try {
        localStorage.setItem('theme', dark ? 'dark' : 'light');
      } catch (e) {}
      if (!reduce) {
        root.classList.add('theme-anim');
        window.setTimeout(() => root.classList.remove('theme-anim'), 420);
      }
    });
  }

  // ── Revelado suave al entrar en viewport ─────────────────────────────────
  const items = document.querySelectorAll('[data-reveal]');
  if (!reduce && 'IntersectionObserver' in window) {
    const io = new IntersectionObserver(
      (entries) => {
        entries.forEach((entry, index) => {
          if (!entry.isIntersecting) return;
          entry.target.style.transitionDelay = `${Math.min(index, 6) * 70}ms`;
          entry.target.classList.add('is-visible');
          io.unobserve(entry.target);
        });
      },
      { threshold: 0.12, rootMargin: '0px 0px -6% 0px' }
    );
    items.forEach((el) => io.observe(el));
  } else {
    items.forEach((el) => el.classList.add('is-visible'));
  }

  // ── Luz que sigue al cursor en tarjetas ──────────────────────────────────
  if (!reduce && window.matchMedia('(hover: hover)').matches) {
    document.querySelectorAll('.spotlight').forEach((card) => {
      card.addEventListener('pointermove', (event) => {
        const rect = card.getBoundingClientRect();
        card.style.setProperty('--mx', `${event.clientX - rect.left}px`);
        card.style.setProperty('--my', `${event.clientY - rect.top}px`);
      });
    });
  }

  // ── Progreso de lectura y header ─────────────────────────────────────────
  const progress = document.getElementById('progress');
  const header = document.querySelector('[data-header]');
  let ticking = false;

  const onScroll = () => {
    if (ticking) return;
    ticking = true;
    window.requestAnimationFrame(() => {
      const max = document.documentElement.scrollHeight - window.innerHeight;
      const ratio = max > 0 ? window.scrollY / max : 0;
      if (progress) progress.style.width = `${Math.min(ratio * 100, 100)}%`;
      if (header) header.classList.toggle('is-scrolled', window.scrollY > 8);
      ticking = false;
    });
  };

  window.addEventListener('scroll', onScroll, { passive: true });
  onScroll();

  // El menú móvil se cierra al elegir un destino.
  const menu = document.querySelector('header details');
  if (menu) {
    menu.querySelectorAll('a').forEach((link) => {
      link.addEventListener('click', () => menu.removeAttribute('open'));
    });
  }
})();

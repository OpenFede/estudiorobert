(() => {
  const reduce = window.matchMedia('(prefers-reduced-motion: reduce)').matches;

  // Revelado suave al entrar en viewport.
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

  // Barra de progreso de lectura y header con sombra.
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

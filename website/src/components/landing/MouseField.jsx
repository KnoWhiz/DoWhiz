import { useEffect, useRef } from 'react';

const lerp = (start, end, t) => start + (end - start) * t;
const clamp = (value, min, max) => Math.min(max, Math.max(min, value));

const palettes = {
  dark: [
    { r: 58, g: 58, b: 60 },
    { r: 99, g: 99, b: 102 },
    { r: 142, g: 142, b: 147 }
  ],
  light: [
    { r: 58, g: 58, b: 60 },
    { r: 99, g: 99, b: 102 },
    { r: 142, g: 142, b: 147 }
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

export default MouseField;

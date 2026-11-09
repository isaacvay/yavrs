/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.{rs,html,yav,rsx,yavml}",
    "./public/**/*.{html,js}",
  ],
  theme: {
    extend: {
      fontFamily: {
        sans: ['Geist', 'system-ui', '-apple-system', 'sans-serif'],
        mono: ['Geist Mono', 'ui-monospace', 'monospace'],
        serif: ['Instrument Serif', 'Georgia', 'serif'],
      },
      colors: {
        obsidian: {
          950: '#050507',
          900: '#08080b',
          850: '#0c0d11',
          800: '#111217',
          750: '#17181f',
          700: '#1e2029',
          600: '#2b2d39',
        },
        ember: {
          50: '#fff6f2',
          100: '#ffe8de',
          200: '#ffd0be',
          300: '#ffa98d',
          400: '#ff7750',
          500: '#ff5c2b',
          600: '#f04313',
          700: '#c7300a',
          800: '#9e270d',
          900: '#7e2410',
        },
        phosphor: {
          400: '#4ade80',
          500: '#22c55e',
          600: '#16a34a',
        },
        surface: {
          0: '#060608',
          1: '#0b0c0f',
          2: '#111216',
          3: '#181920',
          4: '#20222a',
        },
      },
      borderColor: {
        hairline: 'rgba(255, 255, 255, 0.06)',
        subtle: 'rgba(255, 255, 255, 0.08)',
        medium: 'rgba(255, 255, 255, 0.12)',
        strong: 'rgba(255, 255, 255, 0.18)',
        ember: 'rgba(255, 92, 43, 0.3)',
        rule: 'var(--rule)',
        'rule-2': 'var(--rule-2)',
        'rule-3': 'var(--rule-3)',
        oxide: 'var(--oxide)',
      },
      boxShadow: {
        'specular': 'inset 0 1px 0 rgba(255, 255, 255, 0.09), 0 20px 40px -15px rgba(0, 0, 0, 0.7)',
        'specular-ember': 'inset 0 1px 0 rgba(255, 140, 95, 0.2), 0 20px 40px -15px rgba(255, 92, 43, 0.15)',
        'glow-ember': '0 0 35px -5px rgba(255, 92, 43, 0.25)',
      },
    },
  },
  plugins: [],
}


/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./index.html", "./src/*.rs", "./src/**/*.rs", "./src/**/**/*.rs", "styles/**.css"],
  theme: {
    extend: {animation: {
      'logo-spin': 'logo-spin 20s linear infinite',
      'typing': 'typing 3s steps(40, end)',
      'blink-caret': 'blink-caret 0.75s step-end infinite',
      'flash': 'flash 1.5s cubic-bezier(0.4, 0, 0.6, 1)'
    },
    keyframes: {
      'logo-spin': {
        from: { transform: 'rotate(0deg)' },
        to: { transform: 'rotate(360deg)' },
        
      },
      'typing': {
        from: { width: '0' },
        to: { width: '100%' }
    },
    'blink-caret': {
        'from, to': { 'border-color': 'transparent' },
        '50%': { 'border-color': 'currentColor' }
    },
    'flash': {
        '0%': { opacity: '1', transform: 'scale(1)' },
        '50%': { opacity: '0.8', transform: 'scale(1.2)' },
        '100%': { opacity: '0', transform: 'scale(1.5)' }
    }
    }},
  },
  plugins: [],
}

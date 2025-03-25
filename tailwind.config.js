/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./index.html", "./src/*.rs", "./src/**/*.rs", "./src/**/**/*.rs", "styles/**.css"],
  theme: {
    extend: {animation: {
      'logo-spin': 'logo-spin 20s linear infinite',
    },
    keyframes: {
      'logo-spin': {
        from: { transform: 'rotate(0deg)' },
        to: { transform: 'rotate(360deg)' },
      }
    }},
  },
  plugins: [],
}

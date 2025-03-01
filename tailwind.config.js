/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./index.html", "./src/*.rs", "./src/**/*.rs", "./src/**/**/*.rs", "styles/**.css"],
  theme: {
    extend: {},
  },
  plugins: [],
}

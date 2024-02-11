module.exports = {
  content: [
    './src/**/*.rs',
    './index.html',
    './src/**/*.html',
    './src/**/*.css'
  ],
  safelist: [
    {
      pattern: /bg-.*-(100|200|300|500|600|700|900)/,
    },
    {
      pattern: /border-.*-(100|200|300|500|600|700|900)/,
    },
    {
      pattern: /hover:bg-.*-(100|200|300|500|600|700|900)/,
    },
  ],
  theme: {
    extend: {},
  },
  plugins: [
    require("@tailwindcss/typography"),
  ],
}


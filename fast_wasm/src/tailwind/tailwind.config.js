/** @type {import('frontend/tailwind/tailwindcss').Config} */
module.exports = {
  content: [
    '../frontend/src/*.rs',
    '../frontend/src/**/*.rs',
  ],
  theme: {
    extend: {},
  },
  plugins: [
    require('@tailwindcss/forms'),
    require('@tailwindcss/typography'),
  ]
}

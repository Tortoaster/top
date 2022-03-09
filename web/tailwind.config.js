const plugin = require('tailwindcss/plugin')

module.exports = {
  content: ["./src/**/*.hbs"],
  theme: {
    extend: {},
  },
  plugins: [
    plugin(function ({ addVariant }) {
      addVariant('syncing', '&[syncing]')
      addVariant('synced', '&[synced]')
      addVariant('failed', ['&[failed]', '&:invalid'])
    })
  ],
}

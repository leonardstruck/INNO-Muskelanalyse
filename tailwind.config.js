/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.html", "./src/components/**/*.{js,jsx,ts,tsx}", "./src/pages/**/*.{js,jsx,ts,tsx}"],
  theme: {
    extend: {
      fontFamily: {
        display: ["var(--font-teko)"],
        sans: ["var(--font-rubik)"],
      },
      backgroundImage: {
        "myotube": "url('assets/myotube.png')",
      },
      colors: {
        "dark-blue": "#06091A"
      }
    },
  },
  plugins: [
    require('@tailwindcss/forms'),
    require('@tailwindcss/aspect-ratio'),
  ],
}

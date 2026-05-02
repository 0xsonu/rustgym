/** @type {import('tailwindcss').Config} */
export default {
  content: ['./index.html', './src/**/*.{js,ts,jsx,tsx}'],
  theme: {
    extend: {
      colors: {
        primary: {
          DEFAULT: '#CE422B',
          50: '#F9E8E5',
          100: '#F4D1CC',
          200: '#E9A399',
          300: '#DE7566',
          400: '#D45A45',
          500: '#CE422B',
          600: '#A53522',
          700: '#7C281A',
          800: '#531B11',
          900: '#2A0D09',
          light: '#E05A40',
          dark: '#A8341F',
        },
        amber: {
          DEFAULT: '#E8913A',
        },
        dark: {
          950: '#0B0B0A',
          900: '#111110',
          800: '#1A1917',
          700: '#242320',
          card: '#1E1D1B',
        },
        border: {
          DEFAULT: '#2E2D2A',
          light: '#3A3936',
        },
        text: {
          primary: '#F0EDE8',
          secondary: '#B8B3AC',
          muted: '#756F68',
        },
        green: {
          DEFAULT: '#4ADE80',
        },
        blue: {
          DEFAULT: '#60A5FA',
        },
      },
      fontFamily: {
        code: ['JetBrains Mono', 'monospace'],
        display: ['Barlow Condensed', 'sans-serif'],
        body: ['Barlow', 'sans-serif'],
      },
    },
  },
  plugins: [],
};

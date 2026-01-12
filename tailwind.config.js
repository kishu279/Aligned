/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./app/**/*.{js,jsx,ts,tsx}", "./components/**/*.{js,jsx,ts,tsx}"],
  presets: [require("nativewind/preset")],
  theme: {
    extend: {
      fontFamily: {
        sans: ["NunitoSans", "system-ui", "sans-serif"],
        nunito: ["NunitoSans", "system-ui", "sans-serif"],
      },
      colors: {
        // Main backgrounds
        primary: "#FFFFFF",
        secondary: "#F5F5F5",

        // Text colors
        text: {
          primary: "#000000",
          secondary: "#4A4A4A",
          muted: "#9CA3AF",
        },

        // UI colors
        gray: {
          50: "#FAFAFA",
          100: "#F5F5F5",
          200: "#E5E5E5",
          300: "#D4D4D4",
          400: "#A3A3A3",
          500: "#737373",
          600: "#525252",
          700: "#404040",
          800: "#262626",
          900: "#171717",
        },

        // Accent colors
        accent: "#3B82F6",
        rose: "#FBE8E7",

        // Tab bar
        tab: {
          bg: "#FFFFFF",
          border: "#E5E5E5",
          icon: "#525252",
          iconActive: "#FFFFFF",
        }
      },
    },
  },
  plugins: [],
};
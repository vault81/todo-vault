/** @type {import('tailwindcss').Config} */
const defaultTheme = require("tailwindcss/defaultTheme");
module.exports = {
  content: [
    "./app/src/**/*.rs",
    "./public/*.html",
    "./node_modules/flowbite/**/*.js",
  ],
  safelist: [
    "shadow-lg",
    "backdrop-blur-sm",
    "rounded-l-lg",
    "rounded-r-lg",
    "bg-gray-200",
    "grid-cols-4",
    "grid-cols-7",
    "w-64",
    "w-1/2",
    "h-6",
    "h-9",
    "leading-6",
    "leading-9",
  ],
  theme: {
    extend: {
      animation: {
        show: "show 150ms 0ms ease-in-out both",
        "intro-1": "intro 250ms 50ms ease-in-out both",
        "intro-2": "intro 250ms 150ms ease-in-out both",
      },
      keyframes: {
        show: {
          "100%": { opacity: 1, transform: "none" },
        },
        intro: {
          "0%": { opacity: 0.0 },
          "100%": { opacity: 1 },
        },
      },
      invert: {
        85: ".85",
      },
    },
  },
  variants: {
    scale: ["responsive", "hover", "focus", "group-hover"],
    textColor: ["responsive", "hover", "focus", "group-hover"],
    opacity: ["responsive", "hover", "focus", "group-hover"],
    backgroundColor: ["responsive", "hover", "focus", "group-hover"],
  },
  darkMode: "media",
  corePlugins: {
    aspectRatio: false,
  },
  plugins: [
    require("@tailwindcss/forms"),
    require("flowbite/plugin"),
    require("flowbite-typography"),
  ],
};

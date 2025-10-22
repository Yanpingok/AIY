// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

const defaultTheme = require("tailwindcss/defaultTheme");

module.exports = {
  corePlugins: {
    preflight: false, // disable Tailwind's reset
  },
  content: ["./src/**/*.{js,jsx,ts,tsx}", "./docs/**/*.mdx"], // my markdown stuff is in ../docs, not /src
  safelist: ["text-aiy-success-dark"],
  darkMode: ["class", '[data-theme="dark"]'], // hooks into docusaurus' dark mode settings
  theme: {
    extend: {
      fontFamily: {
        sans: ["Inter", ...defaultTheme.fontFamily.sans],
        twkeverett: ["Twkeverett"],
      },
      colors: {
        "aiy-black": "var(--aiy-black)",
        "aiy-blue-primary": "rgb(var(--aiy-blue-primary)/<alpha-value>)",
        "aiy-blue": "var(--aiy-blue)",
        "aiy-blue-bright": "rgb(var(--aiy-blue-bright)/<alpha-value>)",
        "aiy-blue-light": "rgb(var(--aiy-blue-light)/<alpha-value>)",
        "aiy-blue-lighter": "var(--aiy-blue-lighter)",
        "aiy-blue-dark": "rgb(var(--aiy-blue-dark)/<alpha-value>)",
        "aiy-blue-darker": "var(--aiy-blue-darker)",
        "aiy-hero": "var(--aiy-hero)",
        "aiy-hero-dark": "var(--aiy-hero-dark)",
        "aiy-steel": "var(--aiy-steel)",
        "aiy-steel-dark": "var(--aiy-steel-dark)",
        "aiy-steel-darker": "var(--aiy-steel-darker)",
        "aiy-header-nav": "var(--aiy-header-nav)",
        "aiy-success": "var(--aiy-success)",
        "aiy-success-dark": "var(--aiy-success-dark)",
        "aiy-success-light": "var(--aiy-success-light)",
        "aiy-issue": "var(--aiy-issue)",
        "aiy-issue-dark": "var(--aiy-issue-dark)",
        "aiy-issue-light": "var(--aiy-issue-light)",
        "aiy-warning": "var(--aiy-warning)",
        "aiy-warning-dark": "var(--aiy-warning-dark)",
        "aiy-warning-light": "var(--aiy-warning-light)",
        "aiy-code": "var(--aiy-code)",
        "aiy-gray-3s": "rgb(var(--aiy-gray-3s)/<alpha-value>)",
        "aiy-gray-5s": "rgb(var(--aiy-gray-5s)/<alpha-value>)",
        "aiy-gray": {
          35: "rgb(var(--aiy-gray-35)/<alpha-value>)",
          40: "rgb(var(--aiy-gray-40)/<alpha-value>)",
          45: "rgb(var(--aiy-gray-45)/<alpha-value>)",
          50: "var(--aiy-gray-50)",
          55: "rgb(var(--aiy-gray-55)/<alpha-value>)",
          60: "var(--aiy-gray-60)",
          65: "var(--aiy-gray-65)",
          70: "var(--aiy-gray-70)",
          75: "var(--aiy-gray-75)",
          80: "var(--aiy-gray-80)",
          85: "var(--aiy-gray-85)",
          90: "var(--aiy-gray-90)",
          95: "var(--aiy-gray-95)",
          100: "var(--aiy-gray-100)",
        },
        "aiy-grey": {
          35: "rgb(var(--aiy-gray-35)/<alpha-value>)",
          40: "rgb(var(--aiy-gray-40)/<alpha-value>)",
          45: "rgb(var(--aiy-gray-45)/<alpha-value>)",
          50: "var(--aiy-gray-50)",
          55: "rgb(var(--aiy-gray-55)/<alpha-value>)",
          60: "var(--aiy-gray-60)",
          65: "var(--aiy-gray-65)",
          70: "var(--aiy-gray-70)",
          75: "var(--aiy-gray-75)",
          80: "var(--aiy-gray-80)",
          85: "var(--aiy-gray-85)",
          90: "var(--aiy-gray-90)",
          95: "var(--aiy-gray-95)",
          100: "var(--aiy-gray-100)",
        },
        "aiy-disabled": "rgb(var(--aiy-disabled)/<alpha-value>)",
        "aiy-link-color-dark": "var(--aiy-link-color-dark)",
        "aiy-link-color-light": "var(--aiy-link-color-light)",
        "aiy-ghost-white": "var(--aiy-ghost-white)",
        "aiy-ghost-dark": "var(--aiy-ghost-dark)",
        "ifm-background-color-dark": "var(--ifm-background-color-dark)",
        "aiy-white": "rgb(var(--aiy-white)/<alpha-value>)",
        "aiy-card-dark": "rgb(var(--aiy-card-dark)/<alpha-value>)",
        "aiy-card-darker": "rgb(var(--aiy-card-darker)/<alpha-value>)",
      },
      borderRadius: {
        aiy: "40px",
      },
      boxShadow: {
        aiy: "0px 0px 4px rgba(0, 0, 0, 0.02)",
        "aiy-button": "0px 1px 2px rgba(16, 24, 40, 0.05)",
        "aiy-notification": "0px 0px 20px rgba(29, 55, 87, 0.11)",
      },
      gradientColorStopPositions: {
        36: "36%",
      },
    },
  },
  plugins: [
    function ({ addUtilities }) {
      const arrowMask = `url("data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'><path d='M8.12 4.12a1 1 0 0 1 1.41 0l6.35 6.35a1 1 0 0 1 0 1.41l-6.35 6.35a1 1 0 1 1-1.41-1.41L13.59 12 8.12 6.53a1 1 0 0 1 0-1.41z'/></svg>") no-repeat center / contain`;

      addUtilities({
        ".mask-arrow": {
          transition: "transform 0.2s ease",
          background: "currentColor",
          WebkitMask: arrowMask,
          mask: arrowMask,
        },
      });
    },
  ],
};

/** @type {import('tailwindcss').Config} */
export default {
	content: {
		relative: true,
		files: [
			"./docs/index.html",
			"./docs/src/**/*.rs",
			"./index.html",
			"./src/**/*.rs",
		],
	},
	theme: {
		extend: {
			colors: {
				border: "hsl(var(--border))",
				input: "hsl(var(--input))",
				ring: "hsl(var(--ring))",
				background: "hsl(var(--background))",
				foreground: "hsl(var(--foreground))",
				primary: {
					DEFAULT: "hsl(var(--primary))",
					foreground: "hsl(var(--primary-foreground))",
				},
				secondary: {
					DEFAULT: "hsl(var(--secondary))",
					foreground: "hsl(var(--secondary-foreground))",
				},
				destructive: {
					DEFAULT: "hsl(var(--destructive))",
					foreground: "hsl(var(--destructive-foreground))",
				},
				muted: {
					DEFAULT: "hsl(var(--muted))",
					foreground: "hsl(var(--muted-foreground))",
				},
				accent: {
					DEFAULT: "hsl(var(--accent))",
					foreground: "hsl(var(--accent-foreground))",
				},
				popover: {
					DEFAULT: "hsl(var(--popover))",
					foreground: "hsl(var(--popover-foreground))",
				},
				card: {
					DEFAULT: "hsl(var(--card))",
					foreground: "hsl(var(--card-foreground))",
				},
			},
			keyframes: {
				"fade-in": {
					from: { opacity: "0" },
					to: { opacity: "1" },
				},
				"fade-in-up": {
					from: { opacity: "0", transform: "translateY(8px)" },
					to: { opacity: "1", transform: "translateY(0)" },
				},
			},
			animation: {
				"fade-in": "fade-in 0.35s ease-out both",
				"fade-in-up": "fade-in-up 0.45s ease-out both",
			},
			borderRadius: {
				lg: "var(--radius)",
				md: "calc(var(--radius) - 2px)",
				sm: "calc(var(--radius) - 4px)",
			},
		},
	},
	darkMode: "class",
	plugins: [],
};

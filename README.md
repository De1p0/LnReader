# Urayomi (lnreader)

A desktop reader application built with React + Vite and bundled with Tauri. This repository contains the UI and Tauri integration for a lightweight, themed desktop app.

## Features

- Single page React app using Vite
- Custom title bar with native window controls (Tauri)
- Left sidebar that expands as an overlay (doesn't push content)
- Backdrop blur when the sidebar is expanded
- Project-wide accent colour applied (`#C8A2C8`)
- Tailwind CSS utilities plus a small global CSS file in `src/App.css`

## Prerequisites

- Node.js (recommended 18+)
- npm (or yarn)
- Rust (for building the Tauri native bundle)
- Tauri CLI (optional; can use `cargo tauri`)

## Install

```bash
# from project root
npm install
```

## Development

Run the frontend dev server (Vite):

```bash
npm run dev
```

To run the full Tauri application in dev mode you can either use the Tauri CLI directly or via npm:

```bash
# using npm wrapper (passes args to tauri)
npm run tauri -- dev

# or via cargo (if you have Rust + tauri-cli installed)
cargo tauri dev
```

## Build

Build the web assets and then create a Tauri release bundle:

```bash
npm run build
# then
npm run tauri -- build
# or
cargo tauri build
```

## Project structure (key files)

- `src/` — React source
  - `src/components/SideBar.tsx` — Sidebar implementation (absolute overlay, expands without pushing content; applies `backdrop-blur` when expanded)
  - `src/components/TitleBar.tsx` — Custom title bar and controls
  - `src/App.css` — Global CSS with theme variables (accent color defined here)
  - `src/App.tsx` — App shell (contains sidebar + main content)
- `src-tauri/` — Tauri configuration and Rust source for the native app
- `package.json` — Scripts and node dependencies

## Theming & Accent Color

The project uses a CSS variable for the accent color. To change the accent, edit the variable in `src/App.css`:

```css
:root {
  /* current accent */
  --color-accent: #C8A2C8;
}
```

Utility classes were added for convenience:

- `.text-accent` — text color
- `.bg-accent` — background color
- `.border-accent` — border color

These can be used anywhere in React components to apply the accent quickly.

## Sidebar behaviour

The sidebar is positioned absolutely inside the app container so that when expanded it overlays the main content rather than pushing it. When expanded it applies:

- `backdrop-blur-md` and a translucent background (`bg-surface/80`) to create a blurred overlay effect

If you need to adjust blur strength or behaviour, edit `src/components/SideBar.tsx` and/or your Tailwind config (if present).

## Notes

- Ensure your Tailwind build includes the `backdrop-blur` utilities (Tailwind >= v2.1 with the `backdropFilter` plugin or native in v3+).
- The repo uses Tauri — make sure Rust and necessary toolchains are installed when building native binaries.

## Contributing

Contributions are welcome. Please open issues or PRs. For local development, follow the steps in the Development section.

## License

This project does not include a license file. Add one if you plan to publish or share the code.

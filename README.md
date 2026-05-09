# ToDoList Desktop

Lightweight and minimal desktop todo app based on Tauri + Vue.

## Features

- Create todos
- Tick to complete / uncomplete
- Delete todos
- Local persistence with SQLite
- Reminder contract reserved (`schedule/cancel/resync`)

## Development

1. Install Node.js and Rust toolchain.
2. Install dependencies:
   - `npm install`
3. Run app:
   - Web: `npm run dev`
   - Desktop: `npm run tauri:dev`

## Build

- `npm run build`
- `npm run tauri:build`

## CI Packaging (No local Windows setup)

This project includes GitHub Actions workflow at `.github/workflows/build-desktop.yml`.

- Trigger manually: GitHub -> Actions -> `Build Desktop Installers` -> `Run workflow`
- Trigger release build: push a tag like `v0.1.0`

Outputs:

- Windows: `.msi` and `.exe` installers
- Linux: `.deb` and `.rpm` installers

For tag builds, files are uploaded to the GitHub Release automatically.

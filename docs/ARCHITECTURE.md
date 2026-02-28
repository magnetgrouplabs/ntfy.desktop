# Architecture Overview

## Technology Stack

- **Frontend**: Svelte + Vite
- **Backend**: Rust + Tauri
- **Build System**: npm scripts + Cargo

## Project Structure

```
ntfy.desktop/
├── src/                 # Svelte frontend source
│   ├── App.svelte      # Main application component
│   ├── main.js         # Application entry point
│   └── index.html      # HTML template
├── src-tauri/          # Rust backend
│   ├── src/main.rs     # Application entry point
│   ├── Cargo.toml      # Rust dependencies
│   └── tauri.conf.json # Tauri configuration
├── performance-tests/   # Performance testing
│   ├── baseline/       # Baseline performance metrics
│   ├── comparison/     # Performance comparisons
│   └── scripts/        # Test automation scripts
└── docs/              # Documentation
```

## Development Workflow

1. Frontend development: Edit files in `src/`
2. Backend development: Edit files in `src-tauri/src/`
3. Build: `npm run build`
4. Development: `npm run tauri:dev`
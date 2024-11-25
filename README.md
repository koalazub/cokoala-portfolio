# CoKoala

Looking to fuck around and learn more WASM and deployment steps for Fermyon

## Tech Stack
- **Leptos**: Rust-based reactive web framework
- **Tailwind CSS**: Utility-first CSS framework
- **Nix**: Reproducible development environment
- **Fermyon Spin**: WebAssembly-based serverless platform

## Development
This project uses Nix flakes for dependency management. Ensure you have Nix installed with flakes enabled.

```bash
# Enter development shell
nix develop

# Start development server
trunk serve
```

## Features
- Fast, responsive UI built with Leptos and Tailwind
- Zero JavaScript dependencies
- Reproducible builds via Nix
- WebAssembly-first architecture
- Hot reloading during development

## Deployment
Deployments are handled through Fermyon Cloud:
```bash
spin deploy
```

## Prerequisites
- Nix package manager with flakes enabled
- Rust toolchain (automatically managed via Nix)
- Fermyon Spin CLI (included in development shell)

## License
MIT

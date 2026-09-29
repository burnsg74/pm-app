# PM App

A fast, local-first project management and ticketing system designed around a
personal workflow.

The goal is to keep the useful parts of Jira, monday.com, and ClickUp—projects,
issues, priorities, status, planning, and time tracking—without their latency,
complexity, or dependence on a browser. The experience should feel closer to
using Zed: focused, responsive, keyboard-driven, and easy to customize.

> This project is an early prototype. Build one slice at a time. Leave the app
> usable at the end of each slice before starting the next one.

## Product principles

- **Local-first:** core features work without an account or internet connection.
- **Fast by default:** instant startup, navigation, search, and updates.
- **Keyboard-first:** every common action has a shortcut or command.
- **Workflow-driven:** adapt the app to the user instead of forcing a rigid process.
- **Focused:** make the current task and next action obvious.
- **Interoperable:** integrate with existing tools when that saves time; do not
recreate them without a clear benefit.
- **Private:** keep project data on the device unless the user explicitly enables  
an external integration or sync.

## Experience and design

The visual direction is inspired by Zed's desktop experience:

- Dense but readable layouts with minimal visual noise.
- A project sidebar, central board, and contextual detail panel.
- Fast transitions with no decorative animation that delays interaction.
- Clear typography, restrained color, and strong focus states.
- Dark theme
- Discoverable shortcut hints.

Accessibility remains a requirement: keyboard focus, contrast, reduced motion,  
and screen-reader semantics should stay intact as the layout gets denser.

## Technical direction

The desktop app uses:

- [Tauri 2](https://tauri.app/) for the native application shell.
- [Svelte 5](https://svelte.dev/) for the interface.
- [Vite](https://vite.dev/) for development and builds.
- Rust for native capabilities.
- SQLite for local storage.
- Svelte Store for caching 

Implementation priorities:

- Explicit, versioned database migrations.
- Native menu-bar support 
- Background work kept small, observable, and power-efficient.


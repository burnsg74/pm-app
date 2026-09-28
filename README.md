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

## How to build

Work through the slices below in order. Each slice is one increment for a person
or an AI session.

- Implement only the current slice.
- Keep the decisions in [MVP decisions](#mvp-decisions) fixed until slice 7 is done.
- Treat [Later](#later) as out of scope for the current slice.
- A slice is finished only when its **Done when** checks pass and earlier slices
  still work.

## MVP decisions

These choices stay fixed through the first usable release:

- One local user and one SQLite database.
- Three fixed statuses: Todo, In Progress, and Done.
- A ticket has an ID, project, title, description, status, order, and timestamps.
- The vertical board is the main screen.
- The timer works inside the app before it moves to the menu bar.

## MVP slices

### 1. Persisted tickets

Create a title-only ticket in one default project and show it in a list.

**Done when**

- [ ] A ticket can be created with a title.
- [ ] Created tickets appear in a list.
- [ ] Quit and reopen, and the tickets remain.

### 2. Vertical board

Replace the list with the stacked board described in [Vertical Kanban](#vertical-kanban).

**Done when**

- [ ] Todo, In Progress, and Done are all visible on one screen.
- [ ] Todo and Done stay compact and can be expanded or collapsed.
- [ ] In Progress uses the remaining space.
- [ ] Moving a ticket between statuses survives a restart.

### 3. Ticket detail

Select a ticket and edit it from a detail panel.

**Done when**

- [ ] Selecting a ticket shows its title and description.
- [ ] Title and description edits survive a restart.

### 4. Projects

Add a sidebar for more than one project.

**Done when**

- [ ] A project can be created and selected.
- [ ] Each project shows only its own tickets.
- [ ] Switching projects and restarting preserves that separation.

### 5. Order and basic shortcuts

Reorder work inside a status and move through the board from the keyboard.

**Done when**

- [ ] Ticket order within a status survives a restart.
- [ ] A shortcut creates a ticket.
- [ ] Shortcuts move the selected ticket between statuses.
- [ ] The available shortcuts are visible in the app.

### 6. In-app current task

Track time against one current ticket from inside the app.

**Done when**

- [ ] One ticket can be marked current.
- [ ] Elapsed time is visible in the app.
- [ ] Switching tasks stops the previous timer.
- [ ] Time entries and totals survive a restart.

### 7. Menu bar timer

Show the current task in the macOS menu bar. The menu bar controls the timer; it
does not duplicate the board.

**Done when**

- [ ] The menu bar shows the current task title and elapsed time.
- [ ] Pause, resume, and complete work from the menu bar.
- [ ] Switch among recent tasks from the menu bar.
- [ ] These actions work while the main window is hidden.
- [ ] Timer state still matches the app after a restart.

After slice 7, the MVP is a local board with durable tasks, keyboard movement,
and a current-task timer.

## Vertical Kanban

The board stacks three workflow levels from top to bottom:

1. **Todo**
2. **In Progress**
3. **Done**

In Progress is the primary working area. Todo and Done use compact,
screen-limited sections so all three levels remain visible without horizontal
scrolling. Each compact section shows an item count and a useful ticket preview,
and can be expanded or collapsed. Tickets move between levels with controls,
drag and drop, or keyboard shortcuts as those interactions land in their slices.

## Experience and design

The visual direction is inspired by Zed's desktop experience:

- Dense but readable layouts with minimal visual noise.
- A project sidebar, central board, and contextual detail panel.
- Fast transitions with no decorative animation that delays interaction.
- Clear typography, restrained color, and strong focus states.
- Dark and light themes, with configurable accent colors and density.
- Discoverable shortcut hints.

Accessibility remains a requirement: keyboard focus, contrast, reduced motion,
and screen-reader semantics should stay intact as the layout gets denser.

Build the sidebar in slice 4 and the detail panel in slice 3. Themes, tabs, and
a command palette wait until [Later](#later).

## Later

Start these only after the MVP slices are reliable.

### Custom workflow

- [ ] Labels, priority, due date, and estimate.
- [ ] Subtasks, notes, and links to local files or external resources.
- [ ] Search, filters, and saved views.
- [ ] Custom statuses and project workflows.
- [ ] Archive projects and tickets.
- [ ] Command palette.
- [ ] Editable shortcuts, including an optional Vim-style mode.
- [ ] Themes, density, tabs, and saved workspaces.
- [ ] Export, backup, and an open portable data format.
- [ ] Project templates, automations, and lightweight time summaries.

### Keyboard workflow

The MVP ships only the shortcuts in slice 5. Later keyboard work includes:

- Quick capture from anywhere in the app.
- Project, ticket, and recent-item switchers.
- Command palette with fuzzy matching.
- Shortcuts for priority, completion, and timer control.
- Multi-select and bulk editing.
- User-editable keybindings with conflict detection.

### Menu bar extras

Slice 7 covers the current task, elapsed time, and timer controls. Later menu
bar work includes:

- Quick capture without opening the main window.
- A shortcut to reveal the current task in the app.
- Optional reminders when a timer runs too long or no task is active.

### Integrations

Add an integration only when it shortens a real workflow:

- **Raycast:** capture tasks, search tickets, switch the current task, and control
  the timer.
- **Obsidian:** link tickets to notes and optionally synchronize selected Markdown
  metadata.
- **Cursor and Zed:** open a repository, file, or workspace attached to a ticket.
- **Warp:** open a project directory, launch a saved workflow, or run an approved
  command.
- **Git providers:** connect branches, commits, pull requests, and issues to tickets.
- **Calendars:** show due dates and time blocks without making calendar access a
  requirement.

Integrations stay optional, permission-scoped, and replaceable. The local
database remains the source of truth.

## Technical direction

The desktop app uses:

- [Tauri 2](https://tauri.app/) for the native application shell.
- [Svelte 5](https://svelte.dev/) for the interface.
- [Vite](https://vite.dev/) for development and builds.
- Rust for native capabilities.
- SQLite for local storage.

Implementation priorities:

- Explicit, versioned database migrations.
- Native menu-bar support in slice 7.
- Background work kept small, observable, and power-efficient.
- Tests for persistence, ordering, timers, and shortcuts in the slice that adds
  each behavior.

Indexed search, global shortcuts, and import/export belong to [Later](#later).

## Development

Prerequisites:

- Node.js
- [pnpm](https://pnpm.io/)
- The platform dependencies required by
  [Tauri](https://v2.tauri.app/start/prerequisites/)

Install dependencies and start the desktop app:

```sh
pnpm install
pnpm tauri-dev
```

Run only the web interface:

```sh
pnpm dev
```

Create a production desktop build:

```sh
pnpm tauri-build
```

## Scope

The MVP is one person using the app locally. Team collaboration, hosted accounts,
real-time multiplayer, and enterprise administration stay out of scope until the
local loop is dependable.

## License

MIT

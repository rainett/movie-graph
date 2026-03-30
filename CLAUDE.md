# Movie Graph - Claude Code Context

## Project Overview

Desktop app for building visual graphs of movies and TV shows, exploring connections through actors. Local-first (file-based storage), 80s device aesthetic.

**Stack:** Tauri 2.0 + Svelte 5 + TypeScript + Rust

## Documentation

All specs are in `/docs/`:

| Doc | Content |
|-----|---------|
| [README.md](docs/README.md) | Overview and quick reference |
| [frontend.md](docs/frontend.md) | UI architecture, components, user flows |
| [backend-tauri.md](docs/backend-tauri.md) | Rust commands, services, data models |
| [architect.md](docs/architect.md) | System design, decisions, patterns |
| [design.md](docs/design.md) | Visual language, colors, typography, components |
| [manager.md](docs/manager.md) | Milestones, planning, checklists |

**Always reference these docs when implementing features.**

## Project Structure

```
movie-graph/
├── src/                      # Svelte frontend
│   ├── routes/               # SvelteKit pages
│   │   └── +page.svelte      # Board (device layout root)
│   └── lib/
│       ├── components/
│       │   ├── board/        # Board.svelte (grid bg, two-panel + status bar)
│       │   ├── devices/      # GraphMonitor.svelte, ControlTerminal.svelte
│       │   ├── canvas/       # MovieNode.svelte, ActorNode.svelte (SvelteFlow nodes)
│       │   ├── controls/     # (future: buttons, sliders, toggles)
│       │   └── overlays/     # SettingsModal.svelte (TMDB key + test)
│       ├── stores/
│       │   ├── graph.ts      # graphStore (movies/actors/edges maps + updateMovie/updateActor)
│       │   ├── project.ts    # projectStore (current open project)
│       │   └── selection.ts  # selectionStore (selected node id + type)
│       ├── services/
│       │   └── tauri.ts      # All typed IPC wrappers (invoke calls)
│       ├── commands/         # (future: undo/redo command pattern)
│       ├── types/
│       │   ├── project.ts    # Project, Manifest, RecentProject, ValidationResult
│       │   ├── node.ts       # MovieNode, ActorNode, Status, Position
│       │   ├── edge.ts       # Edge, Relationship
│       │   ├── tmdb.ts       # TMDB response types + tmdbImage() / releaseYear()
│       │   └── config.ts     # AppConfig (M5)
│       └── utils/
│           └── debounce.ts   # Debounce utility
├── src-tauri/                # Rust backend
│   └── src/
│       ├── main.rs           # Entry point
│       ├── lib.rs            # Module exports + command registration
│       ├── error.rs          # Custom Error enum (thiserror)
│       ├── commands/
│       │   ├── project.rs    # create/open/save/validate project, recent projects
│       │   ├── tmdb.rs       # search_movies, search_people, get_movie_details,
│       │   │                 #   get_person_details, test_api_key
│       │   ├── images.rs     # cache_image (download → resize → JPEG) (M5)
│       │   └── config.rs     # get_config, save_config (M5)
│       ├── services/
│       │   ├── file_io.rs    # FileService (atomic JSON read/write)
│       │   ├── cache.rs      # CacheService (file-based, 7-day TTL)
│       │   └── tmdb_client.rs # TmdbClient (Bearer auth, cache-first)
│       ├── models/
│       │   ├── project.rs    # Project, Manifest, RecentProject, ValidationResult
│       │   ├── node.rs       # MovieNode, ActorNode, Status, Position
│       │   ├── edge.rs       # Edge, Relationship
│       │   └── tmdb.rs       # SearchResults, MovieResult, PersonResult,
│       │                     #   MovieDetails, PersonDetails, MovieCredit, etc.
│       └── config/
│           └── app_config.rs # AppConfig (tmdb_read_access_token, recent projects, prefs)
├── static/                   # Static assets
└── docs/                     # Project documentation
```

## Conventions

### TypeScript/Svelte
- Strict mode enabled
- Use Svelte 5 runes: `$state`, `$derived`, `$effect`
- Prefer `type` over `interface` for simple types
- All Tauri invokes wrapped in typed service functions in `src/lib/services/tauri.ts`
- Use inline `setTimeout`/`clearTimeout` for async-callback debouncing (the `debounce` util only supports sync callbacks)

### Rust
- Use `thiserror` for custom errors
- All commands are `async`
- Use `serde` for JSON serialization
- Group related commands in modules

### Naming
- Files: `kebab-case.ts`, `kebab-case.svelte`
- Components: `PascalCase`
- Stores: `camelCase` with `Store` suffix (e.g., `graphStore`)
- Rust modules: `snake_case`
- IPC commands: `snake_case` (e.g., `save_project`)

### Git
- Commit messages: `type(scope): description`
- Types: `feat`, `fix`, `refactor`, `docs`, `test`, `chore`

## Key Patterns

### IPC Communication
```typescript
// Frontend: src/lib/services/tauri.ts
import { invoke } from '@tauri-apps/api/core';

export async function saveProject(project: Project): Promise<void> {
  return invoke('save_project', { project });
}
```

```rust
// Backend: src-tauri/src/commands/project.rs
#[tauri::command]
async fn save_project(project: Project) -> Result<(), Error> {
    // Implementation
}
```

### Svelte Stores
```typescript
// src/lib/stores/graph.ts
import { writable, derived } from 'svelte/store';

export const graphStore = createGraphStore(); // custom store with methods
export const movieCount = derived(graphStore, $g => $g.movies.size);
```

### Node Selection (cross-component)
```typescript
// GraphMonitor writes, ControlTerminal reads
import { selectionStore } from '$lib/stores/selection';
// { id: 'm:123', type: 'movie' } | { id: 'a:456', type: 'actor' } | null
```

### TMDB Client (Rust)
```rust
// Bearer token auth, cache-first, default token embedded
let client = TmdbClient::new(None); // uses DEFAULT_READ_TOKEN
let results = client.search_movies("inception", 1).await?;
```

### Command Pattern (Undo/Redo) — M6
```typescript
// src/lib/commands/node.ts (future)
export class AddNodeCommand implements Command {
  execute() { /* add node */ }
  undo() { /* remove node */ }
}
```

## Installed Dependencies

### Frontend (npm)
- `@xyflow/svelte` v1.5.2 — graph canvas (Svelte 5 native)
- `elkjs` — auto-layout (M4)
- `fuse.js` — fuzzy search within graph (M7)
- Tailwind CSS v4 + `@tailwindcss/postcss`

### Backend (Cargo.toml)
- `tauri` (features: `protocol-asset`), `tauri-plugin-dialog`, `tauri-plugin-opener`
- `serde`, `serde_json`, `tokio`, `thiserror`, `dirs`, `uuid`, `chrono`
- `reqwest 0.12` (Bearer auth TMDB client)
- `dotenvy 0.15` (loads `.env` in dev)
- `image 0.24` (jpeg feature — download/resize/encode, M5)

**Still needed in future milestones:**
```toml
tauri-plugin-fs = "2"
```

## Custom Skills

Project-specific skills in `.claude/skills/`:

| Skill | Purpose | When to Use |
|-------|---------|-------------|
| `/plan-milestone <N>` | Decompose milestone into atomic tasks mapped to skills/agents | **Before starting a milestone** |
| `/tauri-command <name>` | Generate Tauri IPC command (Rust + TS binding) | Adding new backend commands |
| `/svelte-component <name>` | Create Svelte 5 component with 80s styling | New UI components |
| `/check-docs <area>` | Verify implementation matches docs | Before starting milestone |
| `/milestone-status [N]` | Check progress + suggest next steps | During/after milestone |
| `/sync-state [N]` | Update CLAUDE.md + MEMORY.md | **At milestone boundaries** |

### Recommended Milestone Workflow

**Start:** `/plan-milestone N` → `/check-docs` → implement atomic units
**During:** `/milestone-status N` to check progress
**End:** `/sync-state N` → commit `feat(MN): ...` → push

## Hooks (Automatic)

Configured in `.claude/settings.json`:

| Trigger | Action |
|---------|--------|
| Edit `*.rs` | Auto-runs `cargo check` (output in hook) |
| Edit `*.ts` / `*.svelte` | Auto-runs `npm run check` |
| Edit `CLAUDE.md` | Reminds to commit + run `/milestone-status` |

## Custom Agents

Project-specific agents in `.claude/agents/`:

| Agent | Model | When to Use |
|-------|-------|-------------|
| `@rust-expert` | Opus | Complex async, lifetimes, performance, Tauri plugins |
| `@svelte-expert` | Sonnet | Complex reactivity, SvelteFlow issues, store architecture |
| `@design-reviewer` | Sonnet | **After `/svelte-component`** or UI changes — checks design.md compliance |
| `@explorer` | Haiku | Quick lookups; for this small project, direct Glob/Read is often faster |

**Usage:** `@design-reviewer Review ControlTerminal.svelte` or ask "use rust-expert to optimize..."

## Current Focus

**Milestone:** M6 - History System (Undo/Redo)
**Status:** Next

### M1 — Foundation: COMPLETE
- [x] Tauri + SvelteKit scaffolded, Tailwind CSS v4 configured
- [x] Frontend folder structure (`src/lib/` tree)
- [x] Rust module structure (`commands/`, `services/`, `models/`, `config/`)
- [x] Project model + file I/O (atomic JSON, FileService)
- [x] Project commands: `create_project`, `open_project`, `save_project`, `get_recent_projects`, `validate_project`
- [x] Frontend IPC service layer (`src/lib/services/tauri.ts`)
- [x] Toolchain: `rust-gnu` + `mingw` via scoop

### M2 — Graph Core: COMPLETE
- [x] Device board layout (`Board.svelte`, `GraphMonitor.svelte`, `ControlTerminal.svelte`)
- [x] `@xyflow/svelte` integrated in GraphMonitor
- [x] `MovieNode.svelte`, `ActorNode.svelte` custom node components
- [x] Pan/zoom, node drag, position sync back to `graphStore`
- [x] CSS design tokens in `src/app.css`
- [x] `+page.svelte` replaced with Board

### M3 — TMDB Integration: COMPLETE
- [x] `TmdbClient` in Rust (Bearer token auth, cache-first with 7-day TTL)
- [x] `CacheService` (file-based JSON cache at `~/.config/movie-graph/cache/`)
- [x] IPC commands: `search_movies`, `search_people`, `get_movie_details`, `get_person_details`, `test_api_key`
- [x] TMDB types in Rust (`models/tmdb.rs`) and TypeScript (`types/tmdb.ts`)
- [x] ControlTerminal Search mode (debounced input → results list → preview with cast → ADD TO GRAPH)
- [x] ControlTerminal Inspect mode (movie details or actor details + bio)
- [x] Actor suggestions: top 5 most popular movies not in graph, shown in Inspect mode
- [x] `selectionStore` bridges GraphMonitor node clicks → ControlTerminal Inspect mode

### M4 — Graph Building: COMPLETE
- [x] Wire ADD TO GRAPH button (search preview → `graphStore.addMovie` + auto-edges)
- [x] Wire [+] suggestion buttons (actor suggestions → `graphStore.addMovie` + **`actor→movie`** edge)
- [x] Wire [ADD ACTOR] from movie cast (Inspect mode → `graphStore.addActor` + `movie→actor` edge)
- [x] Auto-create edges on add (checks existing nodes for connections)
- [x] ELK.js layered auto-layout (direction RIGHT); edge direction encodes discovery order
- [x] Project open/save from status bar (`pick_folder` Rust command + OPEN/NEW/SAVE buttons)
- [x] Ctrl+S to save; NEW project modal with name input
- [x] Delete node from Inspect mode (REMOVE FROM GRAPH, cascades edges)
- [x] `projectStore` stores full `Manifest` for correct round-trip saves
- [x] Fix inspect mode lock: `untrack()` around `activeMode` write in `$effect`
- [x] Fix canvas deselect: `onpaneclick` clears `selectionStore`
- [x] TMDB token fallback: `api_key` → `AppConfig.tmdb_read_access_token` → env var
- [x] `dotenvy` loads `.env` in dev; `elkjs` aliased to bundled version in `vite.config.js`

### M5 — Image Caching & Polish: COMPLETE
- [x] Download and cache poster/photo images locally (permanent cache)
- [x] `image` crate for Rust image processing (resize ≤500px, JPEG q80)
- [x] Serve cached images via asset protocol (`convertFileSrc` + `protocol-asset`)
- [x] Settings modal: TMDB API key input + test button (⚙ in status bar)
- [x] Node status badge editing (status select in Inspect mode → `graphStore.updateMovie`)
- [x] Edge labels / relationship types visible on graph (ACTED / LIKED / REC)
- [x] `get_config` / `save_config` IPC commands + `AppConfig` TS type
- [x] `graphStore.updateMovie` / `updateActor` partial-update methods
- [x] `/plan-milestone` skill added for decomposing future milestones

### M6 — History System: Next
- [ ] Command pattern implementation (`src/lib/commands/`)
- [ ] Add node command (movie + actor)
- [ ] Delete node command
- [ ] Move node command
- [ ] Edit node command (status, rating, notes)
- [ ] Add/delete edge commands
- [ ] Undo (Ctrl+Z) / Redo (Ctrl+Shift+Z) keyboard shortcuts
- [ ] History strip UI in Graph Monitor (tape transport display)

## Quick Commands

```bash
# Development
npm run dev              # Start Vite dev server
npm run tauri dev        # Start Tauri dev mode

# Build
npm run build            # Build frontend
npm run tauri build      # Build full app

# Check
npm run check            # TypeScript/Svelte check
cd src-tauri && cargo check  # Rust check
cargo test               # Rust tests
# Note: requires C:\Users\PavloKoval1\scoop\apps\mingw\current\bin in Windows PATH (gnu toolchain).
# If dlltool.exe or x86_64-w64-mingw32-gcc is not found, add that path via System Environment Variables.
```

## TMDB API

- Base URL: `https://api.themoviedb.org/3`
- Image base: `https://image.tmdb.org/t/p/{size}`
- Sizes: `w200`, `w500`, `original`
- Auth: Bearer token (read access token) via `Authorization` header
- Default token embedded in `src-tauri/src/services/tmdb_client.rs` (`DEFAULT_READ_TOKEN`)
- User override: `AppConfig.tmdb_read_access_token` (persisted to `~/.config/movie-graph/config.json`)

## Notes

- Target: Scale to 1000+ nodes
- Offline-first: Cache TMDB responses locally (7 days), images cached permanently (M5)
- User data format: JSON files in user-selected folder
- Visual style: 80s device aesthetic (see design.md)
- Node IDs: `m:{tmdbId}` for movies, `a:{tmdbId}` for actors

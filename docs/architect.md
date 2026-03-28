# Movie Graph - Architect Guide

## Overview

This document covers system architecture, design decisions, module boundaries, and technical guidelines for the Movie Graph application. Written for a solo developer building a desktop app that scales to thousands of nodes with future extensibility for plugins and localization.

---

## System Architecture

### High-Level Overview

```
┌─────────────────────────────────────────────────────────────────────┐
│                         Desktop Application                          │
│  ┌───────────────────────────────────────────────────────────────┐  │
│  │                      Presentation Layer                        │  │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐   │  │
│  │  │   Devices   │  │   Canvas    │  │   Overlays/Modals   │   │  │
│  │  │  (Svelte)   │  │(Svelte Flow)│  │      (Svelte)       │   │  │
│  │  └──────┬──────┘  └──────┬──────┘  └──────────┬──────────┘   │  │
│  │         └────────────────┼────────────────────┘              │  │
│  │                          │                                    │  │
│  │  ┌───────────────────────┴───────────────────────────────┐   │  │
│  │  │                   State Layer (Svelte Stores)          │   │  │
│  │  │  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────────┐  │   │  │
│  │  │  │ Project │ │  Graph  │ │ History │ │    UI       │  │   │  │
│  │  │  │  Store  │ │  Store  │ │  Store  │ │   Store     │  │   │  │
│  │  │  └─────────┘ └─────────┘ └─────────┘ └─────────────┘  │   │  │
│  │  └───────────────────────┬───────────────────────────────┘   │  │
│  └──────────────────────────┼────────────────────────────────────┘  │
│                             │ IPC (invoke/events)                    │
│  ┌──────────────────────────┴────────────────────────────────────┐  │
│  │                       Backend Layer (Rust)                     │  │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────────────┐  │  │
│  │  │ Commands │ │ Services │ │  Models  │ │  Plugin Host     │  │  │
│  │  └──────────┘ └──────────┘ └──────────┘ └──────────────────┘  │  │
│  └───────────────────────────────────────────────────────────────┘  │
│                             │                                        │
└─────────────────────────────┼────────────────────────────────────────┘
                              │
              ┌───────────────┴───────────────┐
              │        External Systems        │
              │  ┌─────────┐  ┌────────────┐  │
              │  │  TMDB   │  │ File System│  │
              │  │   API   │  │            │  │
              │  └─────────┘  └────────────┘  │
              └───────────────────────────────┘
```

### Layer Responsibilities

| Layer | Responsibility | Technology |
|-------|----------------|------------|
| **Presentation** | UI rendering, user interaction, visual feedback | Svelte 5, Svelte Flow, Tailwind |
| **State** | Application state, reactivity, history tracking | Svelte stores |
| **Backend** | File I/O, HTTP, image processing, plugins | Rust, Tauri 2.0 |
| **External** | Movie data, persistent storage | TMDB API, File system |

---

## Data Flow

### Unidirectional Data Flow

```
User Action
    │
    ▼
┌─────────────────┐
│  UI Component   │ ──────────────────────────────────────┐
└────────┬────────┘                                       │
         │ dispatch action                                │
         ▼                                                │
┌─────────────────┐                                       │
│  Store Action   │                                       │
└────────┬────────┘                                       │
         │                                                │
         ▼                                                │
┌─────────────────┐     ┌─────────────────┐              │
│  Update State   │────▶│  Persist Data   │              │
└────────┬────────┘     │  (Backend IPC)  │              │
         │              └─────────────────┘              │
         │ reactive update                               │
         ▼                                               │
┌─────────────────┐                                      │
│  Derived State  │                                      │
└────────┬────────┘                                      │
         │                                               │
         └───────────────────────────────────────────────┘
                        re-render
```

### Key Data Flows

#### 1. Adding a Movie

```
SearchInput.svelte
    │ user types query
    ▼
uiStore.setSearchQuery(query)
    │
    ▼
searchResults (derived store)
    │ debounced, triggers on query change
    ▼
invoke('search_movies', { query })
    │
    ▼
TmdbClient::search_movies()
    │ check cache → fetch API → cache response
    ▼
Results returned to frontend
    │
    ▼
User clicks "Add"
    │
    ▼
graphStore.addMovie(movieData)
    │
    ├──▶ historyStore.push(AddMovieCommand)
    │
    ├──▶ invoke('cache_poster', { url, filename })
    │
    └──▶ invoke('save_movies', { movies })
            │
            ▼
        FileService::write_json_atomic()
```

#### 2. Undo Operation

```
User presses Ctrl+Z
    │
    ▼
historyStore.undo()
    │
    ├──▶ command.undo() // Reverses state change
    │
    └──▶ graphStore updated reactively
            │
            ▼
        Canvas re-renders
            │
            ▼
        invoke('save_*') // Persist reversed state
```

#### 3. Filter Application

```
User toggles "Watched" filter
    │
    ▼
filterStore.toggle('watched')
    │
    ▼
filteredNodes (derived store)
    │ computed from graphStore + filterStore
    ▼
Canvas receives filtered data
    │
    ▼
Non-matching nodes dimmed (CSS opacity)
```

---

## Module Boundaries

### Frontend Modules

```
src/
├── lib/
│   ├── components/
│   │   ├── board/           # Board container, grid background
│   │   ├── devices/         # GraphMonitor, ControlTerminal
│   │   ├── canvas/          # Svelte Flow wrapper, custom nodes
│   │   ├── controls/        # 80s-styled buttons, sliders, toggles
│   │   ├── overlays/        # Modals, posters picker, settings
│   │   └── shared/          # Icons, loaders, status indicators
│   │
│   ├── stores/
│   │   ├── project.ts       # Project metadata, path
│   │   ├── graph.ts         # Nodes, edges, positions
│   │   ├── history.ts       # Undo/redo command stack
│   │   ├── ui.ts            # Device layout, mode, selection
│   │   ├── filter.ts        # Active filters
│   │   └── config.ts        # App settings
│   │
│   ├── services/
│   │   ├── tauri.ts         # IPC wrapper, typed invoke
│   │   ├── tmdb.ts          # TMDB data transformations
│   │   ├── layout.ts        # ELK.js integration
│   │   ├── sound.ts         # Audio feedback
│   │   └── i18n.ts          # Localization
│   │
│   ├── commands/            # Undo/redo command implementations
│   │   ├── base.ts          # Command interface
│   │   ├── node.ts          # Add/delete/edit node commands
│   │   ├── edge.ts          # Add/delete edge commands
│   │   └── batch.ts         # Batch operations (layout, etc.)
│   │
│   ├── types/               # TypeScript interfaces
│   │   ├── node.ts
│   │   ├── edge.ts
│   │   ├── project.ts
│   │   └── tmdb.ts
│   │
│   ├── utils/
│   │   ├── debounce.ts
│   │   ├── id.ts            # ID generation
│   │   └── format.ts        # Date, duration formatting
│   │
│   └── i18n/
│       ├── en.json
│       ├── uk.json
│       └── index.ts
│
├── routes/
│   └── +page.svelte         # Main app entry
│
└── app.css                  # Global styles, Tailwind
```

### Backend Modules

```
src-tauri/src/
├── main.rs                  # Entry point
├── lib.rs                   # Module exports
├── error.rs                 # Error types
│
├── commands/                # IPC command handlers
│   ├── mod.rs
│   ├── project.rs
│   ├── nodes.rs
│   ├── edges.rs
│   ├── images.rs
│   ├── tmdb.rs
│   ├── config.rs
│   └── backup.rs
│
├── services/                # Business logic
│   ├── mod.rs
│   ├── file_io.rs
│   ├── cache.rs
│   ├── backup.rs
│   ├── image_processor.rs
│   └── tmdb_client.rs
│
├── models/                  # Data structures
│   ├── mod.rs
│   ├── project.rs
│   ├── node.rs
│   ├── edge.rs
│   ├── config.rs
│   └── tmdb.rs
│
├── plugins/                 # Plugin system
│   ├── mod.rs
│   ├── host.rs              # Plugin loader
│   ├── api.rs               # Plugin API definitions
│   └── builtin/             # Built-in "plugins"
│
└── crash/                   # Crash reporting
    ├── mod.rs
    └── reporter.rs
```

### Module Dependency Rules

```
┌─────────────────────────────────────────────────────┐
│                    commands/                         │
│         (depends on services, models)               │
└──────────────────────┬──────────────────────────────┘
                       │
         ┌─────────────┴─────────────┐
         ▼                           ▼
┌─────────────────┐       ┌─────────────────┐
│    services/    │       │     models/     │
│ (depends on     │       │  (no deps)      │
│  models only)   │       │                 │
└────────┬────────┘       └─────────────────┘
         │
         ▼
┌─────────────────┐
│     models/     │
└─────────────────┘
```

**Rules:**
- `models/` has no internal dependencies (pure data structures)
- `services/` depends only on `models/`
- `commands/` depends on `services/` and `models/`
- `plugins/` has controlled access to `services/` via API

---

## Key Architectural Decisions

### 1. Tauri over Electron

**Decision:** Use Tauri 2.0 with Rust backend

**Rationale:**
- ~5MB bundle vs ~150MB (Electron)
- Better memory usage (important for thousands of nodes)
- Rust provides memory safety and performance
- Native system integration
- Security-first design with capability-based permissions

**Trade-offs:**
- Smaller ecosystem than Electron
- Must write backend in Rust (learning curve)
- Some platform-specific quirks

---

### 2. Svelte over React/Vue

**Decision:** Use Svelte 5 for frontend

**Rationale:**
- Smaller bundle size
- True reactivity without virtual DOM (better for canvas-heavy UI)
- Less boilerplate
- Runes (Svelte 5) provide fine-grained reactivity
- Svelte Flow available (xyflow/svelte)

**Trade-offs:**
- Smaller community than React
- Fewer ready-made components
- Team familiarity (if expanding later)

---

### 3. File-based Storage over Database

**Decision:** Store data as JSON files in user-chosen directory

**Rationale:**
- User owns their data (no lock-in)
- Works like Obsidian (familiar model)
- No database setup/migration complexity
- Easy backup (just copy folder)
- Human-readable/editable if needed
- Git-friendly for advanced users

**Trade-offs:**
- No complex queries (must load into memory)
- File I/O performance limits
- Need custom indexing for large datasets

**Mitigation for scale:**
- Batch file structure (one file per type, not per node)
- In-memory indexing after load
- Lazy loading for images

---

### 4. Command Pattern for History

**Decision:** Implement undo/redo using command pattern

**Rationale:**
- Clean separation of action and execution
- Each command is self-contained with undo logic
- Easy to serialize for session recovery
- Can batch commands for complex operations
- Extensible for plugins

**Implementation:**

```typescript
interface Command {
  readonly id: string;
  readonly description: string;
  execute(): void;
  undo(): void;
}

class AddNodeCommand implements Command {
  constructor(
    private store: GraphStore,
    private node: Node
  ) {}

  execute() {
    this.store.addNode(this.node);
  }

  undo() {
    this.store.removeNode(this.node.id);
  }
}
```

---

### 5. Hybrid API Key Strategy

**Decision:** Ship with embedded key, allow user override

**Rationale:**
- Zero friction for new users
- Graceful degradation if key is rate-limited
- Power users can use their own key
- No backend infrastructure needed

**Implementation:**

```rust
fn get_api_key(user_key: Option<String>) -> String {
    user_key
        .or_else(|| std::env::var("TMDB_API_KEY").ok())
        .unwrap_or_else(|| DEFAULT_API_KEY.to_string())
}
```

---

### 6. Device-based UI Architecture

**Decision:** UI composed of independent "devices" on a board

**Rationale:**
- Maps to mental model (control room)
- Each device is self-contained (easier testing)
- Flexible layout (user customization)
- Progressive disclosure (show/hide devices)
- Plugin-friendly (plugins can add devices)

**Implementation:**

```typescript
interface Device {
  id: string;
  component: SvelteComponent;
  defaultPosition: Position;
  defaultSize: Size;
  minSize: Size;
  maxSize?: Size;
  canClose: boolean;
}

// Device registry
const devices: Map<string, Device> = new Map();

// Plugin can register new devices
plugins.registerDevice({
  id: 'my-plugin-device',
  component: MyPluginDevice,
  // ...
});
```

---

### 7. Virtualization for Scale

**Decision:** Implement canvas virtualization for 1000+ nodes

**Rationale:**
- DOM can't handle thousands of complex nodes
- Only render visible viewport
- Smooth pan/zoom at any scale

**Implementation Strategy:**

```
┌─────────────────────────────────────────┐
│            Full Graph (memory)          │
│    ┌─────────────────────────────┐      │
│    │      Spatial Index          │      │
│    │      (R-tree / Quadtree)    │      │
│    └──────────────┬──────────────┘      │
│                   │                      │
│    ┌──────────────▼──────────────┐      │
│    │    Viewport Query           │      │
│    │    (visible bounds + buffer)│      │
│    └──────────────┬──────────────┘      │
│                   │                      │
│    ┌──────────────▼──────────────┐      │
│    │    Rendered Nodes           │      │
│    │    (50-200 nodes)           │      │
│    └─────────────────────────────┘      │
└─────────────────────────────────────────┘
```

Svelte Flow handles this internally, but custom optimizations may be needed:
- Level-of-detail rendering (thumbnails when zoomed out)
- Node pooling (reuse DOM elements)
- Debounced viewport updates

---

### 8. Plugin Architecture

**Decision:** Support plugins for extensibility

**Rationale:**
- Custom data sources (beyond TMDB)
- Custom node types
- Custom export formats
- Community contributions
- Keeps core focused

**Plugin API (v1):**

```rust
// src-tauri/src/plugins/api.rs

pub trait Plugin: Send + Sync {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn version(&self) -> &str;

    /// Called when plugin is loaded
    fn on_load(&mut self, ctx: PluginContext) -> Result<(), PluginError>;

    /// Called when plugin is unloaded
    fn on_unload(&mut self) -> Result<(), PluginError>;
}

pub trait DataSourcePlugin: Plugin {
    /// Search for items
    fn search(&self, query: &str) -> Result<Vec<SearchResult>, PluginError>;

    /// Get item details
    fn get_details(&self, id: &str) -> Result<ItemDetails, PluginError>;
}

pub struct PluginContext {
    pub config_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub http_client: HttpClient,
    pub event_emitter: EventEmitter,
}
```

**Plugin Discovery:**
- Plugins stored in `~/.config/movie-graph/plugins/`
- Each plugin is a directory with `manifest.json`
- Rust plugins compiled as dynamic libraries
- JS plugins run in isolated context (future)

---

### 9. Localization Strategy

**Decision:** JSON-based i18n with compile-time extraction

**Rationale:**
- Simple format (JSON)
- Easy for translators
- Type-safe keys with TypeScript
- No runtime overhead for static strings

**Implementation:**

```typescript
// src/lib/i18n/index.ts
import en from './en.json';
import uk from './uk.json';

const translations = { en, uk };

type TranslationKey = keyof typeof en;

function t(key: TranslationKey, params?: Record<string, string>): string {
  const locale = get(localeStore);
  let text = translations[locale]?.[key] ?? translations.en[key] ?? key;

  if (params) {
    Object.entries(params).forEach(([k, v]) => {
      text = text.replace(`{${k}}`, v);
    });
  }

  return text;
}

// Usage
t('search.placeholder'); // "Search movies..."
t('node.added', { title: 'Inception' }); // "Added: Inception"
```

**Translation File Structure:**

```json
// en.json
{
  "app.name": "Movie Graph",
  "search.placeholder": "Search movies or actors...",
  "search.no_results": "No results found",
  "node.movie": "Movie",
  "node.actor": "Actor",
  "status.watched": "Watched",
  "status.want_to_watch": "Want to Watch",
  "error.api_unavailable": "TMDB is unavailable. Check your connection.",
  "error.invalid_api_key": "Invalid API key. Please check your settings."
}
```

---

### 10. Crash Reporting

**Decision:** Implement opt-in crash reporting

**Rationale:**
- Understand real-world failures
- Improve stability
- No usage analytics (privacy-respecting)

**Implementation:**

```rust
// src-tauri/src/crash/reporter.rs

use std::panic;

pub fn init_crash_handler() {
    panic::set_hook(Box::new(|panic_info| {
        let report = CrashReport {
            timestamp: chrono::Utc::now(),
            version: env!("CARGO_PKG_VERSION"),
            os: std::env::consts::OS,
            arch: std::env::consts::ARCH,
            message: panic_info.to_string(),
            backtrace: std::backtrace::Backtrace::capture().to_string(),
        };

        // Save locally
        if let Ok(path) = save_crash_report(&report) {
            // Next launch will prompt to send
            eprintln!("Crash report saved: {:?}", path);
        }
    }));
}

#[derive(Serialize)]
struct CrashReport {
    timestamp: chrono::DateTime<chrono::Utc>,
    version: &'static str,
    os: &'static str,
    arch: &'static str,
    message: String,
    backtrace: String,
    // NO user data, NO project contents
}
```

**Privacy:**
- Reports contain only technical data
- No file contents, no project names
- User must explicitly click "Send" after crash
- Can view report before sending

---

## State Management Architecture

### Store Hierarchy

```
┌─────────────────────────────────────────────────────────────┐
│                      Root Stores                             │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │ configStore │  │projectStore │  │     uiStore         │  │
│  │ (app-wide)  │  │ (per-proj)  │  │  (session state)    │  │
│  └─────────────┘  └──────┬──────┘  └─────────────────────┘  │
│                          │                                   │
│         ┌────────────────┼────────────────┐                 │
│         ▼                ▼                ▼                 │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │
│  │ graphStore  │  │ historyStore│  │ filterStore │         │
│  │(nodes/edges)│  │(undo/redo)  │  │(active filt)│         │
│  └──────┬──────┘  └─────────────┘  └──────┬──────┘         │
│         │                                  │                 │
│         └──────────────┬───────────────────┘                │
│                        ▼                                     │
│              ┌─────────────────┐                            │
│              │  Derived Stores │                            │
│              │ (filteredNodes, │                            │
│              │  visibleEdges)  │                            │
│              └─────────────────┘                            │
└─────────────────────────────────────────────────────────────┘
```

### Store Definitions

```typescript
// configStore - persisted app-wide settings
interface ConfigState {
  tmdbApiKey: string | null;
  recentProjects: RecentProject[];
  soundEnabled: boolean;
  reducedMotion: boolean;
  locale: string;
  deviceLayoutPreset: string;
}

// projectStore - current project metadata
interface ProjectState {
  path: string | null;
  name: string;
  isLoaded: boolean;
  isDirty: boolean;
  lastSaved: Date | null;
}

// graphStore - core data
interface GraphState {
  movies: Map<string, MovieNode>;
  actors: Map<string, ActorNode>;
  edges: Map<string, Edge>;
}

// uiStore - transient UI state
interface UIState {
  selectedNodeId: string | null;
  controlTerminalMode: 'search' | 'inspect' | 'filter';
  searchQuery: string;
  searchResults: SearchResult[];
  isSearching: boolean;
  devicePositions: Map<string, DevicePosition>;
  cameraPosition: { x: number; y: number; zoom: number };
}

// historyStore - undo/redo
interface HistoryState {
  undoStack: Command[];
  redoStack: Command[];
  maxSize: number;
}

// filterStore - active filters
interface FilterState {
  status: Status[];
  ratingMin: number | null;
  ratingMax: number | null;
  yearMin: number | null;
  yearMax: number | null;
  nodeType: 'all' | 'movie' | 'actor';
  searchText: string;
}
```

### Derived Stores

```typescript
// Computed from graphStore + filterStore
const filteredNodes = derived(
  [graphStore, filterStore],
  ([$graph, $filter]) => {
    return filterNodes($graph, $filter);
  }
);

// Computed visibility map (for dimming)
const nodeVisibility = derived(
  [graphStore, filteredNodes],
  ([$graph, $filtered]) => {
    const visibility = new Map<string, boolean>();
    for (const id of $graph.movies.keys()) {
      visibility.set(id, $filtered.has(id));
    }
    for (const id of $graph.actors.keys()) {
      visibility.set(id, $filtered.has(id));
    }
    return visibility;
  }
);
```

---

## Error Handling Strategy

### Error Categories

| Category | Examples | Handling |
|----------|----------|----------|
| **User Error** | Invalid input, no selection | Inline validation, toast |
| **Recoverable** | Network timeout, rate limit | Retry with backoff, user action |
| **Data Error** | Corrupt file, invalid JSON | Recovery attempt, backup restore |
| **Fatal** | Out of memory, disk full | Crash report, graceful shutdown |

### Error Flow

```
Error occurs
    │
    ▼
┌─────────────────────┐
│ Categorize error    │
└──────────┬──────────┘
           │
    ┌──────┴──────┬──────────────┬────────────────┐
    ▼             ▼              ▼                ▼
User Error   Recoverable     Data Error       Fatal
    │             │              │                │
    ▼             ▼              ▼                ▼
Show inline   Show toast    Show recovery    Save crash
message       with retry    options modal    report, exit
```

### Frontend Error Handling

```typescript
// src/lib/services/tauri.ts

async function invoke<T>(cmd: string, args?: object): Promise<T> {
  try {
    return await tauriInvoke<T>(cmd, args);
  } catch (error) {
    const appError = parseError(error);

    switch (appError.category) {
      case 'network':
        errorStore.setNetworkError(appError);
        break;
      case 'api':
        if (appError.code === 'RATE_LIMITED') {
          await handleRateLimit();
        } else if (appError.code === 'INVALID_API_KEY') {
          uiStore.showApiKeyPrompt();
        }
        break;
      case 'file':
        errorStore.setFileError(appError);
        break;
      default:
        errorStore.setGenericError(appError);
    }

    throw appError;
  }
}
```

### Backend Error Propagation

```rust
// All commands return Result<T, Error>
// Error is serialized and sent to frontend

#[tauri::command]
async fn save_project(project: Project) -> Result<(), Error> {
    FileService::write_json_atomic(&project.path, &project)
        .await
        .map_err(|e| Error::FileWrite(e.to_string()))?;

    Ok(())
}
```

---

## Performance Architecture

### Targets

| Metric | Target | Notes |
|--------|--------|-------|
| Initial load | < 2s | For 1000-node project |
| Search response | < 300ms | Including network |
| Pan/zoom | 60 FPS | Even with 1000 nodes |
| Undo/redo | < 50ms | Instant feel |
| Save | < 500ms | Debounced |

### Optimization Strategies

#### 1. Canvas Rendering

```
Zoom Level          Rendering Strategy
─────────────────────────────────────────
> 100%              Full detail (poster, title, badges)
50-100%             Medium detail (poster, title)
25-50%              Low detail (poster thumbnail only)
< 25%               Dots/rectangles only
```

#### 2. Data Loading

```typescript
// Lazy load images
function loadNodeImage(node: Node) {
  if (node.imageLoaded) return;

  // Check if node is in viewport (with buffer)
  if (!isInViewport(node.position, viewportBuffer)) return;

  // Load image asynchronously
  loadImage(node.poster).then(url => {
    node.imageUrl = url;
    node.imageLoaded = true;
  });
}
```

#### 3. Batch Updates

```typescript
// DON'T: Save after each change
nodes.forEach(node => {
  invoke('save_node', { node });  // N API calls
});

// DO: Batch changes
invoke('save_movies', { movies: nodes });  // 1 API call
```

#### 4. Debouncing

```typescript
// Auto-save debounced
const saveDebounced = debounce(async () => {
  await invoke('save_project', { project: get(projectStore) });
  projectStore.markSaved();
}, 2000);

graphStore.subscribe(() => {
  projectStore.markDirty();
  saveDebounced();
});
```

---

## Security Architecture

### Threat Model

| Threat | Mitigation |
|--------|------------|
| Malicious project file | Validate JSON schema, sanitize paths |
| XSS in node content | CSP, sanitize user input |
| Path traversal | Validate all paths are within project dir |
| API key exposure | Store in app config, never in project files |
| Arbitrary code execution | No eval(), sandboxed plugins |

### Input Validation

```rust
// Validate file paths
fn validate_project_path(path: &PathBuf) -> Result<(), Error> {
    // Must be absolute
    if !path.is_absolute() {
        return Err(Error::InvalidPath("Path must be absolute".into()));
    }

    // Must not contain path traversal
    let normalized = path.canonicalize()?;
    if normalized != *path {
        return Err(Error::InvalidPath("Path contains traversal".into()));
    }

    // Must be within allowed directories
    let allowed_roots = get_allowed_roots();
    if !allowed_roots.iter().any(|root| normalized.starts_with(root)) {
        return Err(Error::InvalidPath("Path outside allowed directories".into()));
    }

    Ok(())
}
```

### Content Security Policy

```json
// tauri.conf.json
{
  "security": {
    "csp": {
      "default-src": "'self'",
      "img-src": "'self' https://image.tmdb.org data: blob:",
      "style-src": "'self' 'unsafe-inline'",
      "script-src": "'self'",
      "connect-src": "'self' https://api.themoviedb.org"
    }
  }
}
```

---

## Testing Architecture

### Test Pyramid

```
              ┌───────────┐
              │    E2E    │  Few, slow, high confidence
              │  (Tauri)  │
              └─────┬─────┘
                    │
           ┌───────┴───────┐
           │  Integration   │  Some, medium speed
           │   (Stores +    │
           │   Components)  │
           └───────┬───────┘
                   │
        ┌──────────┴──────────┐
        │      Unit Tests      │  Many, fast
        │ (Functions, Utils,   │
        │  Commands, Services) │
        └─────────────────────┘
```

### Test Organization

```
tests/
├── unit/
│   ├── frontend/
│   │   ├── stores/
│   │   ├── commands/
│   │   └── utils/
│   └── backend/
│       ├── services/
│       └── models/
│
├── integration/
│   ├── project_lifecycle.test.ts
│   ├── graph_operations.test.ts
│   └── tmdb_integration.test.ts
│
└── e2e/
    ├── new_user_flow.test.ts
    ├── add_movie_flow.test.ts
    └── undo_redo.test.ts
```

### Testing Utilities

```typescript
// tests/utils/test-helpers.ts

export function createMockProject(): Project {
  return {
    path: '/tmp/test-project',
    manifest: createMockManifest(),
    movies: [],
    actors: [],
    edges: [],
  };
}

export function createMockMovie(overrides?: Partial<MovieNode>): MovieNode {
  return {
    id: `m:${randomId()}`,
    tmdbId: randomInt(),
    title: 'Test Movie',
    year: 2020,
    status: 'none',
    myRating: null,
    poster: 'posters/test.jpg',
    posterOptions: [],
    notes: '',
    position: { x: 0, y: 0 },
    addedAt: new Date().toISOString(),
    ...overrides,
  };
}

// Mock Tauri invoke
export function mockInvoke(mocks: Record<string, any>) {
  vi.mock('@tauri-apps/api/core', () => ({
    invoke: vi.fn((cmd, args) => {
      if (mocks[cmd]) {
        return Promise.resolve(
          typeof mocks[cmd] === 'function' ? mocks[cmd](args) : mocks[cmd]
        );
      }
      return Promise.reject(new Error(`Unmocked command: ${cmd}`));
    }),
  }));
}
```

---

## Deployment Architecture

### Build Pipeline

```
┌─────────────────┐
│   Source Code   │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│   TypeScript    │──▶ Type checking
│     Check       │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│   Unit Tests    │──▶ Jest/Vitest
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│   Rust Tests    │──▶ cargo test
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│   Build App     │──▶ tauri build
└────────┬────────┘
         │
    ┌────┴────┬────────────┐
    ▼         ▼            ▼
┌───────┐ ┌───────┐ ┌──────────┐
│Windows│ │ macOS │ │  Linux   │
│ .msi  │ │ .dmg  │ │ .AppImage│
│ .exe  │ │       │ │ .deb     │
└───────┘ └───────┘ └──────────┘
```

### Release Process

1. Update version in `package.json` and `Cargo.toml`
2. Update `CHANGELOG.md`
3. Create git tag `v{version}`
4. CI builds all platforms
5. Create GitHub release with artifacts
6. Update website download links

### Update Notification (Manual Download)

```typescript
// Check for updates on app start
async function checkForUpdates() {
  const currentVersion = await getAppVersion();
  const latestRelease = await fetch(
    'https://api.github.com/repos/{owner}/{repo}/releases/latest'
  ).then(r => r.json());

  if (semver.gt(latestRelease.tag_name, currentVersion)) {
    uiStore.showUpdateAvailable({
      version: latestRelease.tag_name,
      releaseNotes: latestRelease.body,
      downloadUrl: getDownloadUrl(latestRelease),
    });
  }
}
```

---

## Future Considerations

### Potential Enhancements

| Feature | Complexity | Notes |
|---------|------------|-------|
| Cloud sync | High | Optional backend, encryption |
| Mobile companion | High | React Native or Flutter |
| Collaborative editing | Very High | CRDT, real-time sync |
| AI recommendations | Medium | Local model or API |
| Import from Letterboxd | Low | CSV parsing |
| Export to various formats | Low | JSON, CSV, Markdown, HTML |

### Technical Debt Management

**Tracking:**
- Mark `// TODO:` with priority and context
- Keep `TECHNICAL_DEBT.md` for larger items
- Review quarterly

**Example:**

```typescript
// TODO(perf): Consider virtualization if >500 visible nodes causes lag
// Context: Svelte Flow handles this but may need custom optimization
// Added: 2024-01-15
// Priority: Medium
```

### Migration Strategy

For breaking changes to data format:

```rust
// Check version on load
let manifest: Manifest = load_manifest(&path)?;

match manifest.version.as_str() {
    "1.0" => migrate_v1_to_v2(&path)?,
    "2.0" => { /* current, no migration */ }
    _ => return Err(Error::UnsupportedVersion(manifest.version)),
}
```

---

## Documentation Standards

### Code Comments

```typescript
// Good: Explains WHY, not WHAT
// Skip layout animation during batch operations to prevent visual noise
const shouldAnimate = !isBatchOperation && !reducedMotion;

// Bad: Explains what code already shows
// Set shouldAnimate to true if not batch operation
const shouldAnimate = !isBatchOperation;
```

### API Documentation

```rust
/// Downloads and caches a movie poster from TMDB.
///
/// # Arguments
///
/// * `project_path` - Root directory of the project
/// * `url` - Full TMDB image URL
/// * `filename` - Target filename (e.g., "m-550.jpg")
///
/// # Returns
///
/// Relative path to cached image on success.
///
/// # Errors
///
/// * `NetworkError` - If download fails
/// * `ImageError` - If image processing fails
/// * `FileError` - If caching fails
#[tauri::command]
async fn cache_poster(
    project_path: PathBuf,
    url: String,
    filename: String
) -> Result<String, Error>
```

### Commit Messages

```
type(scope): description

[optional body]

[optional footer]
```

Types: `feat`, `fix`, `refactor`, `perf`, `test`, `docs`, `chore`

Examples:
```
feat(canvas): add level-of-detail rendering for zoom levels

Nodes now render with reduced detail when zoomed out:
- >50%: Full detail
- 25-50%: No badges
- <25%: Dots only

Closes #123
```

---

## Appendix: Decision Log

| Date | Decision | Rationale | Alternatives Considered |
|------|----------|-----------|------------------------|
| 2024-XX | Tauri over Electron | Bundle size, memory, security | Electron, Neutralino |
| 2024-XX | Svelte over React | Performance, reactivity | React, Vue, Solid |
| 2024-XX | File-based storage | User ownership, simplicity | SQLite, IndexedDB |
| 2024-XX | ELK.js for layout | Flexibility, quality | dagre, d3-force |
| 2024-XX | Command pattern | Clean undo/redo, extensibility | State snapshots |

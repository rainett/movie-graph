# Movie Graph - Frontend Specification

## Overview

A desktop application for creating visual trees/graphs of movies and TV shows, exploring connections through actors. Users start with a movie, select actors they liked, then branch to those actors' other works, building a personal discovery graph.

**Platform:** Desktop (Windows, Mac, Linux)
**Framework:** Tauri 2.0 + Svelte 5
**Key Feature:** 80s device aesthetic - the app feels like physical equipment wrapped by your computer interface

---

## Design Philosophy

### Core Principles

1. **Function defines form** - The 80s aesthetic serves functionality, never blocks it. Choose controls appropriate for each action.
2. **Restraint over ornamentation** - Bezels, screws, LEDs only when they serve a purpose. No gratuitous decoration.
3. **Tangible and distinct** - The app should feel closer to the real world than typical modern UIs.

### Visual Language

| Aspect | Approach |
|--------|----------|
| **Style** | Mashup of 80s devices — film editing, VHS, CRT, broadcast equipment. Pick what fits each function. |
| **Screen effects** | Subtle to medium: vignette, light scan lines, CRT warmth. Never distracting. |
| **Color palette** | Full color with analog warmth - VHS-style color bleed, warm tones |
| **Typography** | Period-appropriate but readable. Orbitron (Google Fonts) is the primary font. Avoid hardcore pixel/OCR fonts. |
| **Frame elements** | Bezels, screws, vents, indicator lights - only when needed |

### Sound Design

Purposeful audio feedback that makes the app feel responsive:

| Action | Sound |
|--------|-------|
| Button click | Mechanical click/relay |
| Node selection | Soft switch sound |
| Adding node | Brief "recording" sound |
| Loading | Tape mechanism whir |
| Error | Static burst |
| Successful save | Satisfying click/chunk |

**No ambient sounds** (like CRT hum) - they become distracting.

---

## Architecture: Device Board

The interface is a "board" (like a control room desk) with multiple "devices" - each a self-contained module handling specific functions.

### Board Layout

- **Background:** Subtle grid pattern
- **Devices:** Positioned on the board, can be rearranged
- **Customization:** Drag freely, snap-to-grid, or use preset layouts
- **Device states:** Active (on) or Standby (screen off but rendered)
- **Visual connections:** Feedback when selection changes (indicator lights, subtle glow)

### Devices (MVP)

#### 1. Graph Monitor

The central, largest device. Displays the movie/actor graph canvas.

**Functions:**
- Pan and zoom navigation
- Node selection (click to select)
- Node connection visualization
- Auto-layout with manual adjustment
- History strip at bottom (tape-style transport: rewind/forward for undo/redo)

**Controls:**
- Toolbar (always visible, customizable, can be hidden)
- Keyboard shortcuts for all actions
- Fullscreen mode available

#### 2. Control Terminal

Multi-mode device with physical-style mode buttons.

**Mode: Search**
- Text input for TMDB queries
- Results list with small posters
- Preview panel (poster, title, year, synopsis, cast)
- [ADD TO GRAPH] action

**Mode: Inspect**
- Selected node details (auto-switches to this mode on node click)
- Movie: poster, title, year, runtime, rating, genres, overview, cast list (top 8)
- Actor: photo, name, birth year, birthplace, biography
- **Actor suggestions:** When an actor is selected, shows up to 5 of their most popular movies that are NOT already in the graph, sorted by TMDB popularity descending. Each suggestion has a [+] add button.
- Edit controls for rating, status, notes (M7)
- Poster change button (M5)
- [ADD ACTOR] / [ADD MOVIE] wiring from cast/filmography (M4)

**Mode: Filter**
- Status toggles: Watched, Want to Watch, Watching, Dropped
- Rating range slider (1-10)
- Year range slider
- Type filter: Movies / Actors / All
- Text search within graph
- Active indicator light when filters applied
- [CLEAR FILTERS] button

#### 3. Status Bar

Thin strip, always visible. Not a full "device" - more like board labeling.

**Displays:**
- Project name
- Save indicator (saved/unsaved)
- Quick stats (node count, etc.)

### Devices (Future)

| Device | Purpose | Planned Release |
|--------|---------|-----------------|
| Project Shelf | Visual project management, file carousel | v2 |
| Settings Panel | Dedicated device for app configuration | v2 |

For MVP, use system file dialogs for project open/create and modal overlay for settings.

---

## Data Model

### File Structure

```
/ProjectName/
  manifest.json           # Project metadata, view state, positions
  data/
    movies.json           # All movie nodes (batch file)
    actors.json           # All actor nodes (batch file)
    edges.json            # All relationships
  images/
    posters/
      m-{tmdbId}.jpg      # Cached TMDB posters
      m-{tmdbId}-alt1.jpg # Alternative posters
    custom/
      m-{tmdbId}-custom.jpg  # User uploads
```

### Node Schema

```typescript
interface MovieNode {
  id: string;              // "m:{tmdbId}"
  type: "movie";
  tmdbId: number;
  title: string;
  year: number;
  status: Status;
  myRating: number | null; // 1-10
  poster: string;          // Relative path to image
  posterOptions: string[]; // Available poster paths
  notes: string;
  position: { x: number; y: number };
  addedAt: string;         // ISO timestamp
}

interface ActorNode {
  id: string;              // "a:{tmdbId}"
  type: "actor";
  tmdbId: number;
  name: string;
  photo: string;
  photoOptions: string[];
  notes: string;
  position: { x: number; y: number };
  addedAt: string;
}

type Status = "watched" | "watching" | "want_to_watch" | "dropped" | "none";

interface Edge {
  id: string;
  from: string;            // Node ID
  to: string;              // Node ID
  relationship: "acted_in" | "liked_actor" | "recommended";
  note?: string;
  createdAt: string;
}
```

---

## Graph Visualization

### Library

**Svelte Flow** - node-based canvas with built-in zoom/pan

### Auto-Layout

**ELK.js** (Eclipse Layout Kernel)

```typescript
const layoutOptions = {
  'elk.algorithm': 'layered',
  'elk.direction': 'DOWN',  // or 'RIGHT'
  'elk.spacing.nodeNode': 80,
  'elk.layered.spacing.nodeNodeBetweenLayers': 120
};
```

**Workflow:**
1. Auto-layout on first load or user request (shortcut: R)
2. User can drag nodes to manually adjust
3. Positions saved to manifest.json
4. "Re-layout" button resets to auto

### Node Rendering

Each node displays:
- Poster/photo image (thumbnail ~200px width)
- Title/name below
- Status badge (color-coded icon)
- Rating (if set)
- Selection highlight when active

### Filtering Behavior

When filters are active:
- **Matching nodes:** Full opacity
- **Non-matching nodes:** 20% opacity (dimmed)
- **Edges to dimmed nodes:** Also dimmed
- Toggle option: "Hide non-matching" vs "Dim non-matching"

---

## Interaction Patterns

### Canvas

| Action | Input |
|--------|-------|
| Pan | Drag empty space |
| Zoom | Scroll wheel |
| Select node | Click node |
| Multi-select | TBD (not MVP priority) |
| Open node detail | Double-click or Enter when selected |
| Add node | Toolbar button or / shortcut |
| Delete node | Delete key when selected (with confirmation) |
| Undo | Ctrl+Z |
| Redo | Ctrl+Shift+Z or Ctrl+Y |
| Search | / |
| Filter panel | F |
| Re-layout | R |
| Save | Ctrl+S |
| Fullscreen graph | F11 or toolbar button |

### Keyboard-First Design

- All actions accessible via keyboard
- Vim-style navigation available for power users:
  - h/j/k/l for node navigation
  - / for search
  - Enter to expand/open
  - Esc to close/deselect
- Tab navigation between devices
- Shortcut hints in tooltips

### Progressive Disclosure

- Default: Clean interface with essential controls
- Advanced: Reveal more options via settings or modifier keys
- Tutorial mode teaches features step-by-step

---

## User Flows

### Flow 1: First Launch

1. App opens - Board visible, devices in "standby" (screens off)
2. Power-on animation, devices boot up
3. Graph Monitor shows empty state: "No project loaded"
   - [NEW PROJECT] [OPEN EXISTING]
4. NEW PROJECT:
   - System folder picker
   - Creates folder structure
   - Graph Monitor ready with empty canvas
   - Control Terminal auto-switches to Search mode
5. OPEN EXISTING:
   - System folder picker
   - Validates manifest.json
   - Loads graph onto canvas

### Flow 2: Adding First Movie

1. Control Terminal in Search mode (or press /)
2. User types movie title
3. VHS tracking animation while fetching
4. Results appear as list with small posters
5. Click result - Preview expands:
   - Poster, title, year, synopsis, cast preview
   - [ADD TO GRAPH] button
6. Click ADD:
   - Node appears on Graph Monitor (center)
   - Brief "recording" animation on node
   - Control Terminal switches to Inspect mode
   - Node selected, ready for interaction

### Flow 3: Expanding Movie -> Actor -> More Movies

1. Movie node selected on canvas
2. Control Terminal (Inspect mode) shows:
   - Movie details
   - Cast list with photos
3. Click actor in cast list:
   - Actor panel expands: bio, filmography preview
   - [ADD ACTOR] button
4. Click ADD ACTOR:
   - Actor node created
   - Edge drawn: Movie -> Actor
   - Auto-layout adjusts
5. Actor now selected, filmography visible
6. Click another movie from filmography:
   - Same flow: preview -> ADD -> node + edge
7. Graph grows: Movie1 -> Actor -> Movie2

### Flow 4: Changing Poster

1. Node selected (movie or actor)
2. Control Terminal (Inspect mode) shows current poster
3. Click poster or [CHANGE POSTER] button
4. Poster picker overlay appears:
   - Row of TMDB alternatives (fetched on demand)
   - [UPLOAD CUSTOM] button
   - VHS tracking animation while loading
5. Click alternative:
   - Static transition effect
   - Poster swaps
   - Cached to images/posters/
6. Or click UPLOAD:
   - System file picker
   - Image resized/processed
   - Saved to images/custom/
7. Overlay closes, new poster on node

### Flow 5: Filtering and Discovery

1. Control Terminal -> Filter mode (button or F)
2. Filter UI appears:
   - Status toggles
   - Rating slider
   - Year range
   - Type filter
   - Text search
3. User toggles "Want to Watch" + rating 7+
4. Graph Monitor updates:
   - Matching nodes: full brightness
   - Non-matching: dimmed
5. Indicator light shows "ACTIVE"
6. [CLEAR FILTERS] restores all nodes

---

## State Management

### History System (Undo/Redo)

Command pattern implementation:

```typescript
interface Command {
  execute(): void;
  undo(): void;
  description: string;  // e.g., "Added Movie: Fight Club"
}

class HistoryManager {
  private undoStack: Command[] = [];  // max ~50 entries
  private redoStack: Command[] = [];

  execute(cmd: Command): void;
  undo(): void;
  redo(): void;
  getHistory(): string[];  // For history panel display
}
```

**Tracked actions:**
- Add node
- Delete node
- Move node
- Edit node (rating, status, notes)
- Change poster
- Add edge
- Delete edge
- Batch operations (e.g., auto-layout)

History displayed in Graph Monitor's bottom strip as tape transport.

### Auto-Save

- Save on every meaningful change (debounced, ~2 seconds after last change)
- Visual indicator: "Saving..." -> "Saved"
- Manual save always available (Ctrl+S)
- Backup system: Keep last 5 versions in hidden .backups folder

---

## Loading States

VHS aesthetic for all loading operations:

| State | Visual |
|-------|--------|
| Fetching from TMDB | "Tracking" adjustment animation - horizontal distortion lines moving vertically |
| Loading large graph | Progress bar with film sprocket holes moving |
| Caching images | Reel spinning indicator |
| Initial boot | Devices power on sequentially with brief static |

Keep loading states brief when possible. Show skeleton/placeholder for images while loading.

---

## Error States

"Malfunction" aesthetic - devices show distress appropriately:

### TMDB API Error

1. Search initiated, fetch fails
2. Control Terminal screen flickers, static burst
3. Warning light blinks amber
4. Screen shows: "SIGNAL LOST - TMDB unreachable"
   - [RETRY] button
   - [USE YOUR API KEY] link
5. If persistent: Settings modal with API key field
   - Clear instructions with link to themoviedb.org
   - [TEST KEY] button

### Corrupt Save File

1. Open project, manifest invalid
2. Affected device shows static/snow
3. Error overlay: "PROJECT DATA CORRUPTED"
   - [ATTEMPT RECOVERY] button
   - [OPEN BACKUP] button
4. Auto-backup system provides fallback

### Image Load Failure

1. Poster fails to load
2. Node shows "NO SIGNAL" placeholder (TV test pattern)
3. Retry on next view, or manual retry in Inspector

### Network Offline

1. Detect offline state
2. Indicator light changes (red)
3. Search disabled with message: "OFFLINE - Search unavailable"
4. All local operations continue normally
5. Auto-retry when connection restored

---

## Tutorial Mode

Accessed from Settings or first-launch prompt. Separate mode, not default.

### Structure

"TRAINING MODE" label visible on board. Lessons completed independently, progress saved.

### Lessons

**Lesson 1: Navigation**
- Dummy graph loaded
- Prompts: "Pan by dragging empty space"
- "Zoom with scroll wheel"
- Completes when user performs actions

**Lesson 2: Adding Movies**
- Empty canvas
- "Press / to search"
- Guides through search -> add flow

**Lesson 3: Building Connections**
- Single movie node
- "Click to select, find actors in Inspector"
- Guides through expansion flow

**Lesson 4: Organization**
- Cluttered graph
- "Use filters to focus"
- "Press R to auto-layout"

**Lesson 5: Customization**
- "Change a poster"
- "Rate a movie"
- "Add notes"

Exit anytime with "Exit Training Mode" button.

---

## Poster Handling

### Image Sources

1. **TMDB posters** - Multiple options per movie (fetched on demand)
2. **Custom uploads** - User's own images

### Processing Pipeline

```
User selects poster
       |
       v
Download/receive image
       |
       v
Resize to standard dimensions:
  - Thumbnail: 200px width (for canvas)
  - Full: 500px width (for detail view)
       |
       v
Compress (JPEG, ~80% quality)
       |
       v
Save to images/posters/ or images/custom/
       |
       v
Update node.poster reference
```

### Lazy Loading

- Fetch posters on demand, not upfront
- Cache all viewed posters locally
- Offline: Use cached images, show placeholder for uncached

---

## TMDB Integration

### API Key Strategy

**Hybrid approach:**
1. Ship with default embedded key
2. If rate-limited or errors occur, prompt user to get their own (free) key
3. Settings allow key override anytime
4. Clear, simple instructions: "Get free key at themoviedb.org/settings/api"

### Endpoints Used

| Purpose | Endpoint |
|---------|----------|
| Search movies | `/search/movie` |
| Search people | `/search/person` |
| Movie details | `/movie/{id}` |
| Movie credits | `/movie/{id}/credits` |
| Person details | `/person/{id}` |
| Person filmography | `/person/{id}/movie_credits` |
| Movie images | `/movie/{id}/images` |
| Person images | `/person/{id}/images` |

### Caching Strategy

- Cache all API responses locally (JSON files or IndexedDB equivalent)
- Cache duration: 7 days for movie/person data
- Images: Cached permanently once downloaded
- Offline mode uses cached data only

---

## Accessibility

Not primary priority for MVP, but foundational support:

| Feature | Implementation |
|---------|----------------|
| Keyboard navigation | Full support (see Interaction Patterns) |
| Screen reader | Semantic HTML, ARIA labels on controls |
| High contrast | Settings toggle (future) |
| Reduced motion | Settings toggle - disables animations |
| Focus indicators | Clear, visible focus rings on all interactive elements |

---

## Tech Stack Summary

| Layer | Technology |
|-------|------------|
| Desktop shell | Tauri 2.0 |
| Frontend framework | Svelte 5 |
| Language | TypeScript |
| Graph visualization | Svelte Flow |
| Auto-layout | ELK.js |
| Styling | Tailwind CSS |
| State management | Svelte stores + custom HistoryManager |
| File I/O | @tauri-apps/plugin-fs |
| Image processing | @tauri-apps/plugin-image |
| Search (local) | fuse.js (fuzzy search within graph) |
| HTTP client | fetch (Tauri's) |

---

## Component Hierarchy

```
App
├── Board (main container)
│   ├── BoardBackground (grid pattern)
│   ├── DeviceContainer (manages device positions)
│   │   ├── GraphMonitor
│   │   │   ├── Toolbar
│   │   │   ├── Canvas (Svelte Flow)
│   │   │   │   ├── MovieNode (custom node)
│   │   │   │   ├── ActorNode (custom node)
│   │   │   │   └── ConnectionEdge (custom edge)
│   │   │   └── HistoryStrip (tape transport)
│   │   └── ControlTerminal
│   │       ├── ModeSelector (physical buttons)
│   │       ├── SearchMode
│   │       │   ├── SearchInput
│   │       │   ├── ResultsList
│   │       │   └── PreviewPanel
│   │       ├── InspectMode
│   │       │   ├── NodeDetails
│   │       │   ├── CastList / Filmography
│   │       │   └── EditControls
│   │       └── FilterMode
│   │           ├── StatusToggles
│   │           ├── RangeSliders
│   │           └── TypeFilter
│   └── StatusBar
├── Overlays
│   ├── PosterPicker
│   ├── SettingsModal
│   ├── ConfirmDialog
│   └── ErrorOverlay
└── TutorialLayer (when active)
```

---

## Development Phases

### Phase 1: Foundation
- Tauri + Svelte project scaffold
- Board layout system
- Device container with positioning
- Basic file I/O (create/open project)

### Phase 2: Graph Core
- Svelte Flow integration
- Custom node components (static)
- Basic pan/zoom/select
- Node position persistence

### Phase 3: TMDB Integration
- API client with caching
- Search functionality
- Movie/actor data fetching
- Control Terminal Search mode

### Phase 4: Graph Editing
- Add nodes from search
- Create edges (movie <-> actor)
- Delete nodes/edges
- Control Terminal Inspect mode

### Phase 5: Auto-Layout
- ELK.js integration
- Layout triggers (auto, manual)
- Position saving after adjustments

### Phase 6: Images
- Poster fetching and caching
- Thumbnail generation
- Alternative poster selection
- Custom upload

### Phase 7: History System
- Command pattern implementation
- Undo/redo functionality
- History strip UI

### Phase 8: Filtering
- Filter state management
- Filter mode UI
- Canvas dimming/hiding logic

### Phase 9: Polish - Core
- Status tracking (watched, etc.)
- Ratings
- Notes
- Keyboard shortcuts complete

### Phase 10: Polish - Experience
- Sound design integration
- Loading animations (VHS style)
- Error states (malfunction aesthetic)
- Tutorial mode

### Phase 11: Advanced
- Multiple project support
- Settings panel device
- Export/sharing features
- Accessibility improvements

---

## Open Questions for Future Planning

1. **Data export formats** - JSON, CSV, markdown, image?
2. **Sharing mechanism** - Export file, generate image, shareable link (needs backend)?
3. **TV show handling** - Seasons/episodes as nodes, or just show-level?
4. **Performance limits** - Max nodes before virtualization needed?
5. **Sync/backup** - Cloud integration later? (Google Drive, etc.)

# Movie Graph - Documentation

## Project Summary

**Movie Graph** is a desktop application for creating visual trees/graphs of movies and TV shows, exploring connections through actors. Start with a movie you love, pick actors you enjoyed, explore their filmographies, and build a personal discovery map.

### Key Characteristics

| Aspect | Choice |
|--------|--------|
| **Platform** | Desktop (Windows, Mac, Linux) |
| **Tech Stack** | Tauri 2.0 + Svelte 5 + TypeScript + Rust |
| **Data Storage** | Local files (user-owned, like Obsidian) |
| **Data Source** | TMDB (The Movie Database) |
| **Visual Style** | 80s device aesthetic (function-first retro) |
| **Target Scale** | Thousands of nodes |
| **Extensibility** | Plugin architecture (v1.5+) |
| **Localization** | Multi-language support |

### Core Features (v1.0)

- Search movies and actors via TMDB
- Build visual graphs: Movie → Actor → Movie → ...
- Auto-layout with manual adjustment
- Track watch status (watched, want to watch, etc.)
- Personal ratings and notes
- Poster customization
- Full undo/redo history
- Filter and explore your graph
- Offline-capable after initial data fetch

---

## Documentation Index

| Document | Purpose | Audience |
|----------|---------|----------|
| [Frontend Spec](frontend.md) | UI architecture, components, user flows, interactions | Frontend development |
| [Backend Spec](backend-tauri.md) | Rust commands, services, data models, API integration | Backend development |
| [Architect Guide](architect.md) | System design, decisions, patterns, module boundaries | Technical planning |
| [Design Spec](design.md) | Visual language, components, colors, typography, assets | UI/UX implementation |
| [Manager Guide](manager.md) | Milestones, risks, planning, checklists, metrics | Project management |

---

## Quick Reference

### Tech Stack

```
┌─────────────────────────────────────────┐
│            Presentation                  │
│  Svelte 5 + Svelte Flow + Tailwind CSS  │
├─────────────────────────────────────────┤
│              State                       │
│  Svelte Stores + Command Pattern        │
├─────────────────────────────────────────┤
│             Backend                      │
│    Tauri 2.0 + Rust + tokio             │
├─────────────────────────────────────────┤
│            External                      │
│   TMDB API + Local File System          │
└─────────────────────────────────────────┘
```

### Project File Structure

```
/UserProject/
├── manifest.json        # Project metadata
├── data/
│   ├── movies.json      # Movie nodes
│   ├── actors.json      # Actor nodes
│   └── edges.json       # Relationships
└── images/
    ├── posters/         # Cached TMDB posters
    └── custom/          # User uploads
```

### UI Architecture

```
Board (workspace)
├── Graph Monitor (main canvas)
│   ├── Canvas (Svelte Flow)
│   └── History Strip (undo/redo transport)
├── Control Terminal (multi-mode)
│   ├── Search Mode
│   ├── Inspect Mode
│   └── Filter Mode
└── Status Bar
```

### Key User Flow

```
Search Movie → Add to Graph → Select Node → View Cast
                                              ↓
                              Add Actor → View Filmography
                                              ↓
                              Add Another Movie → Repeat
```

---

## Milestones Overview

| # | Name | Duration | Focus |
|---|------|----------|-------|
| M1 | Foundation | 2 weeks | Tauri scaffold, file I/O, project management |
| M2 | Graph Core | 2 weeks | Svelte Flow, custom nodes, pan/zoom |
| M3 | TMDB Integration | 2 weeks | API client, caching, search |
| M4 | Graph Building | 2 weeks | Add nodes from search, edges, auto-layout |
| M5 | Images | 2 weeks | Poster download, cache, upload |
| M6 | History System | 2 weeks | Undo/redo, command pattern |
| M7 | Filtering & Status | 2 weeks | Filters, watch status, ratings |
| M8 | Polish & UX | 3 weeks | 80s styling, sounds, loading/error states |
| M9 | Tutorial | 1 week | Onboarding, lessons |
| M10 | Release Prep | 2 weeks | Testing, signing, distribution |

**Total estimated duration:** ~20 weeks

---

## Design Quick Reference

### Color Palette

| Role | Color | Hex |
|------|-------|-----|
| Board Background | Deep charcoal | `#1a1a1e` |
| Device Body | Warm dark gray | `#2d2d32` |
| Screen Background | Near-black blue | `#0f1218` |
| Primary Text | Warm white | `#e8e6e3` |
| Primary Accent | Warm orange | `#e07850` |
| Success/Watched | Muted green | `#50c070` |
| Warning/Want | Amber | `#e0a850` |

### Typography

| Use | Font |
|-----|------|
| UI Text | Eurostile |
| Mono/Counters | Share Tech Mono |

### Design Principles

1. **Function defines form** - 80s aesthetic serves usability
2. **Restraint** - Decorative elements only when purposeful
3. **Tangible** - App feels like physical equipment
4. **Responsive** - Every action has feedback (visual + audio)

---

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Desktop framework | Tauri 2.0 | Small bundle, memory efficient, secure |
| Frontend framework | Svelte 5 | Performance, true reactivity, small bundle |
| Data storage | Local JSON files | User ownership, no lock-in, offline |
| Graph library | Svelte Flow | Built-in zoom/pan, custom nodes, performant |
| Layout engine | ELK.js | Flexible, quality layouts |
| History system | Command pattern | Clean undo/redo, extensible |
| API key | Hybrid (embedded + user) | Zero friction + graceful fallback |

---

## Getting Started

### Prerequisites

- Node.js 18+
- Rust 1.70+
- Tauri CLI (`cargo install tauri-cli`)

### Development

```bash
# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

### Project Setup (New Feature)

1. Check scope in [Manager Guide](manager.md)
2. Review relevant spec ([Frontend](frontend.md) or [Backend](backend-tauri.md))
3. Check design requirements in [Design Spec](design.md)
4. Review architecture patterns in [Architect Guide](architect.md)
5. Follow checklists in [Manager Guide](manager.md)

---

## Document Maintenance

| When | Action |
|------|--------|
| Architecture decision made | Add to [Architect Guide](architect.md) decision log |
| New component designed | Update [Design Spec](design.md) |
| Milestone completed | Update [Manager Guide](manager.md) progress |
| Scope change | Update relevant spec + manager guide |
| Bug pattern identified | Add to relevant troubleshooting section |

---

## Links

- **TMDB API:** https://developers.themoviedb.org/3
- **Tauri Docs:** https://tauri.app/v2/
- **Svelte Docs:** https://svelte.dev/docs
- **Svelte Flow:** https://svelteflow.dev/
- **ELK.js:** https://github.com/kieler/elkjs

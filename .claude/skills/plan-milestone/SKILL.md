---
name: plan-milestone
description: Decompose a milestone into atomic tasks mapped to the right skills and agents before starting work
argument-hint: <N>
---

# Milestone Task Planner

Break a milestone into small, atomic units before starting implementation. Each unit should be small enough that the right tool is obvious.

## Input

Milestone: $ARGUMENTS

## Steps

### 1. Read the milestone

- Read `docs/manager.md` for the full deliverables list
- Read the current milestone checklist in `CLAUDE.md`
- Read relevant sections of `docs/frontend.md` and `docs/backend-tauri.md` for spec details

### 2. Decompose into atomic units

Each unit must be **one logical change** — one command, one component, one model, one service method. If a unit touches more than ~3 files or has more than one distinct concern, split it further.

**Atomic = a unit that can be verified independently by `cargo check` or `npm run check`.**

### 3. Map each unit to a tool

| Unit type | Tool | Skip when |
|-----------|------|-----------|
| New Tauri IPC command (simple CRUD) | `/tauri-command` | Editing existing command |
| New Tauri IPC command (complex async, image processing, lifetimes) | `@rust-expert` | — |
| New full device/overlay/canvas component | `/svelte-component` → `@design-reviewer` | Component is < ~80 lines or styling is copied from adjacent code |
| New sub-widget inside existing device | Direct implementation | — |
| UI component with complex reactivity / SvelteFlow integration | `@svelte-expert` | Problem is solvable by reading existing patterns |
| Edit to existing component | Direct implementation | — |
| New store or TS utility | Direct implementation | — |
| Quick codebase lookup | Direct Glob/Read | Always — `@explorer` is slower than direct search on this small project |
| Pre-milestone doc verification | `/check-docs` | Milestone is purely additive with no spec ambiguity |

**Default rule:** if you can write a unit confidently without referencing a skill template, write it directly. Skills add value when the template encodes project-specific patterns you'd otherwise have to re-derive.

### 4. Sequence and flag dependencies

Order units so that:
- Data models and types come first (Rust models → TS types)
- Backend commands before frontend wiring
- Stores before components that consume them
- New components before design review

Mark dependencies explicitly: **"requires: unit X"**

### 5. Output

Produce a numbered task list:

```
## M{N} Task Plan

### Pre-work
- [ ] /check-docs backend — verify spec matches current implementation

### Backend
- [ ] 1. [Unit name] — tool: `/tauri-command` or `@rust-expert` or direct
      Files: src-tauri/src/...
      Depends on: —

### Frontend
- [ ] 2. [Unit name] — tool: `/svelte-component` or direct
      Files: src/lib/...
      Depends on: unit 1

- [ ] 3. Design review — tool: `@design-reviewer`
      Depends on: unit 2

### Close-out
- [ ] /sync-state {N}
```

## Rules

- **Never group "add image caching + settings modal + edge labels" into one unit.** Each is separate.
- A unit that needs both Rust + TypeScript changes is fine if they're tightly coupled (same command + its TS wrapper). That's one atomic change.
- If a unit is genuinely simple (under 20 lines, one file), note it as "direct — no skill needed" to avoid overhead.
- Re-run this plan if scope changes mid-milestone.

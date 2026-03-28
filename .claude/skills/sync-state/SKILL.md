---
name: sync-state
description: Update CLAUDE.md and MEMORY.md to reflect current implementation state. Run at milestone boundaries.
argument-hint: [milestone-number]
---

# Sync Project State

Update project documentation to reflect actual implementation state. **Run this at the end of each milestone** or when significant changes have been made.

## Steps

### 1. Audit Current Implementation

Scan actual code to determine what exists:

```bash
# Frontend components
ls -la src/lib/components/**/*.svelte 2>/dev/null || true

# Stores
ls -la src/lib/stores/*.ts 2>/dev/null || true

# Rust commands
grep -r "#\[tauri::command\]" src-tauri/src/commands/ 2>/dev/null | head -20

# Rust services
ls -la src-tauri/src/services/*.rs 2>/dev/null || true
```

### 2. Update CLAUDE.md

Update the following sections in `CLAUDE.md`:

- **Project Structure** — list actual files that exist now
- **Current Focus** — mark completed milestones, set next milestone
- **Installed Dependencies** — reflect actual Cargo.toml and package.json
- **Key Patterns** — update with actual current patterns from the code

### 3. Update MEMORY.md

Update `~/.claude/projects/.../memory/MEMORY.md`:

- Move completed milestone sections to "COMPLETE" status
- Add any new key decisions or gotchas discovered
- Update "Key File Locations" if new important files were added
- Add any API/library notes learned during implementation

### 4. Check Docs Alignment

Review if `docs/*.md` diverged from implementation:

- `docs/frontend.md` vs actual component behavior
- `docs/backend-tauri.md` vs actual command signatures
- `docs/manager.md` milestone checklists vs reality

If significant divergence, update the docs or note discrepancies.

### 5. Suggest Commit

If changes were made, suggest:

```bash
git add CLAUDE.md
git add docs/  # if updated
git status
git commit -m "docs(M$ARGUMENTS): sync project state with implementation"
```

## Input

Milestone: $ARGUMENTS (if not specified, determine from CLAUDE.md current focus)

## Output

Provide a summary:
- What was updated in CLAUDE.md
- What was updated in MEMORY.md
- Any doc discrepancies found
- Suggested next steps

---
name: milestone-status
description: Check progress against milestone deliverables and suggest next steps
argument-hint: [milestone-number]
---

# Milestone Status Check

Review progress against milestone deliverables from `docs/manager.md`.

## Input

Milestone: $ARGUMENTS (or current from CLAUDE.md if not specified)

## Process

### 1. Load Context

```bash
# Read milestone spec
cat docs/manager.md | grep -A 50 "Milestone $ARGUMENTS"

# Read current state
cat CLAUDE.md | grep -A 30 "Current Focus"
```

### 2. Check Deliverables

For each deliverable in the milestone, verify:

**Frontend deliverables:**
```bash
ls -la src/lib/components/**/*.svelte
ls -la src/lib/stores/*.ts
```

**Backend deliverables:**
```bash
grep -r "#\[tauri::command\]" src-tauri/src/commands/
ls -la src-tauri/src/services/*.rs
```

**Types:**
```bash
ls -la src/lib/types/*.ts
ls -la src-tauri/src/models/*.rs
```

### 3. Run Acceptance Checks

```bash
# Rust compiles
cd src-tauri && cargo check

# TypeScript passes
npm run check
```

### 4. Output Report

```markdown
## Milestone [N]: [Name]

### Progress: X/Y deliverables complete

### Completed
- [x] Deliverable 1 — location/evidence
- [x] Deliverable 2 — location/evidence

### In Progress
- [ ] Deliverable 3 — X% done, remaining: ...

### Not Started
- [ ] Deliverable 4

### Acceptance Criteria
- [x] Criterion 1 — passes because...
- [ ] Criterion 2 — not yet met

### Build Status
- Rust: ✅ cargo check passes (N warnings)
- TypeScript: ✅ npm run check passes

### Recommended Next Steps
1. Complete [specific deliverable]
2. Fix [specific issue]
3. Run /sync-state to update docs
```

### 5. If Milestone Complete

When all deliverables done:

1. **Sync state:**
   ```
   /sync-state [milestone-number]
   ```

2. **Commit:**
   ```bash
   git add -A
   git status
   git commit -m "feat(M[N]): complete [milestone name]

   - [key deliverable 1]
   - [key deliverable 2]
   - [key deliverable 3]"
   ```

3. **Update focus to next milestone in CLAUDE.md**

## Quick Status Commands

```bash
# Count components
find src/lib/components -name "*.svelte" | wc -l

# Count Rust commands
grep -c "#\[tauri::command\]" src-tauri/src/commands/*.rs

# Check for uncommitted changes
git status --short
```

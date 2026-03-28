---
name: check-docs
description: Verify implementation matches documentation, flag discrepancies, suggest fixes
argument-hint: <area> (frontend | backend | design | all)
---

# Documentation Compliance Check

Compare current implementation against project documentation. **Run before starting a new milestone** to ensure docs are accurate.

## Input

Area to check: $ARGUMENTS

## Process

### If "frontend" or "ui"

Compare against `docs/frontend.md`:

1. **Component hierarchy** — Does the actual component tree match the spec?
   ```bash
   find src/lib/components -name "*.svelte" -type f
   ```

2. **User flows** — Are the documented flows implemented?
   - Search → Results → Preview → Add
   - Node click → Inspect mode
   - Filter mode toggles

3. **Store structure** — Do stores match the documented patterns?
   ```bash
   cat src/lib/stores/*.ts | head -50
   ```

### If "backend" or "rust" or "tauri"

Compare against `docs/backend-tauri.md`:

1. **Commands** — Are all documented commands implemented?
   ```bash
   grep -r "#\[tauri::command\]" src-tauri/src/commands/ | grep "pub async fn"
   ```

2. **Models** — Do data structures match?
   ```bash
   cat src-tauri/src/models/*.rs | grep "pub struct\|pub enum"
   ```

3. **Services** — Are services implemented as documented?

4. **Auth method** — Doc says `api_key` query param, code uses Bearer token. Flag this.

### If "design" or "styling"

Compare against `docs/design.md`:

1. **Color palette** — Grep for hardcoded colors that should use variables
   ```bash
   grep -r "#[0-9a-fA-F]\{6\}" src/lib/components/ --include="*.svelte"
   ```

2. **Typography** — Check font-family usage
3. **Device frame consistency** — Compare device components

### If "all"

Run all checks above sequentially.

## Output Format

For each area checked:

```markdown
## [Area] Compliance

### ✅ Matches Spec
- Feature X — implemented as documented
- Feature Y — implemented as documented

### ⚠️ Discrepancies
| Location | Doc Says | Code Does | Severity | Fix |
|----------|----------|-----------|----------|-----|
| `tmdb_client.rs` | api_key param | Bearer token | Minor | Update doc |
| `frontend.md` | No actor suggestions | Has suggestions | Minor | Update doc |

### ❌ Missing
- [ ] Feature Z documented but not implemented

### Recommendations
1. Update `docs/backend-tauri.md` to reflect Bearer auth
2. Add actor suggestions to `docs/frontend.md` Inspect mode section
```

## When to Run

- **Before starting a new milestone** — ensure you're building against accurate specs
- **After completing a milestone** — verify implementation matches intent
- **When something feels wrong** — check if docs drifted from code

## Quick Fixes

If discrepancy is in docs (code is correct):
```bash
# Edit the doc file
# Then commit
git add docs/
git commit -m "docs: sync [area] with implementation"
```

If discrepancy is in code (docs are correct):
- Fix the code to match spec
- Or discuss with user if spec should change

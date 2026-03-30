---
name: svelte-component
description: Create a Svelte 5 component with 80s device aesthetic styling
argument-hint: <ComponentName> [description]
---

# Svelte Component Generator

## When to Use This Skill

**Use `/svelte-component`** when creating a **new standalone component** that:
- Is a full device frame, overlay, or canvas node (> ~80 lines)
- Needs careful 80s aesthetic that requires checking design.md
- Will be reused across multiple parent components

**Skip and write directly** when:
- The component is a sub-strip or small widget inside one existing device (e.g., HistoryStrip inside GraphMonitor — < 100 lines, no design ambiguity)
- Styling can be copied verbatim from an adjacent component in the same file
- The component is purely structural with no visual design decisions

Always follow `/svelte-component` with `@design-reviewer` for full device/overlay components.

Create a Svelte 5 component following current project patterns.

## Input

Component: $ARGUMENTS

## Steps

### 1. Determine Location

Based on component type:

| Type | Location | Examples |
|------|----------|----------|
| Board/layout | `src/lib/components/board/` | Board.svelte |
| Device frames | `src/lib/components/devices/` | GraphMonitor.svelte, ControlTerminal.svelte |
| SvelteFlow nodes | `src/lib/components/canvas/` | MovieNode.svelte, ActorNode.svelte |
| UI controls | `src/lib/components/controls/` | (future) |
| Modals/overlays | `src/lib/components/overlays/` | (future) |

### 2. Create Component

Use Svelte 5 runes pattern:

```svelte
<script lang="ts">
  import { someStore } from '$lib/stores/some';
  import type { SomeType } from '$lib/types/some';

  // Props
  let { title, count = 0 }: { title: string; count?: number } = $props();

  // Local state
  let items = $state<SomeType[]>([]);
  let loading = $state(false);

  // Derived
  let total = $derived(items.length);

  // Effects (use sparingly)
  $effect(() => {
    // React to changes
  });

  // Functions
  function handleClick() {
    // ...
  }
</script>

<div class="component-root">
  <!-- Content -->
</div>

<style>
.component-root {
  /* Use CSS variables from design tokens */
}
</style>
```

### 3. Apply 80s Device Styling

**Design tokens** (from `src/app.css`):

```css
/* Backgrounds */
--color-board-bg: #1a1a1e;
--color-device-body: #2d2d32;
--color-device-bezel: #252528;
--color-screen-bg: #0a0e14;

/* Text */
--color-text-primary: #f0f0f0;
--color-text-secondary: #a0a0a8;
--color-text-screen: #c8d8e8;
--color-text-disabled: #505058;

/* Accents */
--color-accent-primary: #e07850;    /* Warm orange */
--color-accent-secondary: #50a8e0;  /* Cool blue */

/* LEDs */
--color-led-green: #30ff50;
--color-led-amber: #ffaa30;
--color-led-off: #1a1a20;

/* Fonts */
--font-ui: 'Orbitron', system-ui, sans-serif;
--font-mono: 'JetBrains Mono', monospace;
```

**Device frame pattern** (from GraphMonitor/ControlTerminal):

```svelte
<div class="device">
  <div class="device-header">
    <span class="device-title">TITLE</span>
    <div class="device-indicators">
      <div class="led led-green"></div>
    </div>
  </div>
  <div class="device-screen">
    <!-- Content -->
  </div>
</div>

<style>
.device {
  background: var(--color-device-body);
  border: 2px solid var(--color-device-bezel);
  border-radius: 8px;
  box-shadow: var(--shadow-device);
}
.device-screen {
  background: var(--color-screen-bg);
  box-shadow: inset 0 0 20px rgba(0, 0, 0, 0.5);
}
</style>
```

### 4. Add Types (if needed)

Create in `src/lib/types/`:

```typescript
export type NewComponentData = {
  id: string;
  // ...
};
```

### 5. Verify

After saving, hooks will auto-run `npm run check`.

### 6. Design Review

**After creating/modifying significant UI components, invoke `@design-reviewer`:**

```
@design-reviewer Review ComponentName.svelte against docs/design.md
```

Check for:
- Color palette compliance
- Typography (Orbitron for UI, JetBrains Mono for data)
- Device frame consistency
- CRT/80s aesthetic effects where appropriate
- Reduced motion support (`@media (prefers-reduced-motion)`)

## Current Component Examples

**Device component** — `src/lib/components/devices/ControlTerminal.svelte`
**Canvas node** — `src/lib/components/canvas/MovieNode.svelte`
**Board layout** — `src/lib/components/board/Board.svelte`

Read these for current patterns before creating new components.

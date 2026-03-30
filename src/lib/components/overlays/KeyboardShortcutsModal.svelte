<script lang="ts">
  let { onclose }: { onclose: () => void } = $props();

  function onKeyDown(e: KeyboardEvent) {
    if (e.key === 'Escape') onclose();
  }

  function onOverlayClick(e: MouseEvent) {
    if (e.target === e.currentTarget) onclose();
  }

  const groups: Array<{ label: string; shortcuts: Array<{ keys: string; action: string }> }> = [
    {
      label: 'NAVIGATION',
      shortcuts: [
        { keys: 'Tab', action: 'Cycle SEARCH → INSPECT → FILTER' },
        { keys: 'Escape', action: 'Deselect node / Close modal' },
      ],
    },
    {
      label: 'GRAPH',
      shortcuts: [
        { keys: 'Delete / Backspace', action: 'Remove selected node' },
        { keys: 'Escape', action: 'Deselect node' },
      ],
    },
    {
      label: 'HISTORY',
      shortcuts: [
        { keys: 'Ctrl+Z', action: 'Undo' },
        { keys: 'Ctrl+Shift+Z', action: 'Redo' },
        { keys: 'Ctrl+Y', action: 'Redo' },
      ],
    },
    {
      label: 'SYSTEM',
      shortcuts: [
        { keys: 'Ctrl+S', action: 'Save project' },
        { keys: '? / F1', action: 'Show keyboard shortcuts' },
      ],
    },
  ];
</script>

<svelte:window onkeydown={onKeyDown} />

<div
  class="modal-overlay"
  role="dialog"
  aria-modal="true"
  aria-label="Keyboard Shortcuts"
  tabindex="-1"
  onclick={onOverlayClick}
  onkeydown={onKeyDown}
>
  <div class="modal">
    <div class="modal-header">
      <span class="modal-title">KEYBOARD SHORTCUTS</span>
      <button class="close-btn" onclick={onclose} aria-label="Close">×</button>
    </div>

    <div class="modal-body">
      {#each groups as group}
        <div class="shortcut-group">
          <div class="group-label">{group.label}</div>
          {#each group.shortcuts as s}
            <div class="shortcut-row">
              <kbd class="kbd">{s.keys}</kbd>
              <span class="shortcut-action">{s.action}</span>
            </div>
          {/each}
        </div>
      {/each}
    </div>

    <div class="modal-footer">
      <span class="footer-hint">Press <kbd class="kbd-sm">Esc</kbd> or click outside to close</span>
    </div>
  </div>
</div>

<style>
.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.7);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 100;
}

.modal {
  background: var(--color-device-body);
  border: 2px solid var(--color-device-bezel);
  border-radius: 6px;
  width: 420px;
  max-height: 85vh;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.modal-header {
  display: flex;
  align-items: center;
  height: 36px;
  padding: 0 14px;
  background: linear-gradient(180deg, #3a3a40 0%, #2d2d32 100%);
  border-bottom: 1px solid #1a1a20;
  flex-shrink: 0;
}

.modal-title {
  color: var(--color-text-secondary);
  font-family: var(--font-ui);
  font-size: 11px;
  font-weight: 500;
  letter-spacing: 0.2em;
}

.close-btn {
  background: none;
  border: none;
  color: var(--color-text-disabled);
  cursor: pointer;
  font-size: 18px;
  line-height: 1;
  margin-left: auto;
  padding: 0 2px;
  transition: color var(--duration-fast);
}
.close-btn:hover { color: var(--color-text-secondary); }

.modal-body {
  flex: 1;
  overflow-y: auto;
  padding: 6px 0;
  scrollbar-width: thin;
  scrollbar-color: #2a2a38 transparent;
}

.shortcut-group {
  padding: 10px 16px;
  border-bottom: 1px solid #1a1a20;
}

.shortcut-group:last-child { border-bottom: none; }

.group-label {
  color: var(--color-text-disabled);
  font-family: var(--font-ui);
  font-size: 9px;
  letter-spacing: 0.2em;
  margin-bottom: 8px;
}

.shortcut-row {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 4px 0;
}

.kbd {
  background: rgba(255, 255, 255, 0.06);
  border: 1px solid #3a3a50;
  border-radius: 3px;
  color: var(--color-text-screen);
  font-family: var(--font-mono);
  font-size: 10px;
  letter-spacing: 0.04em;
  padding: 2px 8px;
  white-space: nowrap;
  flex-shrink: 0;
  min-width: 140px;
}

.shortcut-action {
  color: var(--color-text-secondary);
  font-family: var(--font-mono);
  font-size: 11px;
}

.modal-footer {
  border-top: 1px solid #1a1a20;
  padding: 8px 16px;
  flex-shrink: 0;
}

.footer-hint {
  color: var(--color-text-disabled);
  font-family: var(--font-mono);
  font-size: 9px;
  letter-spacing: 0.05em;
  opacity: 0.6;
}

.kbd-sm {
  background: rgba(255, 255, 255, 0.06);
  border: 1px solid #3a3a50;
  border-radius: 2px;
  font-family: var(--font-mono);
  font-size: 9px;
  padding: 1px 5px;
}
</style>

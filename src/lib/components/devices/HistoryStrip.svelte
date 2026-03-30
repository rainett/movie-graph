<script lang="ts">
  import { historyStore, canUndo, canRedo } from '$lib/stores/history';
  import { playSound } from '$lib/services/sound';

  const MAX_VISIBLE = 6;

  const recentHistory = $derived(
    $historyStore.undoStack.slice(-MAX_VISIBLE).reverse()
  );
</script>

<div class="history-strip" role="toolbar" aria-label="History">
  <button
    class="transport-btn"
    disabled={!$canUndo}
    onclick={() => { playSound('undo'); historyStore.undo(); }}
    title="Undo (Ctrl+Z)"
    aria-label="Undo"
  >◀◀</button>

  <div class="history-tape" role="log" aria-label="Action history" aria-live="polite">
    {#if recentHistory.length === 0}
      <span class="tape-empty">── NO HISTORY ──</span>
    {:else}
      {#each recentHistory as cmd, i (i)}
        <span class="tape-entry" class:tape-current={i === 0}>{cmd.description}</span>
        {#if i < recentHistory.length - 1}
          <span class="tape-sep" aria-hidden="true">·</span>
        {/if}
      {/each}
    {/if}
  </div>

  <button
    class="transport-btn"
    disabled={!$canRedo}
    onclick={() => { playSound('undo'); historyStore.redo(); }}
    title="Redo (Ctrl+Shift+Z)"
    aria-label="Redo"
  >▶▶</button>
</div>

<style>
.history-strip {
  display: flex;
  align-items: center;
  height: 28px;
  padding: 0 8px;
  gap: 6px;
  background: linear-gradient(180deg, #1a1a22 0%, #141418 100%);
  border-top: 1px solid #252530;
  flex-shrink: 0;
}

.transport-btn {
  background: rgba(255, 255, 255, 0.04);
  border: 1px solid #2a2a38;
  border-radius: 2px;
  color: var(--color-text-disabled);
  cursor: pointer;
  font-family: var(--font-mono);
  font-size: 8px;
  height: 18px;
  padding: 0 7px;
  flex-shrink: 0;
  letter-spacing: 0.05em;
  transition: color var(--duration-fast), background var(--duration-fast), border-color var(--duration-fast);
}

.transport-btn:not(:disabled):hover {
  background: rgba(224, 120, 80, 0.12);
  border-color: rgba(224, 120, 80, 0.4);
  color: var(--color-accent-primary);
}

.transport-btn:disabled {
  opacity: 0.25;
  cursor: default;
}

.history-tape {
  flex: 1;
  display: flex;
  align-items: center;
  gap: 5px;
  overflow: hidden;
  min-width: 0;
}

.tape-empty {
  color: var(--color-text-disabled);
  font-family: var(--font-mono);
  font-size: 9px;
  letter-spacing: 0.1em;
  width: 100%;
  text-align: center;
  opacity: 0.4;
}

.tape-entry {
  color: var(--color-text-disabled);
  font-family: var(--font-mono);
  font-size: 9px;
  letter-spacing: 0.04em;
  white-space: nowrap;
  flex-shrink: 0;
  opacity: 0.35;
  transition: opacity var(--duration-fast), color var(--duration-fast);
}

.tape-entry.tape-current {
  color: var(--color-text-secondary);
  opacity: 1;
}

.tape-sep {
  color: var(--color-text-disabled);
  font-size: 8px;
  opacity: 0.2;
  flex-shrink: 0;
}
</style>

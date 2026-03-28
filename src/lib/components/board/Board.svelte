<script lang="ts">
  import GraphMonitor from '../devices/GraphMonitor.svelte';
  import ControlTerminal from '../devices/ControlTerminal.svelte';
  import { movieCount, actorCount } from '$lib/stores/graph';
</script>

<div class="board">
  <div class="board-devices">
    <GraphMonitor />
    <ControlTerminal />
  </div>
  <div class="status-bar">
    <span class="status-project">MOVIE GRAPH</span>
    <span class="status-sep">·</span>
    <span class="status-stats">{$movieCount} MOVIES · {$actorCount} ACTORS</span>
    <span class="status-save">—</span>
  </div>
</div>

<style>
.board {
  width: 100vw;
  height: 100vh;
  background-color: var(--color-board-bg);
  background-image:
    linear-gradient(rgba(255, 255, 255, 0.018) 1px, transparent 1px),
    linear-gradient(90deg, rgba(255, 255, 255, 0.018) 1px, transparent 1px);
  background-size: 32px 32px;
  display: flex;
  flex-direction: column;
  padding: 14px;
  gap: 10px;
}

.board-devices {
  display: flex;
  flex: 1;
  gap: 12px;
  min-height: 0;
}

.status-bar {
  height: 26px;
  flex-shrink: 0;
  background: var(--color-device-bezel);
  border: 1px solid #1a1a20;
  border-radius: 4px;
  display: flex;
  align-items: center;
  padding: 0 14px;
  gap: 10px;
  font-family: var(--font-mono);
  font-size: 10px;
  letter-spacing: 0.08em;
}

.status-project {
  color: var(--color-accent-primary);
  font-weight: 500;
}

.status-sep {
  color: var(--color-text-disabled);
}

.status-stats {
  color: var(--color-text-secondary);
}

.status-save {
  margin-left: auto;
  color: var(--color-text-disabled);
}
</style>

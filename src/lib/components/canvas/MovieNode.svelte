<script lang="ts">
  import { Handle, Position } from '@xyflow/svelte';
  import type { NodeProps } from '@xyflow/svelte';
  import type { MovieNode } from '$lib/types/node';
  import { filterStore, isFilterActive, matchesMovie } from '$lib/stores/filter';

  type Props = NodeProps & { data: MovieNode };
  let { data, selected }: Props = $props();

  const dimmed = $derived($isFilterActive && !matchesMovie(data, $filterStore));

  const statusColors: Record<string, string> = {
    watched: '#50c070',
    watching: '#50a8e0',
    want_to_watch: '#e0a850',
    dropped: '#9a9590',
    none: '#5c5955',
  };

  const statusLabels: Record<string, string> = {
    watched: 'WATCHED',
    watching: 'WATCHING',
    want_to_watch: 'QUEUED',
    dropped: 'DROPPED',
    none: '',
  };
</script>

<Handle type="target" position={Position.Top} />

<div class="node-card" class:selected class:dimmed>
  <div class="poster-area">
    {#if data.poster}
      <img class="poster poster-vhs" src={data.poster} alt={data.title} />
    {:else}
      <div class="no-signal">
        <span class="no-signal-text">NO SIGNAL</span>
      </div>
    {/if}
    {#if data.status && data.status !== 'none'}
      <div class="status-badge" style="background-color: {statusColors[data.status]}">
        {statusLabels[data.status]}
      </div>
    {/if}
  </div>
  <div class="node-info">
    <div class="node-title">{data.title}</div>
    <div class="node-meta">
      <span class="node-year">{data.year || '—'}</span>
      {#if data.my_rating !== null && data.my_rating !== undefined}
        <span class="node-rating">★ {data.my_rating}</span>
      {/if}
    </div>
  </div>
</div>

<Handle type="source" position={Position.Bottom} />

<style>
.node-card {
  background: var(--color-device-body);
  border: 2px solid var(--color-device-bezel);
  border-radius: 6px;
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.4), inset 0 1px 0 rgba(255, 255, 255, 0.05);
  overflow: hidden;
  width: 140px;
  cursor: pointer;
  transition: border-color var(--duration-fast) var(--ease-mechanical);
  user-select: none;
  animation: node-record 0.3s var(--ease-soft) both;
}

.node-card.selected {
  border-color: var(--color-accent-primary);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.4), 0 0 0 2px rgba(224, 120, 80, 0.3);
}

.node-card.dimmed {
  opacity: 0.15;
  transition: opacity 0.2s ease;
}

.poster-area {
  position: relative;
  aspect-ratio: 2 / 3;
  background: var(--color-screen-bg);
  overflow: hidden;
}

.poster {
  width: 100%;
  height: 100%;
  object-fit: cover;
  display: block;
}

.no-signal {
  width: 100%;
  height: 100%;
  background: linear-gradient(
    90deg,
    #fff 0%, #fff 14.28%,
    #ff0 14.28%, #ff0 28.57%,
    #0ff 28.57%, #0ff 42.85%,
    #0f0 42.85%, #0f0 57.14%,
    #f0f 57.14%, #f0f 71.42%,
    #f00 71.42%, #f00 85.71%,
    #00f 85.71%, #00f 100%
  );
  filter: saturate(0.5) brightness(0.4);
  display: flex;
  align-items: center;
  justify-content: center;
}

.no-signal-text {
  font-family: var(--font-mono);
  font-size: 9px;
  letter-spacing: 0.05em;
  color: rgba(255, 255, 255, 0.7);
  background: rgba(0, 0, 0, 0.6);
  padding: 2px 6px;
}

.status-badge {
  position: absolute;
  bottom: 4px;
  left: 4px;
  font-family: var(--font-mono);
  font-size: 8px;
  font-weight: 600;
  letter-spacing: 0.08em;
  color: #0f1218;
  padding: 1px 5px;
  border-radius: 2px;
}

.node-info {
  padding: 6px 8px;
  background: rgba(0, 0, 0, 0.3);
}

.node-title {
  color: var(--color-text-primary);
  font-family: var(--font-ui);
  font-size: 11px;
  font-weight: 500;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  line-height: 1.3;
}

.node-meta {
  display: flex;
  justify-content: space-between;
  margin-top: 2px;
}

.node-year {
  color: var(--color-text-secondary);
  font-family: var(--font-mono);
  font-size: 10px;
}

.node-rating {
  color: var(--color-warning);
  font-family: var(--font-mono);
  font-size: 10px;
}
</style>

<script lang="ts">
  import { Handle, Position } from '@xyflow/svelte';
  import type { NodeProps } from '@xyflow/svelte';
  import type { ActorNode } from '$lib/types/node';
  import { filterStore, isFilterActive, matchesActor } from '$lib/stores/filter';

  type Props = NodeProps & { data: ActorNode };
  let { data, selected }: Props = $props();

  const dimmed = $derived($isFilterActive && !matchesActor(data, $filterStore));
</script>

<Handle type="target" position={Position.Top} />

<div class="node-card" class:selected class:dimmed>
  <div class="photo-area">
    {#if data.photo}
      <img class="photo" src={data.photo} alt={data.name} />
    {:else}
      <div class="no-photo">
        <svg class="person-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
          <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2" />
          <circle cx="12" cy="7" r="4" />
        </svg>
      </div>
    {/if}
  </div>
  <div class="node-info">
    <div class="node-name">{data.name}</div>
    <div class="node-type-label">ACTOR</div>
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
  width: 110px;
  cursor: pointer;
  transition: border-color var(--duration-fast) var(--ease-mechanical);
  user-select: none;
}

.node-card.selected {
  border-color: var(--color-accent-secondary);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.4), 0 0 0 2px rgba(80, 168, 224, 0.3);
}

.node-card.dimmed {
  opacity: 0.15;
  transition: opacity 0.2s ease;
}

.photo-area {
  aspect-ratio: 1 / 1;
  background: var(--color-screen-bg);
  overflow: hidden;
}

.photo {
  width: 100%;
  height: 100%;
  object-fit: cover;
  object-position: top;
  display: block;
  filter: sepia(0.06) saturate(0.95) contrast(1.05);
}

.no-photo {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(180deg, #1a2030 0%, #0f1218 100%);
}

.person-icon {
  width: 44px;
  height: 44px;
  color: #2a3a50;
}

.node-info {
  padding: 6px 8px;
  background: rgba(0, 0, 0, 0.3);
}

.node-name {
  color: var(--color-text-primary);
  font-family: var(--font-ui);
  font-size: 11px;
  font-weight: 500;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  line-height: 1.3;
}

.node-type-label {
  color: var(--color-accent-secondary);
  font-family: var(--font-mono);
  font-size: 9px;
  letter-spacing: 0.1em;
  margin-top: 2px;
}
</style>

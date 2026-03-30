<script lang="ts">
  import { untrack } from 'svelte';
  import { get } from 'svelte/store';
  import { SvelteFlow, Background, BackgroundVariant, Controls, type Node, type Edge as FlowEdge } from '@xyflow/svelte';
  import '@xyflow/svelte/dist/style.css';
  import ELK from 'elkjs';
  import { graphStore, movieCount, actorCount } from '$lib/stores/graph';
  import { selectionStore } from '$lib/stores/selection';
  import MovieNodeComponent from '../canvas/MovieNode.svelte';
  import ActorNodeComponent from '../canvas/ActorNode.svelte';

  const nodeTypes = {
    movie: MovieNodeComponent,
    actor: ActorNodeComponent,
  };

  let flowNodes: Node[] = $state([]);
  let flowEdges: FlowEdge[] = $state([]);

  // Sync graphStore → flow (preserve existing positions on update)
  $effect(() => {
    const g = $graphStore;

    untrack(() => {
      const posMap = new Map(flowNodes.map((n) => [n.id, n.position]));

      flowNodes = [
        ...Array.from(g.movies.values()).map((m) => ({
          id: m.id,
          type: 'movie' as const,
          position: posMap.get(m.id) ?? { x: m.position.x, y: m.position.y },
          data: m,
        })),
        ...Array.from(g.actors.values()).map((a) => ({
          id: a.id,
          type: 'actor' as const,
          position: posMap.get(a.id) ?? { x: a.position.x, y: a.position.y },
          data: a,
        })),
      ];

      flowEdges = Array.from(g.edges.values()).map((e) => ({
        id: e.id,
        source: e.from,
        target: e.to,
        label: edgeLabel(e.relationship),
      }));
    });
  });

  function handleNodeClick({ node }: { node: Node; event: MouseEvent | TouchEvent }) {
    if (node.type === 'movie' || node.type === 'actor') {
      selectionStore.set({ id: node.id, type: node.type });
    }
  }

  // Sync dragged positions back to graphStore
  function handleDragStop({ targetNode }: { targetNode: Node | null; nodes: Node[]; event: MouseEvent | TouchEvent }) {
    if (!targetNode) return;
    if (targetNode.type === 'movie') {
      graphStore.updateMoviePosition(targetNode.id, targetNode.position);
    } else if (targetNode.type === 'actor') {
      graphStore.updateActorPosition(targetNode.id, targetNode.position);
    }
  }

  function edgeLabel(relationship: string): string {
    if (relationship === 'acted_in') return 'ACTED';
    if (relationship === 'liked_actor') return 'LIKED';
    if (relationship === 'recommended') return 'REC';
    return '';
  }

  const isEmpty = $derived($movieCount === 0 && $actorCount === 0);

  // ── ELK auto-layout ───────────────────────────────────────────
  const elk = new ELK();

  async function runElkLayout() {
    const g = get(graphStore);
    const children = [
      ...Array.from(g.movies.values()).map((m) => ({ id: m.id, width: 160, height: 90 })),
      ...Array.from(g.actors.values()).map((a) => ({ id: a.id, width: 80, height: 80 })),
    ];
    if (children.length === 0) return;

    const edges = Array.from(g.edges.values()).map((e) => ({
      id: e.id,
      sources: [e.from],
      targets: [e.to],
    }));

    try {
      const layout = await elk.layout({
        id: 'root',
        layoutOptions: {
          'elk.algorithm': 'layered',
          'elk.direction': 'RIGHT',
          'elk.layered.spacing.nodeNodeBetweenLayers': '100',
          'elk.spacing.nodeNode': '60',
        },
        children,
        edges,
      });
      layout.children?.forEach((n) => {
        const pos = { x: n.x ?? 0, y: n.y ?? 0 };
        if (n.id.startsWith('m:')) graphStore.updateMoviePosition(n.id, pos);
        else graphStore.updateActorPosition(n.id, pos);
      });
    } catch (err) {
      console.error('ELK layout failed:', err);
    }
  }

  // Trigger layout when node count increases (new nodes added)
  let layoutNodeCount = 0;
  $effect(() => {
    const total = $movieCount + $actorCount;
    if (total > layoutNodeCount) {
      layoutNodeCount = total;
      runElkLayout();
    }
  });
</script>

<div class="device" role="region" aria-label="Graph Monitor">
  <div class="device-header">
    <span class="device-title">GRAPH MONITOR</span>
    <div class="device-indicators">
      <div class="led" aria-hidden="true"></div>
      <div class="led led-green" aria-hidden="true"></div>
      <div class="led led-amber" aria-hidden="true"></div>
    </div>
  </div>

  <div class="device-screen">
    <SvelteFlow
      bind:nodes={flowNodes}
      bind:edges={flowEdges}
      {nodeTypes}
      fitView
      minZoom={0.1}
      maxZoom={4}
      onnodeclick={handleNodeClick}
      onnodedragstop={handleDragStop}
      onpaneclick={() => selectionStore.set(null)}
      colorMode="dark"
    >
      <Background gap={32} patternColor="rgba(255,255,255,0.06)" variant={BackgroundVariant.Dots} size={1} />
      <Controls />

      {#if isEmpty}
        <div class="empty-state">
          <div class="empty-icon">▣</div>
          <div class="empty-title">NO SIGNAL</div>
          <div class="empty-hint">Open a project or add a movie to begin</div>
        </div>
      {/if}
    </SvelteFlow>
  </div>
</div>

<style>
.device {
  flex: 1;
  display: flex;
  flex-direction: column;
  background: var(--color-device-body);
  border: 2px solid var(--color-device-bezel);
  border-radius: 8px;
  box-shadow: var(--shadow-device), inset 0 1px 0 rgba(255, 255, 255, 0.04);
  overflow: hidden;
  min-width: 0;
}

.device-header {
  display: flex;
  align-items: center;
  height: 32px;
  padding: 0 12px;
  background: linear-gradient(180deg, #3a3a40 0%, #2d2d32 100%);
  border-bottom: 1px solid #1a1a20;
  flex-shrink: 0;
}

.device-title {
  color: var(--color-text-secondary);
  font-family: var(--font-ui);
  font-size: 11px;
  font-weight: 500;
  letter-spacing: 0.15em;
  text-transform: uppercase;
}

.device-indicators {
  display: flex;
  gap: 6px;
  margin-left: auto;
}

.led {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: var(--color-led-off);
  box-shadow: inset 0 1px 2px rgba(0, 0, 0, 0.5);
}

.led-green {
  background: var(--color-led-green);
  box-shadow: 0 0 6px var(--color-led-green), 0 0 12px rgba(48, 255, 80, 0.3),
    inset 0 1px 2px rgba(0, 0, 0, 0.2);
}

.led-amber {
  background: var(--color-led-amber);
  box-shadow: 0 0 6px var(--color-led-amber), 0 0 12px rgba(255, 170, 48, 0.3),
    inset 0 1px 2px rgba(0, 0, 0, 0.2);
}

.device-screen {
  flex: 1;
  margin: 10px;
  border: 3px solid #1a1a20;
  border-radius: 4px;
  overflow: hidden;
  position: relative;
  background: var(--color-screen-bg);
  box-shadow: inset 0 0 30px rgba(0, 0, 0, 0.5), inset 0 0 2px rgba(255, 255, 255, 0.03);
}

/* CRT vignette overlay */
.device-screen::before {
  content: '';
  position: absolute;
  inset: 0;
  background: radial-gradient(
    ellipse at center,
    transparent 0%,
    transparent 55%,
    rgba(0, 0, 0, 0.2) 100%
  );
  pointer-events: none;
  z-index: 10;
}

/* Subtle scan lines */
.device-screen::after {
  content: '';
  position: absolute;
  inset: 0;
  background: repeating-linear-gradient(
    0deg,
    transparent 0px,
    transparent 2px,
    rgba(0, 0, 0, 0.025) 2px,
    rgba(0, 0, 0, 0.025) 4px
  );
  pointer-events: none;
  z-index: 9;
}

.empty-state {
  position: absolute;
  inset: 0;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 8px;
  z-index: 5;
  pointer-events: none;
}

.empty-icon {
  font-size: 48px;
  color: #1e2830;
  line-height: 1;
}

.empty-title {
  font-family: var(--font-mono);
  font-size: 18px;
  letter-spacing: 0.2em;
  color: #1e2a3a;
}

.empty-hint {
  font-family: var(--font-mono);
  font-size: 11px;
  letter-spacing: 0.08em;
  color: #1e2a3a;
  margin-top: 4px;
}

/* Override SvelteFlow default styles for dark theme */
:global(.svelte-flow) {
  background: transparent !important;
}

:global(.svelte-flow__controls) {
  background: var(--color-device-bezel);
  border: 1px solid #3a3a40;
  border-radius: 4px;
  overflow: hidden;
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.3);
}

:global(.svelte-flow__controls-button) {
  background: var(--color-device-bezel);
  border-bottom: 1px solid #3a3a40;
  color: var(--color-text-secondary);
  fill: var(--color-text-secondary);
  width: 26px;
  height: 26px;
  padding: 4px;
}

:global(.svelte-flow__controls-button:hover) {
  background: #3a3a42;
  color: var(--color-text-primary);
  fill: var(--color-text-primary);
}

:global(.svelte-flow__edge-path) {
  stroke: #50a8e060;
  stroke-width: 2;
}

:global(.svelte-flow__edge.selected .svelte-flow__edge-path) {
  stroke: var(--color-accent-secondary);
}

:global(.svelte-flow__edge-textwrapper text) {
  fill: #3a5a70;
  font-family: 'Courier New', monospace;
  font-size: 9px;
  letter-spacing: 0.08em;
}

:global(.svelte-flow__edge-textbg) {
  fill: #0a0e14;
  opacity: 0.85;
}

:global(.svelte-flow__handle) {
  width: 8px;
  height: 8px;
  background: var(--color-device-bezel);
  border: 2px solid #3a3a50;
  border-radius: 50%;
}

:global(.svelte-flow__handle:hover) {
  background: var(--color-accent-secondary);
  border-color: var(--color-accent-secondary);
}

@media (prefers-reduced-motion: reduce) {
  .device-screen::after {
    display: none;
  }
}
</style>

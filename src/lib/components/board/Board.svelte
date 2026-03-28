<script lang="ts">
  import GraphMonitor from '../devices/GraphMonitor.svelte';
  import ControlTerminal from '../devices/ControlTerminal.svelte';
  import { movieCount, actorCount, graphStore } from '$lib/stores/graph';
  import { projectStore } from '$lib/stores/project';
  import { createProject, openProject, saveProject, pickFolder } from '$lib/services/tauri';
  import type { Project } from '$lib/types/project';

  let saveStatus = $state<'idle' | 'saving' | 'saved' | 'error'>('idle');
  let newProjectModal = $state(false);
  let newProjectName = $state('');

  async function handleSave() {
    const ps = $projectStore;
    if (!ps.isLoaded || !ps.path || !ps.manifest) return;
    saveStatus = 'saving';
    try {
      const g = $graphStore;
      const project: Project = {
        path: ps.path,
        manifest: ps.manifest,
        movies: Array.from(g.movies.values()),
        actors: Array.from(g.actors.values()),
        edges: Array.from(g.edges.values()),
      };
      await saveProject(project);
      projectStore.markSaved();
      saveStatus = 'saved';
      setTimeout(() => (saveStatus = 'idle'), 2000);
    } catch {
      saveStatus = 'error';
      setTimeout(() => (saveStatus = 'idle'), 3000);
    }
  }

  async function handleOpen() {
    const folder = await pickFolder();
    if (!folder) return;
    try {
      const project = await openProject(folder);
      projectStore.load(project);
      graphStore.load(project.movies, project.actors, project.edges);
    } catch (e) {
      console.error('Failed to open project:', e);
    }
  }

  async function handleNewProject() {
    const folder = await pickFolder();
    if (!folder) return;
    newProjectModal = true;
    newProjectName = folder.split(/[/\\]/).pop() ?? 'My Graph';
    // Store folder for when modal is confirmed
    pendingNewFolder = folder;
  }

  let pendingNewFolder = $state('');

  async function confirmNewProject() {
    if (!pendingNewFolder || !newProjectName.trim()) return;
    try {
      const project = await createProject(pendingNewFolder, newProjectName.trim());
      projectStore.load(project);
      graphStore.load([], [], []);
    } catch (e) {
      console.error('Failed to create project:', e);
    } finally {
      newProjectModal = false;
      newProjectName = '';
      pendingNewFolder = '';
    }
  }

  // Ctrl+S to save
  function onKeyDown(e: KeyboardEvent) {
    if ((e.ctrlKey || e.metaKey) && e.key === 's') {
      e.preventDefault();
      handleSave();
    }
  }

  const saveLabel = $derived(
    saveStatus === 'saving' ? 'SAVING...' :
    saveStatus === 'saved' ? 'SAVED' :
    saveStatus === 'error' ? 'SAVE FAILED' :
    $projectStore.isDirty ? '[SAVE]' : 'SAVED'
  );
</script>

<svelte:window onkeydown={onKeyDown} />

<div class="board">
  <div class="board-devices">
    <GraphMonitor />
    <ControlTerminal />
  </div>
  <div class="status-bar">
    <span class="status-project">
      {$projectStore.isLoaded ? $projectStore.manifest?.name ?? 'MOVIE GRAPH' : 'MOVIE GRAPH'}
    </span>
    <span class="status-sep">·</span>
    <span class="status-stats">{$movieCount} MOVIES · {$actorCount} ACTORS</span>
    <div class="status-actions">
      <button class="status-btn" onclick={handleOpen}>OPEN</button>
      <button class="status-btn" onclick={handleNewProject}>NEW</button>
      {#if $projectStore.isLoaded}
        <button
          class="status-btn"
          class:save-dirty={$projectStore.isDirty}
          class:save-ok={saveStatus === 'saved'}
          class:save-err={saveStatus === 'error'}
          onclick={handleSave}
          disabled={saveStatus === 'saving'}
        >{saveLabel}</button>
      {/if}
    </div>
  </div>
</div>

{#if newProjectModal}
  <div class="modal-overlay" role="dialog" aria-modal="true" aria-label="New Project">
    <div class="modal">
      <div class="modal-title">NEW PROJECT</div>
      <div class="modal-field">
        <label class="modal-label" for="project-name">PROJECT NAME</label>
        <input
          id="project-name"
          class="modal-input"
          type="text"
          bind:value={newProjectName}
          onkeydown={(e) => e.key === 'Enter' && confirmNewProject()}
          placeholder="My Movie Graph"
        />
      </div>
      <div class="modal-hint">{pendingNewFolder}</div>
      <div class="modal-actions">
        <button class="modal-btn-cancel" onclick={() => (newProjectModal = false)}>CANCEL</button>
        <button class="modal-btn-ok" onclick={confirmNewProject}>CREATE</button>
      </div>
    </div>
  </div>
{/if}

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

.status-actions {
  margin-left: auto;
  display: flex;
  gap: 8px;
}

.status-btn {
  background: transparent;
  border: 1px solid #2a2a38;
  border-radius: 2px;
  color: var(--color-text-disabled);
  cursor: pointer;
  font-family: var(--font-mono);
  font-size: 9px;
  letter-spacing: 0.1em;
  padding: 2px 8px;
  transition: color var(--duration-fast), border-color var(--duration-fast);
}

.status-btn:hover:not(:disabled) {
  color: var(--color-text-secondary);
  border-color: #3a3a50;
}

.status-btn.save-dirty {
  color: var(--color-accent-primary);
  border-color: rgba(224, 120, 80, 0.4);
}

.status-btn.save-ok {
  color: var(--color-led-green);
  border-color: rgba(48, 255, 80, 0.3);
}

.status-btn.save-err {
  color: #ff6060;
  border-color: rgba(255, 80, 80, 0.3);
}

/* Modal */
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
  padding: 20px;
  width: 320px;
  display: flex;
  flex-direction: column;
  gap: 14px;
}

.modal-title {
  color: var(--color-text-secondary);
  font-family: var(--font-ui);
  font-size: 12px;
  font-weight: 600;
  letter-spacing: 0.2em;
}

.modal-field {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.modal-label {
  color: var(--color-text-disabled);
  font-family: var(--font-ui);
  font-size: 9px;
  letter-spacing: 0.15em;
}

.modal-input {
  background: rgba(255, 255, 255, 0.04);
  border: 1px solid #2a2a38;
  border-radius: 2px;
  color: var(--color-text-screen);
  font-family: var(--font-mono);
  font-size: 13px;
  outline: none;
  padding: 8px 10px;
  caret-color: var(--color-accent-primary);
  transition: border-color var(--duration-fast);
}

.modal-input:focus {
  border-color: var(--color-accent-primary);
}

.modal-hint {
  color: var(--color-text-disabled);
  font-family: var(--font-mono);
  font-size: 10px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.modal-actions {
  display: flex;
  gap: 8px;
  justify-content: flex-end;
}

.modal-btn-cancel {
  background: transparent;
  border: 1px solid #2a2a38;
  border-radius: 2px;
  color: var(--color-text-disabled);
  cursor: pointer;
  font-family: var(--font-ui);
  font-size: 10px;
  letter-spacing: 0.1em;
  padding: 6px 14px;
  transition: color var(--duration-fast);
}

.modal-btn-cancel:hover { color: var(--color-text-secondary); }

.modal-btn-ok {
  background: var(--color-accent-primary);
  border: none;
  border-radius: 2px;
  color: #fff;
  cursor: pointer;
  font-family: var(--font-ui);
  font-size: 10px;
  font-weight: 600;
  letter-spacing: 0.1em;
  padding: 6px 14px;
  transition: opacity var(--duration-fast);
}

.modal-btn-ok:hover { opacity: 0.85; }
</style>

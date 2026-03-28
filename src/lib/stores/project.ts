import { writable, derived } from 'svelte/store';
import type { Project, Manifest } from '../types/project';

type ProjectState = {
  path: string | null;
  manifest: Manifest | null;
  isLoaded: boolean;
  isDirty: boolean;
  lastSaved: Date | null;
};

const initialState: ProjectState = {
  path: null,
  manifest: null,
  isLoaded: false,
  isDirty: false,
  lastSaved: null,
};

function createProjectStore() {
  const { subscribe, set, update } = writable<ProjectState>(initialState);

  return {
    subscribe,
    load(project: Project) {
      set({
        path: project.path,
        manifest: project.manifest,
        isLoaded: true,
        isDirty: false,
        lastSaved: new Date(project.manifest.modified_at),
      });
    },
    markDirty() {
      update((s) => ({ ...s, isDirty: true }));
    },
    markSaved() {
      update((s) => ({ ...s, isDirty: false, lastSaved: new Date() }));
    },
    close() {
      set(initialState);
    },
  };
}

export const projectStore = createProjectStore();

export const isProjectLoaded = derived(projectStore, ($p) => $p.isLoaded);

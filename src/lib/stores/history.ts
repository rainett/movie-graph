import { writable, derived } from 'svelte/store';
import type { Command } from '$lib/commands';

const MAX_HISTORY = 50;

function createHistoryStore() {
  const { subscribe, update, set } = writable<{
    undoStack: Command[];
    redoStack: Command[];
  }>({ undoStack: [], redoStack: [] });

  return {
    subscribe,
    execute(cmd: Command) {
      cmd.execute();
      update((s) => {
        const undoStack = [...s.undoStack, cmd];
        if (undoStack.length > MAX_HISTORY) undoStack.shift();
        return { undoStack, redoStack: [] };
      });
    },
    undo() {
      update((s) => {
        if (s.undoStack.length === 0) return s;
        const undoStack = [...s.undoStack];
        const cmd = undoStack.pop()!;
        cmd.undo();
        return { undoStack, redoStack: [...s.redoStack, cmd] };
      });
    },
    redo() {
      update((s) => {
        if (s.redoStack.length === 0) return s;
        const redoStack = [...s.redoStack];
        const cmd = redoStack.pop()!;
        cmd.execute();
        return { redoStack, undoStack: [...s.undoStack, cmd] };
      });
    },
    clear() {
      set({ undoStack: [], redoStack: [] });
    },
  };
}

export const historyStore = createHistoryStore();
export const canUndo = derived(historyStore, ($h) => $h.undoStack.length > 0);
export const canRedo = derived(historyStore, ($h) => $h.redoStack.length > 0);

import { writable } from 'svelte/store';

export type SelectedNode = {
  id: string;
  type: 'movie' | 'actor';
} | null;

export const selectionStore = writable<SelectedNode>(null);

import { writable } from 'svelte/store';

export type TerminalMode = 'search' | 'inspect' | 'filter';

export const terminalModeStore = writable<TerminalMode>('search');

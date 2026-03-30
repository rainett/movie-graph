import { writable, derived } from 'svelte/store';
import type { MovieNode, ActorNode, Status } from '$lib/types/node';

export type NodeTypeFilter = 'all' | 'movies' | 'actors';

export type FilterState = {
  statuses: Set<Status>;
  minRating: number;    // 0 = no minimum
  maxRating: number;    // 0 = no maximum
  minYear: number;      // 0 = no minimum
  maxYear: number;      // 0 = no maximum
  nodeType: NodeTypeFilter;
  text: string;
};

function freshState(): FilterState {
  return {
    statuses: new Set(),
    minRating: 0,
    maxRating: 0,
    minYear: 0,
    maxYear: 0,
    nodeType: 'all',
    text: '',
  };
}

function createFilterStore() {
  const { subscribe, update, set } = writable<FilterState>(freshState());

  return {
    subscribe,
    toggleStatus(status: Status) {
      update((s) => {
        const statuses = new Set(s.statuses);
        if (statuses.has(status)) statuses.delete(status);
        else statuses.add(status);
        return { ...s, statuses };
      });
    },
    setRatingRange(min: number, max: number) {
      update((s) => ({ ...s, minRating: min, maxRating: max }));
    },
    setYearRange(min: number, max: number) {
      update((s) => ({ ...s, minYear: min, maxYear: max }));
    },
    setNodeType(nodeType: NodeTypeFilter) {
      update((s) => ({ ...s, nodeType }));
    },
    setText(text: string) {
      update((s) => ({ ...s, text }));
    },
    clear() {
      set(freshState());
    },
  };
}

export const filterStore = createFilterStore();

export const isFilterActive = derived(filterStore, ($f) =>
  $f.statuses.size > 0 ||
  $f.minRating > 0 ||
  $f.maxRating > 0 ||
  $f.minYear > 0 ||
  $f.maxYear > 0 ||
  $f.nodeType !== 'all' ||
  $f.text.trim() !== ''
);

export function matchesMovie(movie: MovieNode, f: FilterState): boolean {
  if (f.nodeType === 'actors') return false;
  if (f.statuses.size > 0 && !f.statuses.has(movie.status)) return false;
  if (f.minRating > 0 && (movie.my_rating === null || movie.my_rating < f.minRating)) return false;
  if (f.maxRating > 0 && (movie.my_rating === null || movie.my_rating > f.maxRating)) return false;
  if (f.minYear > 0 && movie.year < f.minYear) return false;
  if (f.maxYear > 0 && movie.year > f.maxYear) return false;
  if (f.text.trim() && !movie.title.toLowerCase().includes(f.text.trim().toLowerCase())) return false;
  return true;
}

export function matchesActor(actor: ActorNode, f: FilterState): boolean {
  if (f.nodeType === 'movies') return false;
  if (f.text.trim() && !actor.name.toLowerCase().includes(f.text.trim().toLowerCase())) return false;
  return true;
}

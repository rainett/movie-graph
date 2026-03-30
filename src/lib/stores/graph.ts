import { writable, derived } from 'svelte/store';
import type { MovieNode, ActorNode } from '../types/node';
import type { Edge } from '../types/edge';

type GraphState = {
  movies: Map<string, MovieNode>;
  actors: Map<string, ActorNode>;
  edges: Map<string, Edge>;
};

const initialState: GraphState = {
  movies: new Map(),
  actors: new Map(),
  edges: new Map(),
};

function createGraphStore() {
  const { subscribe, set, update } = writable<GraphState>(initialState);

  return {
    subscribe,
    load(movies: MovieNode[], actors: ActorNode[], edges: Edge[]) {
      set({
        movies: new Map(movies.map((m) => [m.id, m])),
        actors: new Map(actors.map((a) => [a.id, a])),
        edges: new Map(edges.map((e) => [e.id, e])),
      });
    },
    addMovie(movie: MovieNode) {
      update((s) => {
        const movies = new Map(s.movies);
        movies.set(movie.id, movie);
        return { ...s, movies };
      });
    },
    removeMovie(id: string) {
      update((s) => {
        const movies = new Map(s.movies);
        movies.delete(id);
        return { ...s, movies };
      });
    },
    addActor(actor: ActorNode) {
      update((s) => {
        const actors = new Map(s.actors);
        actors.set(actor.id, actor);
        return { ...s, actors };
      });
    },
    removeActor(id: string) {
      update((s) => {
        const actors = new Map(s.actors);
        actors.delete(id);
        return { ...s, actors };
      });
    },
    addEdge(edge: Edge) {
      update((s) => {
        const edges = new Map(s.edges);
        edges.set(edge.id, edge);
        return { ...s, edges };
      });
    },
    removeEdge(id: string) {
      update((s) => {
        const edges = new Map(s.edges);
        edges.delete(id);
        return { ...s, edges };
      });
    },
    updateMovie(id: string, updates: Partial<MovieNode>) {
      update((s) => {
        const movies = new Map(s.movies);
        const movie = movies.get(id);
        if (movie) movies.set(id, { ...movie, ...updates });
        return { ...s, movies };
      });
    },
    updateActor(id: string, updates: Partial<ActorNode>) {
      update((s) => {
        const actors = new Map(s.actors);
        const actor = actors.get(id);
        if (actor) actors.set(id, { ...actor, ...updates });
        return { ...s, actors };
      });
    },
    updateMoviePosition(id: string, position: { x: number; y: number }) {
      update((s) => {
        const movies = new Map(s.movies);
        const movie = movies.get(id);
        if (movie) movies.set(id, { ...movie, position });
        return { ...s, movies };
      });
    },
    updateActorPosition(id: string, position: { x: number; y: number }) {
      update((s) => {
        const actors = new Map(s.actors);
        const actor = actors.get(id);
        if (actor) actors.set(id, { ...actor, position });
        return { ...s, actors };
      });
    },
    reset() {
      set(initialState);
    },
  };
}

export const graphStore = createGraphStore();

export const movieCount = derived(graphStore, ($g) => $g.movies.size);
export const actorCount = derived(graphStore, ($g) => $g.actors.size);
export const edgeCount = derived(graphStore, ($g) => $g.edges.size);

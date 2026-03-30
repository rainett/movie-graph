import { get } from 'svelte/store';
import { graphStore } from '$lib/stores/graph';
import { projectStore } from '$lib/stores/project';
import { selectionStore } from '$lib/stores/selection';
import type { MovieNode, ActorNode } from '$lib/types/node';
import type { Edge } from '$lib/types/edge';
import type { Command } from './index';

export class AddMovieCommand implements Command {
  description: string;

  constructor(
    private movie: MovieNode,
    private edges: Edge[] = []
  ) {
    this.description = `Added movie: ${movie.title}`;
  }

  execute() {
    graphStore.addMovie(this.movie);
    for (const edge of this.edges) graphStore.addEdge(edge);
    projectStore.markDirty();
  }

  undo() {
    for (const edge of this.edges) graphStore.removeEdge(edge.id);
    graphStore.removeMovie(this.movie.id);
    projectStore.markDirty();
  }
}

export class AddActorCommand implements Command {
  description: string;

  constructor(
    private actor: ActorNode,
    private edges: Edge[] = []
  ) {
    this.description = `Added actor: ${actor.name}`;
  }

  execute() {
    graphStore.addActor(this.actor);
    for (const edge of this.edges) graphStore.addEdge(edge);
    projectStore.markDirty();
  }

  undo() {
    for (const edge of this.edges) graphStore.removeEdge(edge.id);
    graphStore.removeActor(this.actor.id);
    projectStore.markDirty();
  }
}

export class DeleteNodeCommand implements Command {
  private node: MovieNode | ActorNode;
  private nodeType: 'movie' | 'actor';
  private edges: Edge[];
  description: string;

  constructor(id: string, type: 'movie' | 'actor') {
    const g = get(graphStore);
    this.nodeType = type;
    const node = type === 'movie' ? g.movies.get(id) : g.actors.get(id);
    if (!node) throw new Error(`Node ${id} not found`);
    this.node = node;
    this.edges = Array.from(g.edges.values()).filter(
      (e) => e.from === id || e.to === id
    );
    const name = type === 'movie' ? (node as MovieNode).title : (node as ActorNode).name;
    this.description = `Removed ${type}: ${name}`;
  }

  execute() {
    for (const edge of this.edges) graphStore.removeEdge(edge.id);
    if (this.nodeType === 'movie') graphStore.removeMovie(this.node.id);
    else graphStore.removeActor(this.node.id);
    selectionStore.set(null);
    projectStore.markDirty();
  }

  undo() {
    if (this.nodeType === 'movie') graphStore.addMovie(this.node as MovieNode);
    else graphStore.addActor(this.node as ActorNode);
    for (const edge of this.edges) graphStore.addEdge(edge);
    projectStore.markDirty();
  }
}

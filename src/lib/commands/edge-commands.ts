import { graphStore } from '$lib/stores/graph';
import { projectStore } from '$lib/stores/project';
import type { Edge } from '$lib/types/edge';
import type { Command } from './index';

export class AddEdgeCommand implements Command {
  description = 'Added connection';

  constructor(private edge: Edge) {}

  execute() {
    graphStore.addEdge(this.edge);
    projectStore.markDirty();
  }

  undo() {
    graphStore.removeEdge(this.edge.id);
    projectStore.markDirty();
  }
}

export class DeleteEdgeCommand implements Command {
  description = 'Removed connection';

  constructor(private edge: Edge) {}

  execute() {
    graphStore.removeEdge(this.edge.id);
    projectStore.markDirty();
  }

  undo() {
    graphStore.addEdge(this.edge);
    projectStore.markDirty();
  }
}

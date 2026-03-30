import { graphStore } from '$lib/stores/graph';
import { projectStore } from '$lib/stores/project';
import type { MovieNode, ActorNode } from '$lib/types/node';
import type { Command } from './index';

export class EditMovieCommand implements Command {
  description: string;

  constructor(
    private id: string,
    private oldFields: Partial<MovieNode>,
    private newFields: Partial<MovieNode>
  ) {
    this.description = 'Edited movie';
  }

  execute() {
    graphStore.updateMovie(this.id, this.newFields);
    projectStore.markDirty();
  }

  undo() {
    graphStore.updateMovie(this.id, this.oldFields);
    projectStore.markDirty();
  }
}

export class EditActorCommand implements Command {
  description: string;

  constructor(
    private id: string,
    private oldFields: Partial<ActorNode>,
    private newFields: Partial<ActorNode>
  ) {
    this.description = 'Edited actor';
  }

  execute() {
    graphStore.updateActor(this.id, this.newFields);
    projectStore.markDirty();
  }

  undo() {
    graphStore.updateActor(this.id, this.oldFields);
    projectStore.markDirty();
  }
}

import { graphStore } from '$lib/stores/graph';
import { projectStore } from '$lib/stores/project';
import type { Command } from './index';

export class MoveNodeCommand implements Command {
  description: string;

  constructor(
    private id: string,
    private nodeType: 'movie' | 'actor',
    private oldPos: { x: number; y: number },
    private newPos: { x: number; y: number }
  ) {
    this.description = `Moved ${nodeType}`;
  }

  execute() {
    if (this.nodeType === 'movie') graphStore.updateMoviePosition(this.id, this.newPos);
    else graphStore.updateActorPosition(this.id, this.newPos);
    projectStore.markDirty();
  }

  undo() {
    if (this.nodeType === 'movie') graphStore.updateMoviePosition(this.id, this.oldPos);
    else graphStore.updateActorPosition(this.id, this.oldPos);
    projectStore.markDirty();
  }
}

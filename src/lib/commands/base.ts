export interface Command {
  readonly id: string;
  readonly description: string;
  execute(): void;
  undo(): void;
}

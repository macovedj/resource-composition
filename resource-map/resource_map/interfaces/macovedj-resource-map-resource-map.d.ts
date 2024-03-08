export namespace MacovedjResourceMapResourceMap {
  export { ResourceMap };
}

export class ResourceMap {
  constructor()
  insert(name: string, job: string): void;
  get(name: string): string | undefined;
  keys(): string[];
}

interface MessageTypes {
  print: string;
  table: string[][];
}

declare let Outside: {
  stdout(data: unknown): void;
};

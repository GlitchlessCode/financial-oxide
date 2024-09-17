export interface FileStat {
  readonly id: string;
  title: string;
  stats: { title: string; value: string }[];
  favourite: boolean;
  end: false;
}

export interface FileStatEnd {
  end: true;
}

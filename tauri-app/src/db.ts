import Dexie, { Table } from 'dexie';

export interface Channel {
  id?: number;
  title: string;
  link: string;
  feedUrl: string;
  description?: string;
  pubDate?: Date;
}

export class MySubClassedDexie extends Dexie {
  channels!: Table<Channel>;

  constructor() {
    super('salix');
    this.version(1).stores({
      channels: '++id, title, link, feedUrl, description, pubDate'
    });
  }
}

export const db = new MySubClassedDexie();
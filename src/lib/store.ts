import { writable } from 'svelte/store';

export interface BottleMeta {
  id: string;
  from: string;
  to?: string;
  encrypted: boolean;
  timestamp: number;
  direction?: 'sent' | 'received';
}

export interface BottleContent {
  id: string;
  from: string;
  body: string;
  timestamp: number;
}

export const session = writable<string | null>(null);
export const bottles = writable<BottleMeta[]>([]);
export const welcomeMessage = writable<string | null>(null);

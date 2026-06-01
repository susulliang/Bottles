import { invoke } from '@tauri-apps/api/core';

interface BottleMeta {
  id: string;
  from: string;
  encrypted: boolean;
  timestamp: number;
}

interface BottleContent {
  id: string;
  from: string;
  body: string;
  timestamp: number;
}

export const api = {
  register: (username: string, passphrase: string) =>
    invoke('register', { username, passphrase }),

  login: (username: string, passphrase: string) =>
    invoke('login', { username, passphrase }),

  loginOrRegister: (username: string, passphrase: string) =>
    invoke<boolean>('login_or_register', { username, passphrase }),

  logout: () => invoke('logout'),

  throwBottle: (to: string, body: string) =>
    invoke('throw_bottle', { to, body }),

  fetchBottles: () =>
    invoke<BottleMeta[]>('fetch_bottles'),

  openBottle: (id: string) =>
    invoke<BottleContent>('open_bottle', { id }),

  deleteBottle: (id: string) =>
    invoke('delete_bottle', { id }),

  fetchSentBottles: () =>
    invoke<BottleMeta[]>('fetch_sent_bottles'),

  minimizeApp: () => invoke('minimize_app'),

  exitApp: () => invoke('exit_app'),
};

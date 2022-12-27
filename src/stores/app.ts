import { writable } from 'svelte/store';

export const routes = {
  config: 'config'
}
export const initialConfig = {
  preferences: {
    darkMode: false,
    cache: "1GB",
    exclusions: [

    ],
    buckets: []
  },
}

export type ConfigType = {
  [key: string]: {
    [key: string]: any
  }
}


export const initialRoute = 'Dashboard';
export const currentView = writable(initialRoute);
export const currentConfig = writable(initialConfig);
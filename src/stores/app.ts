import { writable } from 'svelte/store';

export const routes = {
  config: 'config'
}
export const initialRoute = 'Dashboard';
export const currentView = writable(initialRoute);
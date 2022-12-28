import { writable, readable } from 'svelte/store';
import { config } from '../services/config';

const _darkMode = config.get<boolean>('preferences.darkMode');
export const darkMode = writable(_darkMode);
export const setDarkMode = (nextValue: boolean) => {
  darkMode.set(nextValue);
  config.set('preferences.darkMode', nextValue);
}

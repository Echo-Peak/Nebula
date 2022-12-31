import { fetchAPI } from '../services/fetchAPI';

export const setDarkMode = async (nextValue: boolean) => {
  await fetchAPI.configSet('preferences.darkMode', nextValue);
}

export const getDarkMode = async () => {
  const result = await fetchAPI.configGet('preferences.darkMode');
  if (result !== undefined) {
    return result;
  }
}



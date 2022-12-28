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
export const isModalOpen = writable(false);
export const modalView = writable('');

export const showModal = () => isModalOpen.set(true);
export const hideModal = () => isModalOpen.set(false);
export const setModalView = (nextView: string) => modalView.set(nextView);

export const ModalViews = {
  AWS: 'aws',
  USER: 'user'
}
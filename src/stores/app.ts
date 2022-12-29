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
type AvailContextViews = 'explorer';
export type ContextMenuContext = {
  pos: {
    x: number;
    y: number;
  },
  contextView: AvailContextViews;
  content: any;
};

const initialContextMenuContext = {
  pos: {
    x: 0,
    y: 0,
  },
  contextView: '',
  content: {}
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

export const isContextMenuOpen = writable(false);
export const contextMenuContext = writable(initialContextMenuContext);
export const hideContextMenu = () => isContextMenuOpen.set(false);
export const showContextMenu = () => isContextMenuOpen.set(true);
export const setContextMenuContext = (x: number, y: number, contextView: AvailContextViews, content: any) => contextMenuContext.set({
  pos: {
    x,
    y,
  },
  contextView,
  content
});

export const ModalViews = {
  AWS: 'aws',
  USER: 'user',
  ABOUT: 'about'
}
export const contextMenuContextViews = {
  EXPLORER: 'EXPLORER'
}
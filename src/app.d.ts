import { DevData } from './types/data';
// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
// and what to do when importing types
declare namespace App {
  interface PageData {
    title: string;
  }
}

declare global {
  namespace NodeJS {
    interface ProcessEnv {
      NODE_ENV: 'development' | 'production';
    }
  }
  declare interface Window {
    __TAURI__?: {
      invoke: (functionName: string, args: Record<string, any>) => Promise<unknown>
    },
    devData: DevData,
    isDevelopment: boolean;
  }
}


declare 
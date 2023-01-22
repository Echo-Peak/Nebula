import { invoke } from '@tauri-apps/api/tauri';
import { getVersion } from '@tauri-apps/api/app';
import type { BackendAPI as IBackendAPI } from '../types/interfaces';

type TauriResult = {
  type: string,
  value: string
};

const isCompiled = () => {
  if (window.__TAURI__ && window.__TAURI__.hasOwnProperty('invoke')) return true;
  return false;
}


class BackendAPI implements IBackendAPI {
  async invoke<T>(commandName: string, args?: Record<string, any>): Promise<T | undefined> {
    const response = await invoke(commandName, args) as TauriResult;
    if (response) return response as T;
  }
  hasTauriApi() {
    return isCompiled();
  }
  async getAppVersion() {
    const appVersion = await getVersion();
    return appVersion;
  }
  async hideWindow() {
    await this.invoke<boolean>('app_hide');
  }
  async showWindow() {
    await this.invoke<boolean>('app_show');
  }
  async exitApp() {
    await this.invoke<void>('app_exit');
  }
  async openURL(url: string) {
    await this.invoke<void>('open_url', { url });
  }
  async saveLog(logMessage: string, otherArgs: any[]) {
    await this.invoke<void>('save_log', { logMessage, args: JSON.stringify(otherArgs) });
  }
}
export const Backend = new BackendAPI();
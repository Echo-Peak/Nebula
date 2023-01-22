import type { AWSCredentials, BucketItem, Config, ConfigSections } from "./data";

export interface BackendAPI {
  invoke<T>(commandName: string, args?: Record<string, any>): Promise<T | undefined>;
  hasTauriApi(): void;
  getAppVersion(): void;
  hideWindow(): void;
  showWindow(): void;
  exitApp(): void;
  openURL(url: string): void;
  saveLog(logMessage: string, otherArgs: any[]): void;
}

export interface S3Api {
  getBuckets(): Promise<BucketItem[]>;
  listContents(S3Path: string): Promise<string[]>;
  downloadFile(S3Path: string): Promise<boolean>
  setCreds(accessKey: string, secret: string, region: string): Promise<boolean>
}

export interface ConfigAPI {
  getConfig(): Promise<Config | undefined>
  getItem<T>(sectionID: ConfigSections, prop: keyof Config[ConfigSections]): Promise<T | undefined>
  setItem(sectionID: ConfigSections, key: string, value: any): Promise<void>,
  getCredentials(): Promise<AWSCredentials | undefined>
}
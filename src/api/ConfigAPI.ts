import type { AWSCredentials } from './../types/data';
import type { Config, ConfigSections } from 'src/types/data';
import type { ConfigAPI as IConfigAPI } from '../types/interfaces';
import { Backend } from './BackendAPI';

export class ConfigAPI implements IConfigAPI {
  async getConfig(): Promise<Config | undefined> {
    if (window.isDevelopment) {
      return window.devData.config;
    }
    const config = await Backend.invoke<Config>('getConfig');
    if (config) return config;
  }
  async getItem<T>(sectionID: ConfigSections, prop: keyof Config[ConfigSections]): Promise<T | undefined> {
    if (window.isDevelopment) {
      return window.devData.config[sectionID][prop];
    }
    const configItem = await Backend.invoke<Config>('getConfigItem');
    if (configItem) return configItem[sectionID][prop];
  }
  async setItem(sectionID: ConfigSections, prop: keyof Config[ConfigSections], value: any): Promise<void> {
    if (window.isDevelopment) {
      window.devData.config = {
        ...window.devData.config,
        [sectionID]: {
          [prop]: value
        }
      }
    }
    const response = await Backend.invoke<void>('setConfigItem');
    if (response) {
      console.log(`Successfully set the item ${sectionID}.${prop} to ${value}`);
    }
  }
  public async getCredentials(): Promise<AWSCredentials | undefined> {
    if (window.isDevelopment) {
      return window.devData.config.credentials;
    }
    const creds = await Backend.invoke<AWSCredentials>('get_aws_credentials');
    if (creds) return creds;
  }
}
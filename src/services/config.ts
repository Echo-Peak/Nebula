import conf from 'conf';
import { invoke } from '@tauri-apps/api/tauri';
import type { Config, ConfigInterface } from '../types/config';

let cachedConfig: any;
async function createConfig() {
    if (cachedConfig) return cachedConfig;
    let cwd = process.cwd();
    if (process.env.NODE_ENV !== 'development') {
        cwd = await invoke('getConfigDir');
    }
    const token = await invoke('getEncryptionToken') as string;
    if (!token) throw new Error("Unable to parse token");
    cachedConfig = new conf({ cwd, encryptionKey: token });
    return cachedConfig;
}


export const getConfig = async (): Promise<ConfigInterface> => {
    const config = await createConfig();
    return {
        async get<T>(prop: string): Promise<T | undefined> {
            const val = config.get(prop) as any;
            console.log(`Getting the prop "${prop}". Value is: "${val}"`);
            if (val === undefined) return undefined;
            return val;
        },
        async set(prop: string, propValue: any) {
            console.log(`Setting the prop "${prop}". Value is: "${propValue}"`);
            config.set(prop, propValue);
        }
    }
}
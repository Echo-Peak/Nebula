export type Config = {
  credentials: {
    accessKeyId: string;
    secretAccessKey: string;
    region: string
  },
  preferences: {
    cache: string,
    maxCacheLimit: number,
    exclusions: string[]
  },
  betaFeatures: {
    enableFilesystemSync: boolean,
    enableDarkMode: boolean,
    enableViewer: boolean
  }
}

export interface ConfigInterface {
  get<GetType>(prop: string): Promise<GetType | undefined>
  set(prop: string, propValue: any): Promise<void>
}
export type S3File = {
  S3Path: string,
  localPath: string,
  cached: boolean,
  storageClass: keyof S3StorageClasses,
  size: number,
  created: number,
  modified: number,
  id: number,
  selected: boolean,
}

export type S3Dir = {
  S3Path: string,
  name: string
}

export type S3SearchResult = {
  files: S3File[],
  dirs: S3Dir[]
}

export const enum S3StorageClasses {
  STANDARD = "STANDARD",
  REDUCED_REDUNDANCY = "REDUCED_REDUNDANCY",
  STANDARD_IA = "STANDARD_IA",
  ONEZONE_IA = "ONEZONE_IA",
  INTELLIGENT_TIERING = "INTELLIGENT_TIERING",
  GLACIER = "GLACIER",
  DEEP_ARCHIVE = "DEEP_ARCHIVE",
  OUTPOSTS = "OUTPOSTS",
  GLACIER_IR = "GLACIER_IR",
}

export interface S3API {
  listBuckets(): Promise<string[]>,
  // openFolder(previousDirectory: string, nextDirectory: string): Promise<S3SearchResult>
}


import { S3Api } from '../api/S3Api';
import { writable } from 'svelte/store';
import { createAsyncWritable } from '../utils/createAsyncWritable';
import type { BucketItem, S3ObjectItem } from 'src/types/data';
import { localStorageHelper } from '../utils/localStorageHelper';

const S3 = new S3Api();

const cachedCurrentBucket = localStorageHelper.get<BucketItem>('currentBucket');

export const currentBucket = writable<BucketItem | undefined>(cachedCurrentBucket);
const buckets = writable<BucketItem[]>([])
export const bucketStream = createAsyncWritable<BucketItem[]>(buckets, S3.getBuckets, []);

export const setCurrentBucket = (bucket: BucketItem) => {
  localStorageHelper.set('currentBucket', bucket);
  currentBucket.set(bucket);
}

export const currentPath = writable<string>("");
export const setPath = (path: string) => currentPath.set(path);

export const loadContents = async (bucket: BucketItem | undefined, path: string): Promise<S3ObjectItem[]> => {
  if (bucket) {
    if (window.isDevelopment) {
      const content = window.devData.bucketContents[bucket.name];
      if (content) return content;
    }

  }
  return [];
}




import { S3Api } from '../api/S3Api';
import { writable } from 'svelte/store';
import { createAsyncWritable } from '../utils/createAsyncWritable';
import type { BucketItem } from 'src/types/data';
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


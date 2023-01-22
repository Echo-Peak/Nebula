
//import { config } from './config';
import type { BucketItem } from 'src/types/data';
import type { S3Api as IS3Api } from '../types/interfaces';
import { Backend } from './BackendAPI';

export class S3Api implements IS3Api {
    async getBuckets(): Promise<BucketItem[]> {
        if (window.devData) {
            return window.devData.buckets;
        }
        try {
            const buckets = await Backend.invoke<BucketItem[]>("S3.getBuckets");
            if (buckets && buckets.length) return buckets;
        } catch (err) {
            console.log(err);
        }
        return [];
    }
    async listContents(S3Path: string): Promise<string[]> {
        return [];
    }
    async downloadFile(S3Path: string): Promise<boolean> {
        return false;
    }
    async setCreds(accessKey: string, secret: string, region: string): Promise<boolean> {
        return false;
    }

}
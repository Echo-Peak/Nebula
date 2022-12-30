import AWS from 'aws-sdk';
import { config } from './config';
import type { S3API, S3SearchResult } from '../types/data';

export class S3Api implements S3API {
    private S3Client: AWS.S3;
    constructor() {
        this.authenticate();
    }
    authenticate() {
        const accessID = config.get<string>('S3.accessKey') || process.env.AWS_ACCESS_KEY || '';
        const secretKey = config.get<string>('S3.secretKey') || process.env.AWS_SECRET || '';
        const region = config.get<string>('S3.region') || process.env.AWS_REGION;

        AWS.config.update({
            secretAccessKey: secretKey,
            accessKeyId: accessID
        });
        this.S3Client = new AWS.S3();
    }
    async listBuckets(): Promise<string[]> {
        try {
            const result = await this.S3Client.listBuckets().promise();
            if (result.Buckets) {
                return result.Buckets.map(e => {
                    if (e.Name) return e.Name;
                    return '';
                }).filter(e => e.length > 0);
            }
            return [];
        } catch (err) {
            console.error(err);
        }
        return [];
    }
    async openFolder(previousDirectory: string, nextDirectory: string): Promise<S3SearchResult> {

    }
}
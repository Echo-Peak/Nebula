import AWS from 'aws-sdk';
import {config} from './config';

export class S3 {
    constructor(){
        this.authenticate();
    }
    authenticate(){
        const accessID = config.get('S3.accessKey');
        const secretKey = config.get('S3.secretKey');
        const region = config.get('S3.region');

        AWS.config.update({
            secretAccessKey: secretKey,
            accessKeyId: accessID
        })
    }
}
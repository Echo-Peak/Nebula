import { S3Api } from '../../services/S3Api';

const S3 = new S3Api();
export async function load() {
  const results = await S3.listBuckets();
  console.log(results);
  return {
    buckets: results
  }
}
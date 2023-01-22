<script lang="ts">
  import { bucketStream, setCurrentBucket } from "@stores/S3";
  import type { BucketItem } from "src/types/data";
  import { Backend } from "../../api/BackendAPI";
  import { goto } from "$app/navigation";
  import { Routes } from "../../Routes";
  import { logger } from "../../api/LoggerAPI";

  let buckets: BucketItem[] = [];
  bucketStream.subscribe((bucketList) => {
    logger.debug("Bucket list", bucketList);
    buckets = bucketList;
  });

  const openInExplorer = (bucket: BucketItem) => {
    setCurrentBucket(bucket);
    goto(Routes.explorer, { state: bucket });
  };
  const openInAWSConsole = (bucket: BucketItem) => {
    const url = `https://s3.console.aws.amazon.com/s3/buckets/${bucket.name}?region=${bucket.region}&tab=objects`;
    if (window.isDevelopment) {
      window.open(url, "_blank");
    } else {
      Backend.openURL(url);
    }
  };
</script>

<div class="grid grid-cols-6 gap-4 ml-4 mt-4">
  {#each buckets as bucket}
    <div
      class="card w-80 bg-base-100 shadow-xl scale-100 hover:scale-105 ease-in duration-100"
    >
      <div class="card-body items-center text-center">
        <h2 class="card-title">{bucket.name}</h2>
        <div class="card-actions">
          <button
            class="btn btn-primary"
            on:click={() => openInExplorer(bucket)}>Explore</button
          >
          <button
            class="btn btn-primary"
            on:click={() => openInAWSConsole(bucket)}>Open in S3</button
          >
        </div>
      </div>
    </div>
  {/each}
</div>

<script lang="ts">
  /** @type {buckets: string[]} */
  export let data: { buckets: string[] };
  console.log(data);
</script>

<div class="grid grid-cols-6 gap-4 ml-4 mt-4">
  {#each data.buckets as bucket}
    <div
      class="card w-80 bg-base-100 shadow-xl scale-100 hover:scale-105 ease-in duration-100"
    >
      <div class="card-body items-center text-center">
        <h2 class="card-title">{bucket}</h2>
        <div class="card-actions">
          <button class="btn btn-primary">Explore</button>
          <button class="btn btn-primary">Open in S3</button>
        </div>
      </div>
    </div>
  {/each}
</div>

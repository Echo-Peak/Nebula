<script>
  import { hideModal } from "@stores/app";
  import { Eye } from "lucide-svelte";

  const maxCache = 10; //GB
</script>

<h3 class="font-medium leading-tight text-3xl mt-0 mb-2">AWS settings</h3>

<div>
  <section>
    <h2>Access Key</h2>

    <div class="relative inline">
      <input
        type="password"
        placeholder="AWS access key"
        class="input input-bordered w-full max-w-xs"
      />
      <div class="absolute inset-y-0 right-0 pr-3">
        <Eye style="cursor: pointer" on:click={() => false} />
      </div>
    </div>
  </section>

  <section>
    <h2>Secret</h2>

    <div class="relative inline">
      <input
        type="password"
        placeholder="AWS Secret"
        class="input input-bordered w-full max-w-xs"
      />
      <div class="absolute inset-y-0 right-0 pr-3">
        <Eye style="cursor: pointer" on:click={() => false} />
      </div>
    </div>
  </section>

  <section>
    <h2>Region</h2>

    <div class="relative inline">
      <input
        type="text"
        placeholder="AWS Region"
        class="input input-bordered w-full max-w-xs"
      />
    </div>
  </section>

  <section class="mb-10">
    <h2>Local cache</h2>

    <input
      type="range"
      min="1"
      max={maxCache}
      value="1"
      class="range"
      step="1"
    />
    <div class="w-full flex justify-between text-xs px-2">
      {#each Array(maxCache) as _, GB}
        <span>{GB + 1}GB</span>
      {/each}
    </div>
  </section>
</div>

<div>
  <button class="btn btn-active btn-primary" on:click={hideModal}>Save</button>
  <button class="btn" on:click={hideModal}>Close</button>
</div>

<style>
  h2 {
    font-size: 20px;
    margin-top: 10px;
    margin-bottom: 10px;
  }
</style>

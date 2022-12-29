<script>
  import { hideModal } from "@stores/app";
</script>

<h3 class="font-medium leading-tight text-3xl mt-0 mb-2">User Preferences</h3>

<div>
  <section>
    <div>
      <label class="label cursor-pointer">
        <span class="label-text">Dark mode</span>
        <input type="checkbox" class="toggle" checked />
      </label>
    </div>
  </section>

  <section class="mb-10">
    <h2>Exclusions</h2>
    <div class="relative inline">
      <input
        type="text"
        placeholder="Directories to exclude from querying"
        class="input input-bordered w-full max-w-xs"
      />
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

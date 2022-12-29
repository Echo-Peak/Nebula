<script>
  import { hideModal } from "@stores/app";
</script>

<h3 class="font-medium leading-tight text-3xl mt-0 mb-2">AWS settings</h3>

<div>
  <section>
    <h2>App version</h2>

    <div class="relative inline">
      <span>v1.0.0</span>
    </div>
  </section>

  <section class="mb-6">
    <h2>Build number</h2>

    <div class="relative inline">
      <span>1028</span>
    </div>
  </section>
</div>

<div>
  <button class="btn" on:click={hideModal}>Close</button>
</div>

<style>
  h2 {
    font-size: 20px;
    margin-top: 10px;
    margin-bottom: 10px;
  }
</style>

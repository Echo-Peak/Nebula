<script>
  import { isModalOpen, modalView, ModalViews } from "@stores/app";
  import AWSSettings from "./AWSSettings.svelte";
  import UserPreferences from "./UserPreferences.svelte";
</script>

<div class={`modal ${$isModalOpen ? "modal-open" : ""}`}>
  <div class="modal-box">
    {#if $modalView === ModalViews.AWS}
      <AWSSettings />
    {/if}
    {#if $modalView === ModalViews.USER}
      <UserPreferences />
    {/if}
  </div>
</div>

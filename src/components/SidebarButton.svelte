<script>
  import * as allIcons from "lucide-svelte";
  export let name = "";
  export let iconName = "";
  export let onClick = () => {};

  const icon = allIcons[iconName];
</script>

<div class="flex flex-col items-center py-4  sidebar-button" on:click={onClick}>
  <svelte:component this={icon} style="color:white" />
  <span>{name}</span>
</div>

<style>
  .sidebar-button {
    width: 100%;
    text-align: center;
    cursor: pointer;
  }
  .sidebar-button span {
    margin-top: 10px;
    text-align: center;
    color: white;
  }
</style>

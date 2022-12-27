<script>
  import Header from "$lib/Header.svelte";
  import { currentView, currentConfig } from "../../stores/app";

  //currentView.set("Config");
</script>

<Header />
<div class="container">
  <div class="flex" style="margin-top:60px">
    <div class="flex-initial w-64">
      <h1 />
    </div>
    <div class="flex-initial w-64">
      <h2>dfsdff</h2>
    </div>
  </div>
  <!-- {#each Object.keys($currentConfig) as section}
    <h1 class="font-medium leading-tight text-5xl mt-0 mb-2 text-blue-600">
      {section}
    </h1>
    {#each Object.entries($currentConfig[section]) as [settingName, settingValue]}
      {#if typeof settingValue === "boolean"}
        <label class="label cursor-pointer">
          <span class="label-text">{settingName}</span>
          <input
            type="checkbox"
            checked={settingValue}
            class="checkbox checkbox-primary"
          />
        </label>
      {/if}
    {/each}
  {/each} -->
</div>

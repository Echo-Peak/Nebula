<script lang="ts">
  import { hideModal } from "@stores/app";
  import { Eye, EyeOff } from "lucide-svelte";
  import { creds, saveCreds } from "@stores/config";

  let accessID = "";
  let secret = "";
  let region = "";
  let showAccessKey = false;
  let showPassword = false;

  creds.subscribe((currentCreds) => {
    if (currentCreds !== undefined) {
      accessID = currentCreds.accessKeyId;
      secret = currentCreds.secretAccessKey;
      region = currentCreds.region;
    }
  });

  const onSave = () => {
    saveCreds({
      accessKeyId: accessID,
      secretAccessKey: secret,
      region: region,
    });
    hideModal();
  };
</script>

<h3 class="font-medium leading-tight text-3xl mt-0 mb-2">AWS settings</h3>

<div>
  <section>
    <h2>Access Key</h2>

    <div class="relative inline">
      <input
        type={showAccessKey ? "text" : "password"}
        value={accessID}
        placeholder="AWS access key"
        class="input input-bordered w-full max-w-xs"
        on:change={(e) => (accessID = e.target.value)}
      />
      <div class="absolute inset-y-0 right-0 pr-3">
        <button
          on:click={() => (showAccessKey = !showAccessKey)}
          title={showAccessKey ? "Hide" : "Show"}
        >
          {#if showAccessKey}
            <EyeOff style="cursor: pointer" />
          {:else}
            <Eye style="cursor: pointer" />
          {/if}
        </button>
      </div>
    </div>
  </section>

  <section>
    <h2>Secret</h2>

    <div class="relative inline">
      <input
        type={showPassword ? "text" : "password"}
        placeholder="AWS Secret"
        value={secret}
        class="input input-bordered w-full max-w-xs"
        on:change={(e) => (secret = e.target.value)}
      />
      <div class="absolute inset-y-0 right-0 pr-3">
        <button
          on:click={() => (showPassword = !showPassword)}
          title={showPassword ? "Hide" : "Show"}
        >
          {#if showPassword}
            <EyeOff style="cursor: pointer" />
          {:else}
            <Eye style="cursor: pointer" />
          {/if}
        </button>
      </div>
    </div>
  </section>

  <section>
    <h2>Region</h2>

    <div class="relative inline">
      <input
        type="text"
        placeholder="AWS Region"
        value={region}
        on:change={(e) => (region = e.target.value)}
        class="input input-bordered w-full max-w-xs"
      />
    </div>
  </section>
</div>

<div class="pt-2">
  <button class="btn btn-active btn-primary" on:click={onSave}>Save</button>
  <button class="btn" on:click={hideModal}>Close</button>
</div>

<style>
  h2 {
    font-size: 20px;
    margin-top: 10px;
    margin-bottom: 10px;
  }
</style>

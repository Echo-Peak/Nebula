<script>
  import { storageClasses } from "../services/storageclasses";
  export let S3Path = "";
  export let local = "";
  export let cached = false;
  export let storageClass = storageClasses.STANDARD;
  export let created = 0;
  export let modified = 0;
  export let size = 0;
  export let selected = false;

  function bytesToHumanReadable(bytes) {
    const thresholds = [
      {
        threshold: 1e3,
        label: "B",
      },
      {
        threshold: 1e6,
        label: "KB",
      },
      {
        threshold: 1e9,
        label: "MB",
      },
      {
        threshold: 1e12,
        label: "GB",
      },
      {
        threshold: 1e15,
        label: "TB",
      },
    ];

    for (const { threshold, label } of thresholds) {
      if (bytes < threshold) {
        return (bytes / (threshold / 1e3)).toFixed(2) + label;
      }
    }
  }
  const friendlySize = bytesToHumanReadable(size);
</script>

<th>
  <label>
    <input type="checkbox" class="checkbox" checked={selected} />
  </label>
</th>

<td>
  <div class="flex items-center space-x-3">
    <div>
      <div class="font-bold">Remote: {S3Path}</div>
      <div class="text-sm opacity-50">Local: {local}</div>
    </div>
  </div>
</td>

<td>
  {#if cached}
    <span class="text-blue-600">YES</span>
  {:else}
    <span class="text-red-600">NO</span>
  {/if}
</td>

<td>
  {storageClass}
</td>

<td>{friendlySize}</td>

<td>{new Date(created)}</td>

<td>{new Date(modified)}</td>

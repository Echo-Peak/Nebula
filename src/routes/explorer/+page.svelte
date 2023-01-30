<script lang="ts">
  import { currentPath, currentBucket, loadContents } from "@stores/S3";
  import type { S3ObjectItem, Selectable } from "../../types/data";
  import Table from "../../lib/Table.svelte";

  let contents: Selectable[] = [];

  (async () => {
    const data = await loadContents($currentBucket, $currentPath);
    if (data) {
      contents = data.map((item) => ({
        ...item,
        selected: false,
      }));

      console.log(contents);
    }
  })();
  // loadContents(_contents: S3ObjectItem[] => {
  //   contents = _contents;
  //   console.log(contents, "HISTORY");
  // })
</script>

<div>
  {#if contents.length > 0}
    <Table list={contents} />
  {:else}
    <h2 style="color:white">No content available</h2>
  {/if}
</div>

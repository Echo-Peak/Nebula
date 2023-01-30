<script lang="ts">
  import type { S3ObjectItem } from "../types/data";
  export let list: S3ObjectItem[];
</script>

<div class="overflow-x-auto w-full h-full">
  <table class="table w-full h-full">
    <!-- head -->
    <thead>
      <tr>
        <th class="no-border-radius">
          <label>
            <input type="checkbox" class="checkbox" on:change={() => false} />
          </label>
        </th>
        <th>S3Path</th>
        <th>cached</th>
        <th>storageClass</th>
        <th>size</th>
        <th>created</th>
        <th class="no-border-radius">modified</th>
      </tr>
    </thead>
    <tbody>
      <!-- row 1 -->
      {#each list as item}
        <tr>
          <CreateTableEntry
            S3Path={item.S3Path}
            local={item.localPath}
            cached={item.cached}
            size={item.size}
            storageClass={item.storageClass}
            created={item.created}
            modified={item.modified}
            selected={item.selected}
          />
        </tr>
      {/each}
      <!-- row 2 -->
    </tbody>
  </table>
</div>

<style>
  .no-border-radius {
    border-radius: 0px;
  }
</style>

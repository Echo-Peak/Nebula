<script lang="ts">
</script>

<div class="overflow-x-auto w-full h-full">
  <table class="table w-full h-full">
    <!-- head -->
    <thead>
      <tr>
        <th class="no-border-radius">
          <label>
            <input
              type="checkbox"
              class="checkbox"
              checked={selectAllElements}
              on:change={toggleAllSelected}
            />
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
    <tbody on:mouseleave={cleanup}>
      <!-- row 1 -->
      <!-- {#each sampleFiles as item}
        <tr
          on:click={() => setClickElement(item.id)}
          on:mouseenter={() => setHoverElement(item.id)}
          on:contextmenu|preventDefault={(e) => openContextMenu(e, item)}
          class={item.id === currentHoverElement ? "active" : ""}
        >
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
      {/each} -->
      <!-- row 2 -->
    </tbody>
  </table>
</div>

<style>
  .no-border-radius {
    border-radius: 0px;
  }
</style>

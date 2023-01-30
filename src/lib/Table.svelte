<script lang="ts">
  import type { S3ObjectItem } from "../types/data";
  import CreateTableEntry from "@components/CreateTableEntry.svelte";
  import {
    contextMenuContextViews,
    showContextMenu,
    setContextMenuContext,
  } from "@stores/app";

  type Selectable = S3ObjectItem & {
    selected: boolean;
  };
  export let list: Selectable[];

  const openContextMenu = (evt: any, selectedItem: any) => {
    const posX = evt.clientX;
    const posY = evt.clientY;
    const contextView = contextMenuContextViews.EXPLORER;
    setContextMenuContext(posX, posY, "explorer", selectedItem);
    showContextMenu();
  };
  let currentHoverElement: any = null;
  let currentClickElement = null;
  let selectAllElements = false;
  const setHoverElement = (id: any) => {
    currentHoverElement = id;
  };
  const toggleAllSelected = () => {
    selectAllElements = !selectAllElements;
    list = list.map((e) => ({
      ...e,
      selected: selectAllElements,
    }));
  };
  const setClickElement = (id: string) => {
    //const index = list.findIndex((entry) => id == entry.id);
    // if (index >= 0) {
    //   //list[index].selected = !list[index].selected;
    // }
  };
  const cleanup = () => {
    currentHoverElement = null;
  };
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
      {#each list as item}
        <tr>
          <CreateTableEntry
            S3Path={item.key}
            local={item.localPath}
            cached={item.cached}
            size={item.size}
            storageClass={item.storageClass}
            created={item.created}
            selected={item.selected}
          />
        </tr>
      {/each}
      <!-- row 2 -->
    </tbody>
  </table>
</div>

<!-- on:click={() => setClickElement(item.id)}
          on:mouseenter={() => setHoverElement(item.id)}
          on:contextmenu|preventDefault={(e) => openContextMenu(e, item)}
          class={item.id === currentHoverElement ? "active" : ""} -->
<style>
  .no-border-radius {
    border-radius: 0px;
  }
</style>

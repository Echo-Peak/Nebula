<script>
  import { setContext, createEventDispatcher } from "svelte";
  import {
    isContextMenuOpen,
    hideContextMenu,
    contextMenuContext,
    contextMenuContextViews,
  } from "@stores/app";
  import ExplorerContextMenuOptions from "./ExplorerMenu.svelte";
  import { fade } from "svelte/transition";
  let posX = 0;
  let posY = 0;
  let contextView = "";
  contextMenuContext.subscribe((e) => {
    posX = e.pos.x;
    posY = e.pos.y;
    contextView = e.contextView;
    console.log(e);
  });

  function clickOutside(node) {
    const handleClick = (event) => {
      if (node && !node.contains(event.target) && !event.defaultPrevented) {
        node.dispatchEvent(new CustomEvent("click_outside", node));
      }
    };
    document.addEventListener("click", handleClick, true);

    return {
      destroy() {
        document.removeEventListener("click", handleClick, true);
      },
    };
  }
</script>

{#if $isContextMenuOpen}
  <div
    transition:fade={{ duration: 100 }}
    style="top: {posY}px; left: {posX}px;"
    on:click={hideContextMenu}
    use:clickOutside
    on:click_outside={hideContextMenu}
  >
    {#if contextView === contextMenuContextViews.EXPLORER}
      <ExplorerContextMenuOptions />
    {/if}
  </div>
{/if}

<style>
  div {
    z-index: 10000;
    position: absolute;
    display: grid;

    box-shadow: 2px 2px 10px 0px rgba(31 41 55);
    background: rgba(31 41 55);
    border-radius: 6px;
  }
</style>

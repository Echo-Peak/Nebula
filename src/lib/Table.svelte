<script>
  import CreateTableEntry from "@components/CreateTableEntry.svelte";
  import {
    contextMenuContextViews,
    showContextMenu,
    setContextMenuContext,
  } from "@stores/app";
  let sampleFiles = [
    {
      S3Path: "releases/staging/x64/app.exe",
      localPath: "",
      cached: false,
      storageClass: "GLACIER",
      size: 8918172,
      created: 1672279397159,
      modified: 1672287691006,
      id: 1,
      selected: false,
    },
    {
      S3Path: "releases/staging/x64/app.exe",
      localPath:
        "c:/Users/User/AppData/Roaming/Nebula/releases/staging/x64/app.exe",
      cached: true,
      storageClass: "STANDARD",
      size: 8918172,
      created: 1672279397159,
      modified: 1672287691006,
      id: 2,
      selected: false,
    },
    {
      S3Path: "releases/staging/x64/app.exe",
      localPath: "",
      cached: false,
      storageClass: "GLACIER",
      size: 8918172,
      created: 1672279397159,
      modified: 1672287691006,
      id: 3,
      selected: false,
    },
    {
      S3Path: "releases/staging/x64/app.exe",
      localPath: "",
      cached: false,
      storageClass: "GLACIER",
      size: 8918172,
      created: 1672279397159,
      modified: 1672287691006,
      id: 5,
      selected: false,
    },
    {
      S3Path: "releases/staging/x64/app.exe",
      localPath: "",
      cached: false,
      storageClass: "GLACIER",
      size: 8918172,
      created: 1672279397159,
      modified: 1672287691006,
      id: 6,
      selected: false,
    },
    {
      S3Path: "releases/staging/x64/app.exe",
      localPath: "",
      cached: false,
      storageClass: "GLACIER",
      size: 8918172,
      created: 1672279397159,
      modified: 1672287691006,
      id: 7,
      selected: false,
    },
    {
      S3Path: "releases/staging/x64/app.exe",
      localPath: "",
      cached: false,
      storageClass: "GLACIER",
      size: 8918172,
      created: 1672279397159,
      modified: 1672287691006,
      id: 8,
      selected: false,
    },
    {
      S3Path: "releases/staging/x64/app.exe",
      localPath: "",
      cached: false,
      storageClass: "GLACIER",
      size: 8918172,
      created: 1672279397159,
      modified: 1672287691006,
      id: 9,
      selected: false,
    },
    {
      S3Path: "releases/staging/x64/app.exe",
      localPath: "",
      cached: false,
      storageClass: "GLACIER",
      size: 8918172,
      created: 1672279397159,
      modified: 1672287691006,
      id: 10,
      selected: false,
    },
    {
      S3Path: "releases/staging/x64/app.exe",
      localPath: "",
      cached: false,
      storageClass: "GLACIER",
      size: 8918172,
      created: 1672279397159,
      modified: 1672287691006,
      id: 11,
      selected: false,
    },
    {
      S3Path: "releases/staging/x64/app.exe",
      localPath: "",
      cached: false,
      storageClass: "GLACIER",
      size: 8918172,
      created: 1672279397159,
      modified: 1672287691006,
      id: 12,
      selected: false,
    },
    {
      S3Path: "releases/staging/x64/app.exe",
      localPath: "",
      cached: false,
      storageClass: "GLACIER",
      size: 8918172,
      created: 1672279397159,
      modified: 1672287691006,
      id: 13,
      selected: false,
    },
    {
      S3Path: "releases/staging/x64/app.exe",
      localPath: "",
      cached: false,
      storageClass: "GLACIER",
      size: 8918172,
      created: 1672279397159,
      modified: 1672287691006,
      id: 14,
      selected: false,
    },
    {
      S3Path: "releases/staging/x64/app.exe",
      localPath: "",
      cached: false,
      storageClass: "GLACIER",
      size: 8918172,
      created: 1672279397159,
      modified: 1672287691006,
      id: 15,
      selected: false,
    },
    {
      S3Path: "releases/staging/x64/app.exe",
      localPath: "",
      cached: false,
      storageClass: "GLACIER",
      size: 8918172,
      created: 1672279397159,
      modified: 1672287691006,
      id: 16,
      selected: false,
    },
  ];

  const openContextMenu = (evt, selectedItem) => {
    const posX = evt.clientX;
    const posY = evt.clientY;
    const contextView = contextMenuContextViews.EXPLORER;
    setContextMenuContext(posX, posY, contextView, selectedItem);
    showContextMenu();
  };
  let currentHoverElement = null;
  let currentClickElement = null;
  let selectAllElements = false;

  const setHoverElement = (id) => {
    currentHoverElement = id;
  };

  const toggleAllSelected = () => {
    selectAllElements = !selectAllElements;
    sampleFiles = sampleFiles.map((e) => ({
      ...e,
      selected: selectAllElements,
    }));
  };
  const setClickElement = (id) => {
    const index = sampleFiles.findIndex((entry) => id == entry.id);
    if (index >= 0) {
      sampleFiles[index].selected = !sampleFiles[index].selected;
    }
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
      {#each sampleFiles as item}
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

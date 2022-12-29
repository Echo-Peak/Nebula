<script>
  import CreateTableEntry from "@components/CreateTableEntry.svelte";
  import {
    contextMenuContextViews,
    showContextMenu,
    setContextMenuContext,
  } from "@stores/app";
  const sampleFiles = [
    {
      S3Path: "releases/staging/x64/app.exe",
      localPath: "",
      cached: false,
      storageClass: "GLACIER",
      size: 8918172,
      created: 1672279397159,
      modified: 1672287691006,
      id: 1,
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
    },
  ];

  const openContextMenu = (evt, selectedItem) => {
    const posX = evt.clientX;
    const posY = evt.clientY;
    const contextView = contextMenuContextViews.EXPLORER;
    setContextMenuContext(posX, posY, contextView, selectedItem);
    showContextMenu();
  };
  let currentActiveEntry = null;
  const setIsActive = (id) => {
    currentActiveEntry = id;
  };
  const cleanup = () => {
    currentActiveEntry = null;
  };
</script>

<div class="overflow-x-auto w-full h-full">
  <table class="table w-full h-full">
    <!-- head -->
    <thead>
      <tr>
        <th class="no-border-radius">
          <label>
            <input type="checkbox" class="checkbox" />
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
          on:mouseenter={() => setIsActive(item.id)}
          on:contextmenu|preventDefault={(e) => openContextMenu(e, item)}
          class={item.id === currentActiveEntry ? "active" : ""}
        >
          <CreateTableEntry
            S3Path={item.S3Path}
            local={item.localPath}
            cached={item.cached}
            size={item.size}
            storageClass={item.storageClass}
            created={item.created}
            modified={item.modified}
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

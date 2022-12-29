<script>
  import CreateTableEntry from "@components/CreateTableEntry.svelte";
  const sampleFiles = [
    {
      S3Path: "releases/staging/x64/app.exe",
      localPath: "",
      cached: false,
      storageClass: "GLACIER",
      size: 8918172,
      created: 1672279397159,
      modified: 1672287691006,
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
    },
    {
      S3Path: "releases/staging/x64/app.exe",
      localPath: "",
      cached: false,
      storageClass: "GLACIER",
      size: 8918172,
      created: 1672279397159,
      modified: 1672287691006,
    },
    {
      S3Path: "releases/staging/x64/app.exe",
      localPath: "",
      cached: false,
      storageClass: "GLACIER",
      size: 8918172,
      created: 1672279397159,
      modified: 1672287691006,
    },
    {
      S3Path: "releases/staging/x64/app.exe",
      localPath: "",
      cached: false,
      storageClass: "GLACIER",
      size: 8918172,
      created: 1672279397159,
      modified: 1672287691006,
    },
    {
      S3Path: "releases/staging/x64/app.exe",
      localPath: "",
      cached: false,
      storageClass: "GLACIER",
      size: 8918172,
      created: 1672279397159,
      modified: 1672287691006,
    },
    {
      S3Path: "releases/staging/x64/app.exe",
      localPath: "",
      cached: false,
      storageClass: "GLACIER",
      size: 8918172,
      created: 1672279397159,
      modified: 1672287691006,
    },
    {
      S3Path: "releases/staging/x64/app.exe",
      localPath: "",
      cached: false,
      storageClass: "GLACIER",
      size: 8918172,
      created: 1672279397159,
      modified: 1672287691006,
    },
    {
      S3Path: "releases/staging/x64/app.exe",
      localPath: "",
      cached: false,
      storageClass: "GLACIER",
      size: 8918172,
      created: 1672279397159,
      modified: 1672287691006,
    },
    {
      S3Path: "releases/staging/x64/app.exe",
      localPath: "",
      cached: false,
      storageClass: "GLACIER",
      size: 8918172,
      created: 1672279397159,
      modified: 1672287691006,
    },
    {
      S3Path: "releases/staging/x64/app.exe",
      localPath: "",
      cached: false,
      storageClass: "GLACIER",
      size: 8918172,
      created: 1672279397159,
      modified: 1672287691006,
    },
    {
      S3Path: "releases/staging/x64/app.exe",
      localPath: "",
      cached: false,
      storageClass: "GLACIER",
      size: 8918172,
      created: 1672279397159,
      modified: 1672287691006,
    },
    {
      S3Path: "releases/staging/x64/app.exe",
      localPath: "",
      cached: false,
      storageClass: "GLACIER",
      size: 8918172,
      created: 1672279397159,
      modified: 1672287691006,
    },
    {
      S3Path: "releases/staging/x64/app.exe",
      localPath: "",
      cached: false,
      storageClass: "GLACIER",
      size: 8918172,
      created: 1672279397159,
      modified: 1672287691006,
    },
    {
      S3Path: "releases/staging/x64/app.exe",
      localPath: "",
      cached: false,
      storageClass: "GLACIER",
      size: 8918172,
      created: 1672279397159,
      modified: 1672287691006,
    },
  ];
</script>

<div class="overflow-x-auto w-full h-full">
  <table class="table w-full h-full">
    <!-- head -->
    <thead>
      <tr>
        <th>
          <label>
            <input type="checkbox" class="checkbox" />
          </label>
        </th>
        <th>S3Path</th>
        <th>cached</th>
        <th>storageClass</th>
        <th>size</th>
        <th>created</th>
        <th>modified</th>
        <th />
      </tr>
    </thead>
    <tbody>
      <!-- row 1 -->
      {#each sampleFiles as item}
        <tr>
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

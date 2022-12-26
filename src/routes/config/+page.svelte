<script>
  import Header from "$lib/Header.svelte";
  import { currentView } from "../../stores/app";

  currentView.set("Config");
</script>

<Header />
<div>
  <h2>config page</h2>
</div>

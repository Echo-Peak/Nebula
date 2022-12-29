<script>
  import { Save, Trash2, File } from "lucide-svelte";
</script>

<ul>
  <li class="flex p-4" style="border-radius:6px">
    <Trash2 color="white" />
    <span class="pl-4">Delete item</span>
  </li>
  <li class="flex p-4" style="border-radius:6px">
    <Save color="white" />
    <span class="pl-4">Cache</span>
  </li>

  <li class="flex p-4" style="border-radius:6px">
    <File color="white" />
    <span class="pl-4">Show in file explorer</span>
  </li>
</ul>

<style>
  li {
    cursor: pointer;
    color: white;
  }
  li:hover {
    background: rgb(75 85 99);
  }
</style>

<script>
  import SearchIcon from "./icons/SearchIcon.svelte";
</script>

<span class="relative w-full">
  <input
    aria-label="search"
    type="search"
    id="search"
    placeholder="Search"
    class="w-full bg-gray-900 text-white transition border border-transparent focus:outline-none focus:border-gray-400 rounded py-3 px-2 pl-10 appearance-none leading-normal"
  />
  <div class="absolute search-icon" style="top: 1rem; left: .8rem;">
    <SearchIcon />
  </div>
</span>

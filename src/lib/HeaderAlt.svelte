<script>
  import SearchBar from "@components/SearchBar.svelte";
</script>

<header>
  <!--Nav-->
  <nav
    aria-label="menu nav"
    class="bg-gray-800 pt-2 md:pt-1 pb-1 px-1 mt-0 h-auto fixed w-full z-20 top-0"
  >
    <div class="flex flex-wrap items-center">
      <div
        class="flex flex-shrink md:w-1/6 justify-center md:justify-start text-white"
      >
        <img src="https://place-hold.it/100x64" class="ml-12" />
      </div>
      <div class="flex flex-1 md:w-1/3 justify-start  text-white px-2 ">
        <div>
          <span class="block">Local directory...</span>
          <span class="block">S3 directory...</span>
        </div>
      </div>
      <div
        class="flex flex-1 md:w-1/3 justify-center md:justify-start text-white px-2"
      >
        <SearchBar />
      </div>

      <div
        class="flex flex-1 md:w-1/2 justify-center md:justify-end text-white px-2"
      />
    </div>
  </nav>
</header>

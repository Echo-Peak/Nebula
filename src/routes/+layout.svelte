<script>
  import "../app.css";
  import Modals from "$lib/modals/Main.svelte";
  import ContextMenus from "$lib/contextMenu/Main.svelte";
  import HeaderAlt from "$lib/Header.svelte";
  import SidebarAlt from "$lib/Sidebar.svelte";
  import ActionBar from "$lib/ActionBar.svelte";
</script>

<Modals />
<ContextMenus />
<HeaderAlt />

<main>
  <div class="flex flex-col md:flex-row">
    <SidebarAlt />

    <section class="md:mt-10">
      <ActionBar />
      <div class="main-content flex-1 bg-gray-100">
        <div class="inner-container">
          <div class="">
            <slot />
          </div>
        </div>
      </div>
    </section>
  </div>
</main>

<style>
  .inner-container {
    /* background-color: rgb(75 85 99); */
    background-color: rgb(31 41 55);
    width: calc(100vw - 196px);
    height: calc(100vh - 140px);
    overflow-y: scroll;
  }
</style>

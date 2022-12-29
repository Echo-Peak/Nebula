<script>
  import "../app.css";
  import Header from "../lib/Header.svelte";
  import Modals from "$lib/modals/Main.svelte";
  import HeaderAlt from "$lib/HeaderAlt.svelte";
  import SidebarAlt from "$lib/SidebarAlt.svelte";
</script>

<Modals />
<HeaderAlt />

<main>
  <div class="flex flex-col md:flex-row">
    <SidebarAlt />

    <section>
      <div class="main-content flex-1 bg-gray-100 md:mt-10">
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
    background-color: rgb(75 85 99);
    width: calc(100vw - 200px);
    height: calc(100vh - 90px);
    overflow-y: scroll;
  }
</style>

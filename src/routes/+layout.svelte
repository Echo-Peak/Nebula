<script>
  import "../app.css";
  import Header from "../lib/Header.svelte";
</script>

<main class="container">
  <Header />
  <div class="flex flex-row">
    <slot />
  </div>
</main>

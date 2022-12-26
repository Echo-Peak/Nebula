<script>
  import { currentView } from "../stores/app";
  import { MenuIcon, SearchIcon } from "svelte-feather-icons";
</script>

<div class="full-width">
  <div
    class="navbar bg-base-100 backdrop-filter backdrop-blur-lg bg-opacity-30"
  >
    <div class="navbar-start">
      <div class="dropdown">
        <label tabindex="0" class="btn btn-ghost btn-circle">
          <MenuIcon size="1.5x" />
        </label>
        <ul
          tabindex="0"
          class="menu menu-compact dropdown-content mt-3 p-2 shadow bg-base-100 rounded-box w-52"
        >
          <li><a href="/config">Open config</a></li>
        </ul>
      </div>
    </div>
    <div class="navbar-center">
      <span class="btn btn-ghost normal-case text-xl">{$currentView}</span>
    </div>
    <div class="navbar-end">
      <button class="btn btn-ghost btn-circle">
        <SearchIcon size="1.5x" />
      </button>
    </div>
  </div>
</div>

<style>
  .full-width {
    width: 100%;
    position: fixed;
  }
</style>

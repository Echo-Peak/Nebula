<script>
  import { Moon } from "lucide-svelte";
  import SidebarButton from "@components/SidebarButton.svelte";

  const goToDashboard = () => {};

  const openAWSModal = () => {};

  const openUserPreferences = () => {};
  const toggleDarkMode = () => {};
</script>

<div class="relative column-1 h-screen bg-custom-blue justify-center side-menu">
  <SidebarButton name="Home" iconName="Home" on:click={goToDashboard} />
  <SidebarButton name="AWS" iconName="Settings" on:click={openAWSModal} />
  <SidebarButton
    name="Preferences"
    iconName="Sliders"
    on:click={openUserPreferences}
  />

  <div class="flex flex-col items-center toggle-dark-mode">
    <button class="btn btn-circle" on:click={toggleDarkMode}>
      <Moon color="white" />
    </button>
  </div>
</div>

<style>
  .side-menu {
    background: #4338ca;
    width: 120px;
  }

  .toggle-dark-mode {
    position: absolute;
    bottom: 0;
    width: 100%;
    padding-bottom: 20px;
  }
</style>

<script>
  import { Moon } from "lucide-svelte";
  import SidebarButton from "@components/SidebarButton.svelte";
  import { darkMode, setDarkMode } from "@stores/Preferences";
  import { goto } from "$app/navigation";
  import { showModal, setModalView, ModalViews } from "@stores/app";

  const goToDashboard = () => {
    goto("/");
  };

  const openAWSModal = () => {
    console.log("openAWSModal");
    showModal();
    setModalView(ModalViews.AWS);
  };

  const openUserPreferences = () => {
    console.log("openUserPreferences");
    showModal();
    setModalView(ModalViews.USER);
  };
  const toggleDarkMode = () => {
    const current = $darkMode;
    setDarkMode(current);
  };
</script>

<div class="relative column-1 h-screen bg-custom-blue justify-center side-menu">
  <SidebarButton name="Home" iconName="Home" onClick={goToDashboard} />
  <SidebarButton name="AWS" iconName="Settings" onClick={openAWSModal} />
  <SidebarButton
    name="Preferences"
    iconName="Sliders"
    onClick={openUserPreferences}
  />

  <div class="flex flex-col items-center toggle-dark-mode">
    <button class="btn btn-circle" onClick={toggleDarkMode}>
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

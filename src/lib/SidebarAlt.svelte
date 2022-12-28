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

<nav aria-label="alternative nav">
  <div
    class="bg-gray-800 h-20 fixed bottom-0 mt-12 md:relative md:h-screen z-10 w-full md:w-48 content-center navbar-container"
  >
    <div
      class="md:mt-12 md:w-48 md:fixed md:left-0 md:top-0 content-center md:content-start text-left justify-between"
    >
      <ul
        class="list-reset flex flex-row md:flex-col pt-3 md:py-3 px-1 md:px-2 text-center md:text-left"
      >
        <li class="mr-3 flex-1 sidebar-item">
          <SidebarButton name="Home" iconName="Home" onClick={goToDashboard} />
        </li>
        <li class="mr-3 flex-1 sidebar-item">
          <SidebarButton
            name="AWS"
            iconName="Settings"
            onClick={openAWSModal}
          />
        </li>
        <li class="mr-3 flex-1 sidebar-item">
          <SidebarButton
            name="Preferences"
            iconName="Sliders"
            onClick={openUserPreferences}
          />
        </li>
      </ul>
    </div>
    <div class="flex flex-col items-center toggle-dark-mode">
      <button class="btn btn-circle" on:click={toggleDarkMode}>
        <Moon color="white" />
      </button>
    </div>
  </div>
</nav>

<style>
  .sidebar-item:hover {
    background-color: rgb(75 85 99);
  }
  .navbar-container {
    max-height: calc(100vh - 96px);
  }
  .toggle-dark-mode {
    position: absolute;
    bottom: 0;
    width: 100%;
    padding-bottom: 20px;
  }
</style>

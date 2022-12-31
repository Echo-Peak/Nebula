<script>
  import { Moon } from "lucide-svelte";
  import SidebarButton from "@components/SidebarButton.svelte";
  import { getDarkMode, setDarkMode } from "@stores/Preferences";
  import { goto } from "$app/navigation";
  import { showModal, setModalView, ModalViews } from "@stores/app";

  let isDarkMode;

  getDarkMode()
    .then(() => {
      isDarkMode = "it worked";
    })
    .catch((err) => {
      console.log(err);
      isDarkMode = "it no work";
    });
  const goToSources = () => {
    goto("/sources");
  };
  const gotoExplorer = () => goto("/explorer");
  const gotoViewer = () => goto("/viewer");

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
    // const current = $darkMode;
    // setDarkMode(current);
  };
  const openAboutModal = () => {
    showModal();
    setModalView(ModalViews.ABOUT);
  };
</script>

<nav aria-label="alternative nav">
  <div
    class="bg-gray-800 h-20 fixed bottom-0 mt-12 md:relative md:h-screen z-10 w-full md:w-48 content-center navbar-container"
  >
    <div
      class="md:mt-12 md:w-48 md:fixed md:left-0 md:top-0 content-center md:content-start text-left justify-between pt-4"
    >
      <ul
        class="list-reset flex flex-row md:flex-col md:py-3 px-1 md:px-2 text-center md:text-left"
      >
        <li class="mr-3 flex-1 sidebar-item">
          <SidebarButton
            name={`Darkmode >> ${isDarkMode}`}
            iconName="Database"
            onClick={goToSources}
          />
        </li>
        <li class="mr-3 flex-1 sidebar-item">
          <SidebarButton
            name="Explorer"
            iconName="Folder"
            onClick={gotoExplorer}
          />
        </li>
        <li class="mr-3 flex-1 sidebar-item">
          <SidebarButton name="Viewer" iconName="Image" onClick={gotoViewer} />
        </li>
        <div class="sidebar-divider" />
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
        <li class="mr-3 flex-1 sidebar-item">
          <SidebarButton
            name="About"
            iconName="HelpCircle"
            onClick={openAboutModal}
          />
        </li>
      </ul>
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
  .sidebar-divider {
    height: 1px;
    width: 60%;
    background: rgb(75 85 99);
    margin: 0 auto;
    margin-top: 4px;
    margin-bottom: 4px;
  }
</style>

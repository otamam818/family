<script>
  import "./style.scss";
  import { get_user_data } from "../dataHandler/requestHandler.js"
  import NavBar from "./NavBar.svelte";
  import SideBar from "./SideBar.svelte";
  import FileShowcase from "./FileShowcase.svelte";
  import { userData } from "./userData.js";

  let duration = 0;
  setTimeout( async () => {
    let session = await get_user_data();
    console.log(session);
    if (!session.exists) {
      window.location.href = "/";
      return;
    }

    console.info("Updating time and user data")
    // fifteen minutes converted to miliseconds
    duration = 900000;
    userData.update(() => session.user_data);
  }, duration);
</script>

<div class="dashboard-container">
  <NavBar />
  <div class="stored-data">
    <SideBar />
    <FileShowcase />
  </div>
</div>

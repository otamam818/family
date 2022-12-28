<script>
  import "./style.scss";
  import { get_user_data } from "../dataHandler/requestHandler.js"
  import NavBar from "./NavBar.svelte";
  import { userData } from "./userData.js";

  let duration = 0;
  setTimeout( async () => {
    let session = await get_user_data();
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
</div>

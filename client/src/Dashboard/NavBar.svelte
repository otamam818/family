<script>
  import "./style-NavBar.scss";
  import {ACCESS_KEY} from "../dataHandler/cookieHandler";
  import SettingsIcon from "../assets/SettingsIcon.svelte";
  import UserIcon from "../assets/UserIcon.svelte";
  import PlusIcon from "../assets/PlusIcon.svelte";
  import LogoutIcon from "../assets/LogOutIcon.svelte";
  import {userData} from "./userData";

  let fullName;
  let username;
  userData.subscribe(session => {
    let lastName = session.last_name ? session.last_name : "";
    fullName = `${session.first_name} ${lastName}`;
    username = session.username;
  });

</script>

<div class="navigation-bar">
  <div class="left-side">
    <UserIcon />
    <div class="names">
      <span id="first"> {fullName} </span>
      <span id="last"> {username} </span>
    </div>
  </div>
  <div class="right-side">
    <button class="add-button">
      <PlusIcon />
      <span> Add </span>
    </button>
    <button>
      <SettingsIcon />
    </button>
    <button
      on:click={() => {
        document.cookie = `${ACCESS_KEY}=; SameSite=Strict`;
        window.location.href = "/";
        return;
      }}
    >
      Logout
      <LogoutIcon />
    </button>
  </div>
</div>


<script>
  import "./form.scss";
  import { onMount } from 'svelte';
  import {formClass} from "./formHandler";
  import Form from "./Form.svelte";
	let usernameInput
  onMount(() => {
    setTimeout(() => {
      usernameInput.focus()
    }, 1200);
  })

  export let updateComponenent;

  onMount(() => {
    setTimeout(() => {
      usernameInput.focus()
    }, 1200);
  })
</script>

<Form>
  <h2> Login </h2>
  <label for="username"> Username </label>
  <input id="username" type="text" bind:this={usernameInput}>
  <label for="password"> Password </label>
  <input type="password">
  <span
    on:keydown={(e) => console.log(e)}
    on:click={() => {
      formClass.update(() => "bounce");
      setTimeout(() => {
        updateComponenent("NewUser");
        formClass.update(() => "no-animation");
      }, 1200);
  }}> Make a new user </span>

  <button> Submit </button>
</Form>

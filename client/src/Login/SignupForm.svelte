<script>
  import Form from "./Form.svelte";
  import {formClass} from "./formHandler";
  import {handleSubmit} from "./SignupSubmit";
  export let updateComponenent;

</script>

<Form>
  <div>
    <h2> Sign Up </h2>
    <div>
      <label for="username"> Username </label>
      <input id="username" type="text">
    </div>
    <div>
      <label for="password"> Password </label>
      <input id="password" type="password">
    </div>
    <div>
      <!-- The `margin-right: 4ch` makes `Code` be 8ch long -->
      <label for="Code" style="margin-right: 4ch;"> Code </label>
      <input id="secret-code" type="text">
    </div>
  </div>

  <div>
    <h3> Other Info </h3>
    <div>
      <label for="First Name"> First Name </label>
      <input id="first-name" type="text">
    </div>
    <div>
      <label for="Last Name"> Last Name </label>
      <input id="last-name" type="text">
    </div>
    <div>
      <label for="Birth Place"> Birth Place </label>
      <input id="birth-place" type="text">
    </div>
    <div>
      <label for="Birthday"> Birthday </label>
      <input id="birthday" type="date">
    </div>
    <div>
      <label for="Gender"> Gender </label>
      <select>
        <option>Male</option>
        <option>Female</option>
        <option>Other</option>
      </select>
    </div>
  </div>
  <span class="back-message"
    on:keydown={(e) => console.log(e)}
    on:click={() => {
      formClass.update(() => "spread fade-out");
      setTimeout(() => {
        updateComponenent("Form");
        formClass.update(() => "squeeze-in");
      }, 400);
    }}
  > Back </span>
  <button on:click|preventDefault={handleSubmit}> Submit </button>
</Form>

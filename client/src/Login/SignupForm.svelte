<script>
  import Form from "./Form.svelte";
</script>

<Form>
  <h2> Sign Up </h2>
  <label for="username"> Username </label>
  <input id="username" type="text">
  <label for="password"> Password </label>
  <input type="password">

  <h3> Other Info </h3>
  <label for="username"> First Name </label>
  <input id="username" type="text">
  <label for="username"> Last Name </label>
  <input id="username" type="text">
  <label for="username"> Birthday </label>
  <input id="username" type="date">
  <label for="username"> Gender </label>
  <select>
    <option>Male</option>
    <option>Female</option>
    <option>Other</option>
  </select>

  <button> Submit </button>
</Form>

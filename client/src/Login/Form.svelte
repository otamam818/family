<script>
  import "./form.scss";
  import {formClass} from "./formHandler";
  let currClass = '';

  formClass.subscribe(value => {
    currClass = value;
  });

</script>

<form class={currClass} on:submit={(e) => e.preventDefault()}>
  <slot></slot>
</form>

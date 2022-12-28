import { writable } from 'svelte/store';

export const userData = writable({
  first_name: " - ",
  last_name: "",
  username: " - "
});

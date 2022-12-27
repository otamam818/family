import {postPromise} from "../dataHandler/requestHandler";

export async function handleSubmit() {
  let [username, password] = [ 'username', 'password' ].map((value) => {
    return document.querySelector(`input[id='${value}']`).value;
  });

  return postPromise("login", { username, password })
    .then((value) => {
      return value.text();
    })
    .then((value) => {
      return value;
    })
    .catch((reason) => {
      console.error(reason);
    });
}


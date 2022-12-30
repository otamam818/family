import {postPromise} from "../dataHandler/requestHandler";

export function handleSubmit() {
  const fields = [
    'username',
    'password',
    'first-name',
    'last-name',
    'birthday',
    'birth-place',
    'secret-code'
  ];

  let elements = fields.map((value) => {
    
    let inpAccept = document.querySelector(`input[id='${value}']`);
    return inpAccept.value;
  });

  let birthday = getDate(elements[4].split('-'));
  if (!birthday.every(val => typeof(val) === 'number')) {
    birthday = [0, 0, 0];
  }

  let gender = document.querySelector("select").value.toUpperCase();
  let first_name = elements[2];
  let last_name = elements[3];

  return postPromise("user", {
    username: elements[0],
    password: elements[1],
    secret_code: elements[6],
    first_name,
    last_name,
    birthday,
    gender,
    place_of_birth: elements[5],
  })
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

/**
 * Constructs a promise from a POST request
 * @param originalDate {Array<string>}
 */
function getDate(originalDate) {
  let temp = originalDate[2];
  originalDate[2] = originalDate[0];
  originalDate[0] = temp;
  return originalDate.map(value => parseInt(value));
}


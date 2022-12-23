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
    return document.querySelector(`input[id='${value}']`).value;
  });

  let birthday = getDate(elements[4].split('-'));
  if (!birthday.every(val => typeof(val) === 'string')) {
    birthday = [0, 0, 0];
  }
  let gender = document.querySelector("select").value.toUpperCase();
  let first_name = elements[2];
  let last_name = elements[3];
  console.log(elements, birthday, gender);

  fetch(new Request("http://localhost:8000/user"), {
    method: 'POST',
    headers: {
      'Accept': 'application/json',
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({
      username: elements[0],
      password: elements[1],
      secret_code: elements[6],
      first_name,
      last_name,
      birthday,
      gender,
      place_of_birth: elements[5],
    }),
    mode: 'cors'
  })
    .then((value) => {
      if (value.ok) {
        return value.json();
      }
      return value.text();
    })
    .then((value) => {
      console.log(value);
    })
    .catch((reason) => {
      console.error(reason);
    });
}

function getDate(originalDate) {
  let temp = originalDate[2];
  originalDate[2] = originalDate[0];
  originalDate[0] = temp;
  return originalDate.map(value => parseInt(value));
}


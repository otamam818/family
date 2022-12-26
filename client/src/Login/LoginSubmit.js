export function handleSubmit() {
  const fields = [
    'username',
    'password',
  ];

  let [username, password] = fields.map((value) => {
    return document.querySelector(`input[id='${value}']`).value;
  });

  return fetch(new Request("http://localhost:8000/login"), {
    method: 'POST',
    headers: {
      'Accept': 'application/json',
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({
      username,
      password,
    }),
    mode: 'cors'
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


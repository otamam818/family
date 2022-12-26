export function handleSubmit() {
  const fields = [
    'username',
    'password',
  ];

  let [username, password] = fields.map((value) => {
    return document.querySelector(`input[id='${value}']`).value;
  });

  console.log(username, password);

  fetch(new Request("http://localhost:8000/login"), {
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
      console.log(value);
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


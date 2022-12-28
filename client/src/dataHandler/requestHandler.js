import {ACCESS_KEY, getCookie} from "../dataHandler/cookieHandler.js";

const currURI = "http://localhost:8000";

/**
 * Constructs a promise from a POST request
 * @param path {string}
 * @param jsonObject {object}
 */
export function postPromise(path, jsonObject) {
  return fetch(new Request(`${currURI}/${path}`), {
    method: 'POST',
    headers: {
      'Accept': 'application/json',
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(jsonObject),
    mode: 'cors'
  })
}

/**
 * Checks if the current session handled by the token is valid
 */
export async function valid_session() {
  // TODO: Check if a login cookie exists, and check-via the server
  //       whether it is still valid
  let accessCookie = getCookie(ACCESS_KEY);
  if (accessCookie) {
    if (accessCookie.length !== 0) {
      let value = await fetch(new Request(`${currURI}/session-check`), {
        method: 'POST',
        headers: {
          'Accept': 'text/plain; charset=utf-8',
          'Content-Type': 'text/plain; charset=utf-8'
        },
        body: accessCookie,
        mode: 'cors'
      });
      return await value.json()
    }
  }
}

/**
 * Checks if the current session handled by the token is valid
 */
export async function get_user_data() {
  // TODO: Check if a login cookie exists, and check-via the server
  //       whether it is still valid
  let accessCookie = getCookie(ACCESS_KEY);
  let value = await fetch(new Request(`${currURI}/get-user-data`), {
    method: 'POST',
    headers: {
      'Accept': 'text/plain; charset=utf-8',
      'Content-Type': 'text/plain; charset=utf-8'
    },
    body: accessCookie,
    mode: 'cors'
  });
  return await value.json()
}

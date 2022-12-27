export const ACCESS_KEY = "ahmed-fam-access";

/**
 * Gets the specific cookie value based on the key it is assigned to
 * @param key {string}
 */
export function getCookie(key) {
  return document.cookie
    .split('; ')
    .find((row) => row.startsWith(`${key}=`))
    ?.split('=')[1];
}


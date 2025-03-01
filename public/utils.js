export function setItem(key, value) {
  localStorage.setItem(key, value);
}
export function getItem(key) {
  return localStorage.getItem(key);
}
export function removeItem(key) {
  localStorage.removeItem(key);
}
/* Snow */
export function setSnowStatus(value) {
  if (!document.readyState == "complete") {
    return;
  }
  if (!typeof value == "boolean") {
    return new Error("value must be of type 'boolean'");
  }
  window.snowEnabled = value;
  return window.snowEnabled;
}
export function getSnowStatus() {
  console.log(window.snowEnabled);
  return window.snowEnabled;
}

import { browser } from "$app/environment";

const defaultLocale = "zh";
const fallbackLocale = "en";

function getLocale() {
  return browser ? window.navigator.language : defaultLocale;
}

function setLocale() {}

export { getLocale, setLocale, defaultLocale, fallbackLocale };

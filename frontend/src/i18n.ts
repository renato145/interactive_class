import { register, init, getLocaleFromNavigator } from "svelte-i18n";

register("en", () => import("./lang/en.json"));
register("es", () => import("./lang/es.json"));

const currentLocale = getLocaleFromNavigator().slice(0, 2);

init({
  fallbackLocale: "en",
  initialLocale: currentLocale,
});

import "primeicons/primeicons.css";

import { createApp } from "vue";
import App from "./App.vue";
import router from "./router/index";
import Nora from "@primevue/themes/nora";
import PrimeVue from "primevue/config";
import { definePreset, palette } from "@primevue/themes";
import { createPinia } from "pinia";
import Ripple from "primevue/ripple";

const pinia = createPinia();
const app = createApp(App);

app.use(PrimeVue, {
  theme: {
    preset: definePreset(Nora, { semantic: { primary: palette("#3EB489") } }),
  },
  ripple: true,
});
app.use(pinia);
app.use(router);

app.directive("ripple", Ripple);

app.mount("#app");

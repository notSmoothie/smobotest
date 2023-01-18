import { createApp } from "vue";
import { registerPlugins } from "./plugins";
import App from "./App.vue";
import router from "./router";

import vuetify from "./plugins/vuetify";

const app = createApp(App);

registerPlugins(app);

app.use(vuetify);
app.use(router);

app.mount("#app");

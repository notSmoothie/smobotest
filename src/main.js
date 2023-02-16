import { createApp } from "vue";
import { registerPlugins } from "./plugins";
import App from "./App.vue";
import router from "./router";

import vuetify from "./plugins/vuetify";

const app = createApp(App);



app.use(vuetify);
app.use(router);

disableMenu();
app.mount("#app");

function disableMenu() {
    if (window.location.hostname === 'localhost') {
    return
    }
    
    document.addEventListener('contextmenu', e => {
    e.preventDefault();
    return false;
    }, { capture: true })
    
    document.addEventListener('selectstart', e => {
    e.preventDefault();
    return false;
    }, { capture: true })
    }

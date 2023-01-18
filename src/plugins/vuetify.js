/**
 * plugins/vuetify.js
 *
 * Framework documentation: https://vuetifyjs.com`
 */

// Styles
import "@mdi/font/css/materialdesignicons.css";
import "vuetify/styles";

// Vuetify
import { createVuetify } from "vuetify";
import * as components from "vuetify/components";
import * as directives from "vuetify/directives";

// Components

export default createVuetify({
    components,
    directives,
    defaults: {
        global: {
            rounded: "sm",
        },
        VAppBar: {
            flat: true,
        },
        VBtn: {
            color: "primary",
            height: 44,
            minWidth: 190,
        },
        VBtnAlt: {
            color: "primary",
            height: 48,
            minWidth: 190,
            variant: "outlined",
        },
        VSheet: {
            color: "#212121",
        },
    },
    theme: {
        defaultTheme: "dark",
        themes: {
            dark: {
                dark: true,
                colors: {
                    primary: "#1697f6",
                },
            },
        },
    },
});

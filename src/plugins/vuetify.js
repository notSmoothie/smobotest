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
    },
    theme: {
        variations:{
            colors:['primary'],
            lighten:5,
            darken:5
        },
        defaultTheme: "dark",
        themes: {
            dark: {
                dark: true,
                colors: {
                    primary: "#424242",
                    secondary: "#424242",
                    '1': '#C69B6D',
                    '2': '#F48CBA',
                    '3': '#AAD372',
                    '4': '#FFF468',
                    '5': '#FFFFFF',
                    '6': '#C41E3A',
                    '7': '#0070DD',
                    '8': '#3FC7EB',
                    '9': '#9482C9',
                    '11': '#FF7C0A',
                    "horde-red": '#8C1616',
                    "horde": '#000000',
                    "alliance": '#00216a',
                    "alliance-gold": '#D4AF37' 
                },
            },
        },
    },
});

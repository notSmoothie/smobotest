<template>
  <v-app>
    <v-system-bar data-tauri-drag-region window>
      <div class="d-flex align-center">
        <v-img
          data-tauri-drag-region
          draggable="false"
          style="user-select: none; pointer-events: none"
          src="/logo.svg"
          height="25px"
          width="25px"
          class="ma-2"
        />
        <span data-tauri-drag-region class="ma-1" style="user-select: none">SMOBOT</span>
      </div>
      <v-spacer></v-spacer>
      <div>
        <v-btn
          @click="minimize"
          icon="mdi-minus"
          variant="text"
          size="25px"
          class="mr-1 my-2 text-white"
        ></v-btn>
        <v-btn
          @click="kill"
          icon="mdi-close"
          color="red"
          variant="text"
          size="25px"
          class="mr-1 my-2 text-black"
        ></v-btn>
      </div>
    </v-system-bar>

    <v-main> </v-main>
    <v-dialog max-width="500px" v-model="updateDialog">
      <v-card>
        <v-card-title variant="tonal" style="background-color: #dd4814"
          >SMOBOT</v-card-title
        >
        <v-card-text>
          <div>Update</div>
          <div>Version: {{ updateInfo.manifest.version }}</div>
          <div>
            Date:
            {{
              new Date(`${updateInfo.manifest.date.split(" +")[0]}Z`).toLocaleDateString()
            }}
          </div>
          <br />
          <div>Do you want to install it now ?</div>
        </v-card-text>
        <v-card-actions>
          <v-btn color="primary" block @click="dialog = false">Close Dialog</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
    <div class="d-flex align-center justify-space-between text-body-2 footer">
      Copyright &copy 2023 by notSmoothie
      <div>v.{{ v }}</div>
    </div>
  </v-app>
</template>

<script>
import { window as _window } from "@tauri-apps/api";
import { invoke } from "@tauri-apps/api/tauri";
import { appWindow } from "@tauri-apps/api/window";
import { checkUpdate, installUpdate } from "@tauri-apps/api/updater";
import _package from "../package.json";

export default {
  name: "App",
  data() {
    return {
      v: _package.version,
      updateDialog: false,
      updateInfo: {},
    };
  },
  async created() {
    console.log(this.v);
    this.updateInfo = await checkUpdate();
    if (this.updateInfo.shouldUpdate) {
      this.updateDialog = true;
      console.log(
        `Installing update ${updateInfo.manifest?.version}, ${updateInfo.manifest?.date}, ${updateInfo.manifest.body}`
      );
      //await installUpdate();
    }
  },
  computed: {},
  methods: {
    kill() {
      invoke("hide_window");
      appWindow.hide();
    },
    async minimize() {
      appWindow.minimize();
    },
  },
};
</script>

<style>
path {
  fill: #3b444b;
}
.title {
  position: fixed;
  width: 100%;
  background-color: #3b444b;
  z-index: 42069;
}

.footer {
  position: fixed;
  bottom: 0;
  width: 100%;
  background-color: #1e1e1e;
  z-index: 42069;
}

*::-webkit-scrollbar {
  display: none;
}
</style>

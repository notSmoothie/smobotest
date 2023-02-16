<template>
  <v-app class="bruh">
    <v-system-bar data-tauri-drag-region window>
      <div class="d-flex align-center">
        <v-img
          data-tauri-drag-region
          draggable="false"
          style="user-select: none; pointer-events: none"
          src="/logo.svg"
          height="25px"
          width="25px"
        />
        <span data-tauri-drag-region class="ma-3" style="user-select: none">SMOBOT</span>
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

    <v-main>
      <div class="grid-wrapper">
        <CharacterView
          @rotated="rotateEvent"
          :rotated="character.name == rotatedName"
          v-for="character of characters.slice(0, 11)"
          :character="character"
        ></CharacterView>
        <CharacterView
          :key="characters.length"
          classS="kokot"
          @rotated="rotateEvent"
          @filled="lastFilled"
          :empty="true"
          :isFilled="lastIsFilled"
          :rotated="
            rotatedName == 'SB-addChar' || (lastFill && rotatedName == lastChar?.name)
          "
          :character="lastChar"
          :chars="characters"
        ></CharacterView>
      </div>
    </v-main>
    <v-dialog max-width="400px" v-model="updateDialog" :persistent="true">
      <v-card>
        <v-card-title
          variant="tonal"
          style="background-color: rgb(var(--v-theme-primary-darken-1))"
          >New Update Available</v-card-title
        >
        <v-card-text>
          <h4 class="text-overline">{{ updateInfo.manifest.body }}</h4>
          <br />
          <div class="text-overline">
            Version:
            <span class="text-primary font-weight-regular">{{
              updateInfo.manifest.version
            }}</span>
          </div>
          <div class="text-overline">
            Date:
            <span class="text-primary font-weight-regular">{{
              new Date(`${updateInfo.manifest.date.split(" +")[0]}Z`).toLocaleDateString()
            }}</span>
          </div>
          <br />
          <div class="text-overline">Do you want to install it now ?</div>
        </v-card-text>
        <v-card-actions>
          <v-btn variant="elevated" @click="updateDialog = false">Cancel</v-btn>
          <v-spacer></v-spacer>
          <v-btn variant="elevated" color="primary" @click="update">Install</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
    <v-overlay :model-value="loadingUpdate" class="align-center justify-center">
      <v-progress-circular color="primary" indeterminate size="64"></v-progress-circular>
    </v-overlay>

    <v-snackbar
      location="bottom"
      :offset="[20, 20, 20, 20]"
      v-model="snackbar"
      :color="snaccBarColor"
      :timeout="2000"
    >
      {{ snaccBarMsg }}
    </v-snackbar>
  </v-app>
</template>

<script>
import { window as _window } from "@tauri-apps/api";
import { invoke } from "@tauri-apps/api/tauri";
import { appWindow, PhysicalSize } from "@tauri-apps/api/window";
import { checkUpdate, installUpdate } from "@tauri-apps/api/updater";
import { listen } from "@tauri-apps/api/event";
import { getClient, ResponseType } from "@tauri-apps/api/http";
import CharacterView from "./components/CharacterView.vue";

import _package from "../package.json";
const client = await getClient();

export default {
  name: "App",
  components: {
    CharacterView,
  },
  data() {
    return {
      rotatedName: "",
      snackbar: false,
      loadingChar: false,
      characters: [],
      charToAdd: "",
      lastIsFilled: false,
      v: _package.version,
      updateDialog: false,
      updateInfo: {},
      loadingUpdate: false,
      lastChar: {},
      snaccBarColor: "info",
    };
  },
  async created() {
    await appWindow.setMinSize(new PhysicalSize(800, 600));
    this.characters = JSON.parse(localStorage.getItem("characters")) ?? [];
    const unlisten = await listen("checkUpdate", async (event) => {
      console.log("thishappened");
      this.loadingUpdate = true;
      this.updateInfo = await checkUpdate();
      this.loadingUpdate = false;
      if (this.updateInfo.shouldUpdate) {
        this.updateDialog = true;
      } else {
        this.snaccBarColor = "info";
        this.snaccBarMsg = "Your version is up to date.";
        this.snackbar = true;
        setTimeout(() => (this.snackbar = false), 2000);
      }
    });

    this.loadingUpdate = true;
    this.updateInfo = await checkUpdate();
    this.loadingUpdate = false;
    if (this.updateInfo.shouldUpdate) {
      this.updateDialog = true;
    }
  },
  computed: {
    lastFill() {
      return this.lastIsFilled;
    },
  },
  methods: {
    lastFilled(e) {
      this.lastIsFilled = true;
      this.lastChar = e;
      setTimeout(() => {
        this.characters.push(this.lastChar);
        localStorage.setItem("characters", JSON.stringify(this.characters));
        this.lastIsFilled = false;
        this.lastChar = "";
      }, 500);
    },

    rotateEvent(e) {
      if (this.rotatedName == e) {
        this.rotatedName = "";
      } else {
        this.rotatedName = e;
      }
    },
    bruh(str) {
      btoa(
        new Uint8Array(str).reduce(function (data, byte) {
          return data + String.fromCharCode(byte);
        }, "")
      );
    },
    async update() {
      await installUpdate();
    },
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
.bruh {
  background: transparent !important;
}

.footMan {
  position: sticky !important;
  bottom: 0;
}

path {
  fill: #3b444b;
}

.title {
  position: fixed;
  width: 100%;
  background-color: #3b444b;
  z-index: 42069;
}

*::-webkit-scrollbar {
  display: none;
}
.grid-wrapper {
  display: grid;
  grid-column-gap: 1rem;
  grid-row-gap: 1rem;
  margin: 1rem 2rem;
  width: calc(100% - 4rem);
  grid-template-columns: repeat(2, 1fr);
  grid-template-rows: repeat(6, 1fr);
  transition: all 0.5s;
}

/*  */
@media screen and (min-width: 720px) {
  .grid-wrapper {
    margin: 1rem 2rem;
    width: calc(100% - 4rem);
    grid-template-columns: repeat(3, 1fr);
    grid-template-rows: repeat(4, 1fr);
    transition: all 0.5s;
  }
}
@media screen and (min-width: 1200px) {
  .grid-wrapper {
    grid-template-columns: repeat(4, 1fr);
    grid-template-rows: repeat(3, 1fr);
    width: calc(100% - 10rem);
    margin: 3rem 5rem;
    transition: all 0.5s;
  }
}

.kokot {
  animation: splash 0.5s normal forwards ease-out;
  transform-origin: 0% 50%;
}

@keyframes splash {
  from {
    transform: translateX(-100%);
  }
  to {
    transform: translateX(0);
  }
}
/* .rotation-wrap {
  width: 100%;
  height: 100%;
} */
</style>

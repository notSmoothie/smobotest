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
      <v-text-field
        v-model="charToAdd"
        @keydown.enter="gimmeChars()"
        label="addchar"
      ></v-text-field>
      <div class="grid-wrapper">
        <CharacterView
          @rotated="rotateEvent"
          :rotated="character.name == rotatedName"
          class="flex-grow-1 flex-shrink-1"
          v-for="character of characters.slice(0, 12)"
          :character="character"
        ></CharacterView>
        <!-- <v-card
            class="charSheet"
            style="flex-grow: 5; user-select: none; pointer-events: none"
          >
            <v-card-text> </v-card-text
          ></v-card> -->
      </div>
      <!-- <div class="rotation-wrap">
        <v-card class="front"
          ><v-card-title>front</v-card-title
          ><v-card-text
            >Lorem ipsum dolor sit amet consectetur adipisicing elit. Quisquam praesentium
            debitis, velit laboriosam commodi fugit, reprehenderit eaque nemo deleniti
            magni deserunt adipisci ullam minima nesciunt accusantium corporis soluta? Et,
            maxime.</v-card-text
          ></v-card
        >
        <v-card class="back"
          ><v-card-title>back</v-card-title
          ><v-card-text
            >Lorem ipsum dolor sit, amet consectetur adipisicing elit. Recusandae illum
            dicta quis! Quia voluptatum provident, suscipit similique nulla obcaecati non
            expedita porro voluptatibus eligendi iste vel est ullam officia
            doloremque!</v-card-text
          ></v-card
        >
      </div> -->
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
      v: _package.version,
      updateDialog: false,
      updateInfo: {},
      loadingUpdate: false,
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
  computed: {},
  methods: {
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
    async gimmeChars() {
      let toonUrl = `https://twinstar-api.twinstar-wow.com/character/?name=${this.charToAdd}&realm=Apollo`;
      if (
        this.characters.find(
          (el) => el.name.toLowerCase() == this.charToAdd.toLowerCase()
        ) !== undefined
      ) {
        this.snaccBarColor = "red";
        this.snaccBarMsg = "Nono!";
        this.snackbar = true;
        setTimeout(() => (this.snackbar = false), 2000);
        this.charToAdd = "";
        return;
      }

      this.charToAdd = "";
      await client
        .get(toonUrl, {
          // the expected response type
          responseType: ResponseType.JSON,
        })
        .catch((error) => {
          console.log(error);
        })
        .then((el) => {
          try {
            if (el.ok == false) return;
            this.loadingChar = true;
            this.characters.push(el.data);
            localStorage.setItem("characters", JSON.stringify(this.characters));
            this.loadingChar = false;
            console.log(this.characters);
          } catch (e) {
            console.log(e);
          }
        });
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
}

/*  */
@media screen and (min-width: 720px) {
  .grid-wrapper {
    margin: 1rem 2rem;
    width: calc(100% - 4rem);
    grid-template-columns: repeat(3, 1fr);
    grid-template-rows: repeat(4, 1fr);
  }
}
@media screen and (min-width: 1200px) {
  .grid-wrapper {
    grid-template-columns: repeat(4, 1fr);
    grid-template-rows: repeat(3, 1fr);
    width: calc(100% - 10rem);
    margin: 3rem 5rem;
  }
}
/* .rotation-wrap {
  width: 100%;
  height: 100%;
} */
</style>

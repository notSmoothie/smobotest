<template>
  <div
    class="wrapper"
    v-if="empty"
    :class="[{ rotated: rotated }, classS]"
    @click="onCardClick"
  >
    <v-card
      v-if="!isFilled"
      style="height: 100%; background-color: rgba(20, 20, 20, 0.5)"
      class="front"
    >
      <v-card-text style="height: 100%" class="mx-5 d-flex justify-center align-center">
        <v-icon class="plusIcon" size="40">mdi-plus</v-icon>
        <v-img
          style="pointer-events: none; user-select: none"
          class="factionLogo right"
          :src="`factions/0.svg`"
          min-width="200px"
        ></v-img>
        <v-img
          style="pointer-events: none; user-select: none"
          class="factionLogo"
          :src="`factions/1.svg`"
          min-width="200px"
        ></v-img>
      </v-card-text>
    </v-card>

    <v-card
      v-if="isFilled"
      style="pointer-events: none; height: 20vh; background-color: rgba(20, 20, 20, 0.5)"
      class="front"
    >
      <v-card-title
        :class="`text-${character.class} primary`"
        class="topBar d-flex justify-space-between"
      >
        <v-spacer></v-spacer>
        <span>{{ character.name }}</span>
        <v-spacer></v-spacer>
        <v-img :src="`classes/${character.class}.png`" max-width="2rem"></v-img>
      </v-card-title>
      <v-card-text class="actionSpace mx-5" style="height: 100%">
        <v-img
          class="factionLogo"
          :src="`factions/${character.faction}.svg`"
          min-width="200px"
        ></v-img>
        <br />
        <v-icon icon="mdi-arrow-up-bold"></v-icon>
        {{ character.level }}<br />
        <span v-if="character.level >= 85">
          <v-icon icon="mdi-crown"></v-icon>
          {{ Math.round(character.averageItemLevel) }}
        </span>
      </v-card-text>
      <v-icon
        class="actionButton"
        style="right: 2px; bottom: 2px; position: absolute"
        size="15px"
        icon="mdi-subdirectory-arrow-right"
      ></v-icon>
    </v-card>

    <v-card @click.stop class="back" style="background-color: rgba(20, 20, 20, 0.5)">
      <v-card-title
        v-ripple.stop
        class="topBar d-flex justify-center"
        style="background-color: rgb(var(--v-theme-primary))"
      >
        <span style="pointer-events: none, user-select: none, color=rgba(60,60,60,0.6)"
          >New Character</span
        >
      </v-card-title>
      <v-card-text v-ripple.stop class="mx-5 mt-5 d-flex align-center justify-center">
        <v-icon size="25" color="#4e4e4e" icon="mdi-magnify"></v-icon>
        <input
          style="width: 70%; outline: 0px"
          class="px-3"
          ref="inputField"
          @keydown.enter="gimmeChar"
          type="text"
          v-model="charToAdd"
        />
      </v-card-text>
    </v-card>
  </div>

  <div class="wrapper" v-if="!empty || isFilled" :class="{ rotated: rotated }">
    <v-card
      @click="onCardClick"
      v-if="!empty"
      style="background-color: rgba(20, 20, 20, 0.5)"
      class="front"
    >
      <v-card-title
        :class="`text-${character.class} primary`"
        class="topBar d-flex justify-space-between"
      >
        <v-spacer></v-spacer>
        <span>{{ character.name }}</span>
        <v-spacer></v-spacer>
        <v-img :src="`classes/${character.class}.png`" max-width="2rem"></v-img>
      </v-card-title>
      <v-card-text v-if="!empty || isFilled" class="actionSpace mx-5">
        <v-img
          class="factionLogo"
          :src="`factions/${character.faction}.svg`"
          min-width="200px"
        ></v-img>
        <br />
        <v-icon icon="mdi-arrow-up-bold"></v-icon>
        {{ character.level }}<br />
        <span v-if="character.level >= 85">
          <v-icon icon="mdi-crown"></v-icon>
          {{ Math.round(character.averageItemLevel) }}
        </span>
      </v-card-text>
      <v-icon
        v-if="!empty || isFilled"
        class="actionButton"
        style="right: 2px; bottom: 2px; position: absolute"
        size="15px"
        icon="mdi-subdirectory-arrow-right"
      ></v-icon>
    </v-card>

    <v-card
      @click="onCardClick"
      v-if="!empty"
      style="background-color: rgba(20, 20, 20, 0.5)"
      class="back"
    >
      <v-card-title
        v-if="!empty"
        :class="`topBar d-flex justify-space-between text-${character.class}`"
      >
        <span>{{ character.name }}</span>
        <v-spacer></v-spacer>
      </v-card-title>
      <div class="containter">
        <v-card-text>
          <v-img :src="`races/${character.race}.png`" max-width="8rem"></v-img>
          <v-img
            class="factionLogo right"
            :src="`factions/${character.faction}.svg`"
            min-width="200px"
          ></v-img>
          <br />
          Lvl: ipsum<br />
          Ilvl: {{ Math.round(character.averageItemLevel) }}</v-card-text
        >
      </div>
    </v-card>
  </div>
</template>

<script>
import { window as _window } from "@tauri-apps/api";
import { getClient, ResponseType } from "@tauri-apps/api/http";
const client = await getClient();
import AddCharacter from "./AddCharacter.vue";

export default {
  name: "CharacterView",
  components: { AddCharacter },
  emits: ["rotated", "filled"],
  props: {
    chars: {},
    classS: "",
    character: {
      name: String,
      default: "",
      level: String,
      default: "",
    },
    rotated: false,
    empty: false,
    isFilled: false,
  },
  data() {
    return {
      charToAdd: "",
    };
  },
  async created() {},
  computed: {
    cssVar() {
      return `url("/races/${this.character.race}.svg")`;
    },
    getFactionCss() {
      return `url("/factions/${this.character.faction}.svg")`;
    },
    cardBg() {
      return `rgb(var(--v-theme-${
        this.character.faction == "0" ? "horde" : "alliance"
      }))`;
    },
    opacity() {
      if (this.isFilled) return 1;
      if (this.empty && !this.rotated) return 0.3;
      return 1;
    },
    border() {
      if (this.isFilled) return "none";
      if (this.empty && !this.rotated) return "1px dashed #ffffff";
      return "";
    },
    borderHover() {
      if (this.isFilled) return "none";
      if (this.empty && !this.rotated) return "1px dashed #606060";
      return "";
    },
  },
  methods: {
    async gimmeChar() {
      let toonUrl = `https://twinstar-api.twinstar-wow.com/character/?name=${this.charToAdd}&realm=Apollo`;
      if (
        this.chars.find((el) => el.name.toLowerCase() == this.charToAdd.toLowerCase()) !==
        undefined
      ) {
        console.log("kokotina");
        return;
      }

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
            this.$emit("filled", el.data);
            this.charToAdd = "";
            setTimeout(() => {
              this.$emit("rotated");
            }, 20);
          } catch (e) {
            console.log(e);
          }
        });
    },
    addChar() {
      this.$emit("addChar", this.charToAdd);
      this.charToAdd = "";
    },
    onCardClick(e) {
      console.log("rotating");
      if (this.empty) {
        this.$emit("rotated", "SB-addChar");
        this.$refs.inputField.focus();
        return;
      }
      this.$emit("rotated", this.character.name);
    },
  },
};
</script>

<style>
.upDog {
  position: absolute !important;
  top: 0;
  left: 0px;
  top: -35px;
}

.topBar {
  background-color: v-bind(cardBg);
  position: relative;
}

.wrapper {
  border-radius: 5px;
  opacity: v-bind(opacity);
  border: v-bind(border);
  position: relative;
  height: 20vh;
}

.wrapper.rotated .back {
  transform: perspective(600px) rotateY(0deg);
}
.wrapper.rotated .front {
  transform: perspective(600px) rotateY(-180deg);
}

.back,
.front {
  border-radius: 5px !important;
  backface-visibility: hidden;
  position: absolute !important;
  transition: transform 0.5s !important;
  width: 100%;
  height: 100%;
  cursor: default !important;
}

input:focus-within {
  border-bottom: 1px solid rgb(var(--v-theme-primary)) !important;
}

.back {
  transform: perspective(600px) rotateY(180deg);
}
.front {
  transform: perspective(600px) rotateY(0);
}

.charSheet {
  display: flex;
  justify-content: flex-end;
  min-width: 23%;
  min-height: 33%;
  margin: 0.5dvw;
}

.factionLogo {
  position: absolute !important;
  top: 50% !important;
  left: 0px !important;
  transform: translate(-50%, -50%);
  height: 80%;
}
.factionLogo.right {
  left: unset;
  right: 0px !important;
  transform: translate(50%, -50%);
}

.actionSpace::after {
  content: "";
  position: absolute;
  border-radius: 100%;
  width: 40px;
  height: 40px;
  bottom: -15px;
  right: -15px;
  background-color: rgba(var(--v-theme-secondary), 0.6);
  transition: all 0.5s;
}

.actionButton {
  transition: all 0.5s;
  color: rgba(255, 255, 255, 0.5);
}

.wrapper {
  transition: all 0.5s;
}

.wrapper:hover {
  opacity: 1;
  transition: all 0.5s;
  border: v-bind(borderHover);
}

.wrapper .plusIcon {
  transition: all 0.5s;
}

.wrapper:hover .plusIcon {
  transition: all 0.5s;
  color: rgba(200, 200, 200, 0.7);
}

.wrapper:hover .actionButton {
  color: white !important;
  font-size: large !important;
  bottom: 4px !important;
  right: 4px !important;
  transition: all 0.5s;
}

.wrapper:hover .actionSpace::after {
  width: 60px;
  height: 60px;
  bottom: -30px;
  right: -30px;
  transition: all 0.5s;
  background-color: rgba(var(--v-theme-secondary), 1);
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
</style>

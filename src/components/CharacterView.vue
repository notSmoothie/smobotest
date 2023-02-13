<template>
  <div class="wrapper" :class="{ rotated: rotated }" @click="onCardClick">
    <v-card class="front ba-0">
      <v-card-title class="topBar d-flex justify-space-between">
        <v-spacer></v-spacer>
        <span>{{ character.name }}</span>
        <v-spacer></v-spacer>
        <v-img :src="`classes/${character.class}.png`" max-width="2rem"></v-img>
      </v-card-title>
      <v-card-text class="actionSpace">
        <v-img
          class="factionLogo"
          :src="`factions/${character.faction}.svg`"
          min-width="200px"
        ></v-img>
        <br />
        Lvl: {{ character.level }}<br />
        Ilvl: {{ Math.round(character.averageItemLevel) }}
      </v-card-text>
      <v-icon
        class="actionButton"
        style="right: 2px; bottom: 2px; position: absolute"
        size="15px"
        icon="mdi-subdirectory-arrow-right"
      ></v-icon>
    </v-card>
    <v-card
      :style="`border-radius: 10px;`"
      class="back"
      :class="`text-${character.class} primary`"
    >
      <v-card-title class="topBar d-flex justify-space-between">
        <span>{{ character.name }}</span>
        <v-spacer></v-spacer>
      </v-card-title>
      <div class="containter">
        <v-card-text class="">
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
export default {
  name: "CharacterView",
  props: {
    character: {
      name: String,
      default: "",
      level: String,
      default: "",
    },
    rotated: false,
  },
  data() {
    return {};
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
  },
  methods: {
    onCardClick(e) {
      console.log("rotating");
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
  position: relative;
  min-height: 20vh;
}

.wrapper.rotated .back {
  transform: perspective(600px) rotateY(0deg);
}
.wrapper.rotated .front {
  transform: perspective(600px) rotateY(-180deg);
}

.back,
.front {
  backface-visibility: hidden;
  position: absolute !important;
  transition: transform 0.5s !important;
  width: 100%;
  height: 100%;
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
  background-color: rgb(var(--v-theme-secondary));
  transition: all 0.5s;
}

.actionButton {
  transition: all 0.5s;
}

.wrapper:hover .actionButton {
  color: rgb(var(--v-theme-primary)) !important;
  font-size: x-large !important;
  bottom: 5px !important;
  right: 5px !important;
  transition: all 0.5s;
}

.wrapper:hover .actionSpace::after {
  width: 60px;
  height: 60px;
  bottom: -26px;
  right: -26px;
  transition: all 0.5s;
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

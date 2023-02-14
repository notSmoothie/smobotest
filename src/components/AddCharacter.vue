<template></template>

<script>
export default {
  name: "AddCharacter",
  data() {
    return {
      charToAdd: "",
      empty: false,
      expandSearch: false,
    };
  },
  props: {
    addingChar: false,
    chars: {},
    rotated: false,
  },

  async created() {},
  computed: {
    opacity() {
      if (!this.rotated) return 0.3;
      return 1;
    },
  },
  methods: {
    addChar() {
      this.$emit("addChar", this.charToAdd);
      this.charToAdd = "";
    },
    onCardClick(e) {
      this.$emit("rotated", "SB-addChar");
    },
  },
};
</script>

<style scoped>
.topBar {
  background-color: black !important;
  position: relative;
}

input[type="text"]:focus-visible {
  border: none;
  outline: none;
  transition: all 0.5s;
}

input[type="text"] {
  position: relative !important;
  background-color: #1e1e1e !important;
  transition: all 0.5s;
  padding-left: 10px;
}

.wrapper {
  border-radius: 5px;
  border: v-bind(border);
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
  border-radius: 5px !important;
  backface-visibility: hidden;
  position: absolute !important;
  transition: transform 0.5s !important;
  width: 100%;
  height: 100%;
  cursor: default !important;
}

.back {
  transform: perspective(600px) rotateY(180deg);
}
.front {
  transform: perspective(600px) rotateY(0);
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
.wrapper {
  transition: all 0.5s;
}

.wrapper:hover {
  transition: all 0.5s;
  border-color: rgb(var(--v-theme-primary)) !important;
}

.wrapper:hover .factionLogo {
  transition: all 0.5s;
  border-color: rgb(var(--v-theme-primary)) !important;
  opacity: 1;
}

.front .factionLogo {
  opacity: 0.4;
  transition: all 0.5s;
}

.wrapper .plusIcon {
  transition: all 0.5s;
}

.wrapper:hover .plusIcon {
  transition: all 0.5s;
  color: rgb(var(--v-theme-primary)) !important;
}

.wrapper:hover .actionButton {
  color: rgb(var(--v-theme-primary)) !important;
  font-size: x-large !important;
  bottom: 5px !important;
  right: 5px !important;
  transition: all 0.5s;
}

path {
  fill: #3b444b;
}

*::-webkit-scrollbar {
  display: none;
}
</style>

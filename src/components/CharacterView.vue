<template>
    <div
        display="none"
        class="wrapper"
        v-if="empty && !loading"
        :class="[{ rotated: rotated }, classS]"
        @click="onCardClick"
    >
        <v-card
            v-if="!isFilled"
            style="height: 100%; background-color: rgba(20, 20, 20, 0.5)"
            class="front"
        >
            <v-card-text
                style="height: 100%"
                class="mx-5 d-flex justify-center align-center"
            >
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
            @click="onCardClick"
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
                <v-img
                    :src="`classes/${character.class}.png`"
                    max-width="2rem"
                ></v-img>
            </v-card-title>
            <v-card-text class="actionSpace ml-12">
                <v-img
                    style="pointer-events: none"
                    class="factionLogo"
                    :src="`factions/${character.faction}.svg`"
                    min-width="100px"
                    max-width="100px"
                ></v-img>
                <br />
                <v-icon icon="mdi-arrow-up-bold"></v-icon>
                {{ character.level }}<br />
                <span v-if="character.level >= 85">
                    <v-icon icon="mdi-crown"></v-icon>
                    {{ Math.round(character.averageItemLevel) }}
                </span>
                <br />
                <span v-if="character.level >= 85">
                    <v-icon
                        v-if="hasId"
                        color="green"
                        icon="mdi-check-circle"
                    ></v-icon>
                    <v-icon v-else color="red" icon="mdi-close-circle"></v-icon>
                </span>
            </v-card-text>
            <v-icon
                class="actionButton"
                style="right: 2px; bottom: 2px; position: absolute"
                size="15px"
                icon="mdi-subdirectory-arrow-right"
            ></v-icon>
        </v-card>

        <v-card
            @click.stop
            class="back"
            style="background-color: rgba(20, 20, 20, 0.5)"
        >
            <v-card-title
                v-ripple.stop
                class="topBar d-flex justify-center"
                style="background-color: rgb(var(--v-theme-primary))"
            >
                <span
                    style="pointer-events: none, user-select: none, color=rgba(60,60,60,0.6)"
                    >New Character</span
                >
            </v-card-title>
            <v-card-text
                v-ripple.stop
                class="mx-5 mt-5 d-flex align-center justify-center"
            >
                <v-icon size="25" color="#4e4e4e" icon="mdi-magnify"></v-icon>
                <input
                    style="width: 70%; outline: 0px"
                    class="px-3"
                    ref="inputField"
                    @keydown.enter="
                        getCharLevels();
                        gimmeChar();
                    "
                    type="text"
                    v-model="charToAdd"
                />
            </v-card-text>
        </v-card>
    </div>

    <div
        v-show="!hidden"
        class="wrapper"
        style="user-select: none"
        v-if="!empty || isFilled"
        :class="{ rotated: rotated }"
    >
        <v-card
            draggable="true"
            @dragstart="(e) => dragStart(e)"
            v-ripple.stop
            ref="frontDrag"
            @click="onCardClick"
            v-if="!empty"
            style="background-color: rgba(20, 20, 20, 0.5)"
            class="front"
        >
            <v-card-title
                v-if="!loading"
                :class="`text-${character.class} primary`"
                class="topBar d-flex justify-space-between"
            >
                <v-spacer></v-spacer>
                <span>{{ character.name }}</span>
                <v-spacer></v-spacer>
                <v-img
                    :src="`classes/${character.class}.png`"
                    max-width="2rem"
                ></v-img>
            </v-card-title>
            <v-card-text
                v-if="(!empty || isFilled) && !loading"
                class="actionSpace ml-12"
            >
                <v-img
                    style="pointer-events: none"
                    class="factionLogo"
                    :src="`factions/${character.faction}.svg`"
                    min-width="100px"
                    max-width="100px"
                ></v-img>
                <br />
                <v-icon icon="mdi-arrow-up-bold"></v-icon>
                {{ character.level }}<br />
                <span v-if="character.level >= 85">
                    <v-icon icon="mdi-crown"></v-icon>
                    {{ Math.round(character.averageItemLevel) }}
                </span>
                <br />
                <span v-if="character.level >= 85">
                    <v-icon
                        v-if="hasId"
                        color="green"
                        icon="mdi-check-circle"
                    ></v-icon>
                    <v-icon v-else color="red" icon="mdi-close-circle"></v-icon>
                </span>
            </v-card-text>
            <v-card-text
                v-if="loading"
                style="height: 100%"
                class="actionSpace ml-12"
            >
                <div
                    style="height: 100%"
                    class="d-flex justify-center align-center"
                >
                    <v-progress-circular
                        color="primary"
                        indeterminate
                        size="30"
                    ></v-progress-circular>
                </div>
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
            draggable="true"
            @dragstart="(e) => dragStart(e)"
            ref="backDrag"
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
                    <v-img
                        style="pointer-events: none"
                        :src="`races/${character.race}.png`"
                        max-width="8rem"
                    ></v-img>
                    <v-img
                        style="pointer-events: none"
                        class="factionLogo right"
                        :src="`factions/${character.faction}.svg`"
                        min-width="200px"
                    ></v-img>
                    <br />
                    Lvl: ipsum<br />
                    Ilvl:
                    {{ Math.round(character.averageItemLevel) }}</v-card-text
                >
            </div>
        </v-card>
    </div>

    <div
        v-if="dragging"
        ref="trash"
        class="trash"
        :class="{ active: overTrash }"
    >
        <v-btn
            :style="
                overTrash
                    ? 'background-color: rgba(255, 0, 0, 0.8)'
                    : 'background-color: rgba(255, 0, 0, 0.4)'
            "
            icon="mdi-delete"
        ></v-btn>
    </div>
</template>

<script>
import { window as _window } from "@tauri-apps/api";
import { getClient, ResponseType } from "@tauri-apps/api/http";
const client = await getClient();

export default {
    name: "CharacterView",
    emits: [
        "rotated",
        "filled",
        "bosskills",
        "dragging",
        "dragging:stop",
        "dragging:delete",
    ],
    props: {
        chars: {},
        hidden: false,
        loading: true,
        classS: "",
        character: {
            name: String,
            default: "",
            level: String,
            default: "",
        },
        characterIds: {},
        rotated: false,
        empty: false,
        isFilled: false,
        lastWed: new Date(),
    },
    data() {
        return {
            charToAdd: "",
            charData: "",
            x: 0,
            y: 0,
            draglisten: undefined,
            overTrash: false,
            dragging: false,
        };
    },
    async created() {},
    computed: {
        hasId() {
            if (this.characterIds) {
                const res = new Date(
                    ...this.characterIds?.ds[0].time
                        .split(/[\s/:]/)
                        .map((el, idx) => {
                            if (idx == 1) return parseInt(el) - 1;
                            return parseInt(el);
                        })
                );
                console.log("date", res, new Date(this.lastWed), this.lastWed);
                return res < new Date(this.lastWed);
            }
            return "false";
        },
        cssVar() {
            return `url("/races/${this.character.race}.svg")`;
        },
        getFactionCss() {
            return `url("/factions/${this.character.faction}.svg")`;
        },
        cardBg() {
            return `rgba(var(--v-theme-${
                this.character.faction == "0" ? "horde" : "alliance"
            }),0.4)`;
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
        enterDrop(e) {
            console.log("asdasdsadsa");
        },
        dragStart(e) {
            const node = e.target.classList.contains("front")
                ? this.$refs.frontDrag
                : this.$refs.backDrag;
            console.log(node);
            const rect = e.target.getBoundingClientRect();
            this.dragging = true;
            this.$emit("dragging", this.character);
            const clone = node.$el.cloneNode(true);
            clone.classList.add("norip");
            clone.style.width = `${e.target.offsetWidth}px`;
            clone.style.height = `${e.target.offsetHeight}px`;
            clone.style.position = "absolute";
            clone.style.backgroundColor = "rgba(0,0,0,0.5)";
            document.body.appendChild(clone);

            this.x = e.clientX;
            this.y = e.clientY;
            const tarW = e.target.offsetWidth / 2;
            const tarH = e.target.offsetHeight / 2;
            clone.style.left = `${this.x - tarW}px`;
            clone.style.top = `${this.y - tarH}px`;

            const listener = (event) => {
                this.x = event.clientX;
                this.y = event.clientY;
                clone.style.left = `${this.x - tarW}px`;
                clone.style.top = `${this.y - tarH}px`;
                clone.style.transition = `all 0.1s ease-out`;
                const rect1 = clone.getBoundingClientRect();
                const rect2 = this.$refs.trash.getBoundingClientRect();
                var overlap = !(
                    rect1.right < rect2.left ||
                    rect1.left > rect2.right ||
                    rect1.bottom < rect2.top ||
                    rect1.top > rect2.bottom
                );
                this.overTrash = overlap;
                if (overlap) {
                    clone.classList.add("active");
                } else {
                    clone.classList.remove("active");
                }
            };
            this.draglisten = window.addEventListener("mousemove", listener);
            window.addEventListener(
                "mouseup",
                (event) => {
                    const dist = Math.sqrt(
                        Math.pow(
                            Math.max(
                                this.x - e.target.offsetWidth / 2,
                                rect.left
                            ) -
                                Math.min(
                                    this.x - e.target.offsetWidth / 2,
                                    rect.left
                                ),
                            2
                        ) +
                            Math.pow(
                                Math.max(
                                    this.y - e.target.offsetHeight / 2,
                                    rect.top
                                ) -
                                    Math.min(
                                        this.y - e.target.offsetHeight / 2,
                                        rect.top
                                    ),
                                2
                            )
                    );
                    this.dragging = false;
                    if (this.overTrash) {
                        this.$emit("dragging:delete");
                        clone.remove();
                    } else {
                        clone.style.transition = `all ${dist / 2000}s linear`;
                        clone.style.left = `${rect.left}px`;
                        clone.style.top = `${rect.top}px`;

                        setTimeout(() => {
                            this.$emit("dragging:stop");
                            clone.remove();
                        }, dist / 2);
                    }

                    window.removeEventListener("mousemove", listener);
                },
                { once: true }
            );
        },
        async getCharLevels() {
            if (
                this.chars.find(
                    (el) =>
                        el?.name?.toLowerCase() ==
                        this?.charToAdd?.toLowerCase()
                ) !== undefined
            ) {
                console.log("kokotina");
                return;
            }
            let charUrl = `https://cata-twinhead.twinstar.cz/?character=${this.charToAdd}&realm=Apollo`;
            await client
                .get(charUrl, {
                    // the expected response type
                    responseType: ResponseType.Text,
                })
                .catch((error) => {
                    console.log(error);
                })
                .then((el) => {
                    const buh = [
                        ...el.data.matchAll(/(?<=data: )\s*.*(?=\s}\);)/g),
                    ];
                    const switcher = ["bot", "bwd", "totfw", "fl", "bh", "ds"];
                    const bossKills = buh.reduce(
                        (acc, el, idx) => {
                            console.log(el);
                            acc[`${switcher[idx]}`] = JSON.parse(el["0"]);
                            return acc;
                        },
                        { bot: [], bwd: [], totfw: [], fl: [], bh: [], ds: [] }
                    );
                    try {
                        if (el.ok == false) return;
                        this.$emit(
                            "bosskills",
                            bossKills,
                            this.charToAdd,
                            this.charData
                        );
                        this.charToAdd = "";
                        setTimeout(() => {
                            this.$emit("rotated");
                        }, 20);
                    } catch (e) {
                        console.log(e);
                    }
                });
        },
        async gimmeChar() {
            let toonUrl = `https://twinstar-api.twinstar-wow.com/character/?name=${this.charToAdd}&realm=Apollo`;
            if (
                this.chars.find(
                    (el) =>
                        el.name.toLowerCase() == this.charToAdd.toLowerCase()
                ) !== undefined
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
                        this.charData = el.data;
                    } catch (e) {
                        console.log(e);
                    }
                });
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
.v-card.active {
    background-color: rgba(255, 0, 0, 0.4) !important;
}

.trash {
    position: absolute;
    left: 50%;
    bottom: 1%;
    transition: all 0.5s ease-in-out;
}

.trash.active {
    bottom: 5%;
    filter: drop-shadow(1px 1px 4px red);
    transition: all 0.5s ease-in-out;
}

.cloned {
    top: 0;
    left: 0;
}

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
    width: 100%;
    height: 100%;
    cursor: default !important;
}

.back:not(.norip),
.front:not(.norip) {
    transition: transform 0.5s !important;
}

input:focus-within {
    border-bottom: 1px solid rgb(var(--v-theme-primary)) !important;
}

.back:not(.norip) {
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

.norip .v-ripple__container {
    display: none !important;
}

*::-webkit-scrollbar {
    display: none;
}
</style>

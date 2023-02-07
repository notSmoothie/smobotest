<template>
    <v-app>
        <div data-tauri-drag-region class="d-flex align-center justify-space-between title">
            <div class="d-flex align-center"> 
                <v-img data-tauri-drag-region draggable="false" style="user-select: none; pointer-events: none;"
                    src="/logo.svg" height="25px" width="25px" class="ma-2"
                />
                <span data-tauri-drag-region class="ma-1" style="user-select: none">SMOBOT</span>
                
            </div>
            <div>
                <v-btn
                @click="minimize"
                icon="mdi-minus"
                variant="text"
                size="25px"
                class="mr-2 my-2 text-white"
                
              ></v-btn>
                <v-btn
                @click="kill"
                icon="mdi-close"
                color="red"
                variant="text"
                size="25px"
                class="mr-2 my-2  text-black"
                
              ></v-btn>
              </div>
                
        </div>
        
        <v-main >
            
        </v-main>
        <div class="d-flex align-center justify-center text-body-2 footer">
           
          Copyright &copy 2023 by notSmoothie
                
        </div>
 
    </v-app>
</template>

<script>
import {window as _window} from '@tauri-apps/api'
export default { 
    name: "App",
    methods:{
        kill(){
            _window.appWindow.close()
        },
        minimize(){
            _window.appWindow.hide();
        }
    } 
};


</script>

<style>
path {
    fill: #3B444B;
}
.title {
    position: fixed;
    width: 100%;
    background-color: #3B444B;
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

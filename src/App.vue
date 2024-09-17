<template>
  <Dialog v-model:visible="dialog_visible" :closable="false" :modal="true">
    <template #header>Load a file...</template>
    <FileLoader
      class="loader"
      :done="
        () => {
          dialog_visible = false;
        }
      "
    />
  </Dialog>

  <VFlex :gap="0">
    <Titlebar />
    <!-- <Button @click="dialog_visible = true">Open Dialog</Button> -->
    <!-- <div class="content"> -->
    <div :class="'router-wrapper'">
      <RouterView v-slot="{ Component, route }">
        <Transition :name="route.meta?.transition as (string|undefined) || 'slide-up'">
          <component :is="Component"></component>
        </Transition>
      </RouterView>
    </div>
    <!-- </div> -->
  </VFlex>
</template>

<script setup lang="ts">
import { ref } from "vue";
import FileLoader from "./components/file_loader/FileLoader.vue";
import { RouterView } from "vue-router";
import Titlebar from "./components/Titlebar.vue";
import VFlex from "./components/flex/VFlex.vue";

const dialog_visible = ref(false);

/*
function encodeSvg(svgString) {
  return "data:image/svg+xml," + svgString.replace('<svg',(~svgString.indexOf('xmlns')?'<svg':'<svg xmlns="http://www.w3.org/2000/svg"'))
        
        //
        //   Encode (may need a few extra replacements)
        //
        .replace(/"/g, '\'')
        .replace(/%/g, '%25')
        .replace(/#/g, '%23')       
        .replace(/{/g, '%7B')
        .replace(/}/g, '%7D')         
        .replace(/</g, '%3C')
        .replace(/>/g, '%3E')

        .replace(/\s+/g,' ') 
        // 
        //    The maybe list (add on documented fail)
        // 
        //  .replace(/&/g, '%26')
        //  .replace('|', '%7C')
        //  .replace('[', '%5B')
        //  .replace(']', '%5D')
        //  .replace('^', '%5E')
        //  .replace('`', '%60')
        //  .replace(';', '%3B')
        //  .replace('?', '%3F')
        //  .replace(':', '%3A')
        //  .replace('@', '%40')
        //  .replace('=', '%3D')
  ;}
*/
</script>

<style scoped>
.v-flex {
  height: 100%;
}

.loader {
  width: 80vw;
  height: 80vh;
}

.router-wrapper {
  width: 100%;
  flex-grow: 1;
  overflow-y: auto;
  overflow-x: hidden;
}

.slide-up-enter-active,
.slide-up-leave-active,
.slide-down-enter-active,
.slide-down-leave-active {
  transition-duration: 0.8s;
  transition-property: transform opacity;
  transition-timing-function: cubic-bezier(0.27, 1.1, 0.46, 1.01);
}

.slide-up-leave-active,
.slide-down-leave-active {
  position: absolute;
  inset: 0;
}

.slide-up-leave-to,
.slide-down-enter-from {
  transform: translateY(-30rem);
  opacity: 0;
}

.slide-up-enter-from,
.slide-down-leave-to {
  opacity: 0;
  transform: translateY(30rem);
}
</style>

<style>
:root {
  font-family: "Inter", sans-serif;
  font-optical-sizing: auto;
  font-style: normal;
  user-select: none;
}

::-webkit-scrollbar {
  width: 12px;
  height: 12px;
}

::-webkit-scrollbar-thumb {
  background-color: var(--p-primary-color);
  border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
  background-color: var(--p-primary-hover-color);
}

::-webkit-scrollbar-track {
  border-radius: 4px;
}

::-webkit-scrollbar-track:hover {
  background-color: var(--p-surface-100);
}

.no-margin {
  margin: 0;
}
</style>

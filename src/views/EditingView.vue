<template>
  <div>
    <Tabs :value="current_id" :pt:root:class="'tabs-scroll'">
      <TabList :pt:tabList:class="'tablist'">
        <RouterLink
          v-for="[value, sheet] in sheets.entries()"
          class="no-margin router-link"
          :to="{ name: 'sheet', params: { id: value } }"
          v-slot="{ navigate, route }"
        >
          <Tab
            @click="(event:MouseEvent) => triggerNavigation(event, route, navigate )"
            :value="value"
            :pt:root:class="'sheet-tab'"
          >
            {{ sheet.title }}
          </Tab>
        </RouterLink>
        <Button
          icon="pi pi-plus"
          size="small"
          :pt:root:class="'sheet-create'"
          @click="addNew"
        />
      </TabList>
    </Tabs>
    <div class="router-wrapper">
      <RouterView v-slot="{ Component, route }">
        <Transition :name="animationName">
          <component :is="Component" :key="route.path" />
        </Transition>
      </RouterView>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from "vue";
import { NavigationFailure, RouteLocationResolvedGeneric, useRouter } from "vue-router";

const router = useRouter();

const current_id = computed(() =>
  parseInt(router.currentRoute.value.params.id as string)
);

const animationName = ref("");

const sheets = ref([
  {
    title: "First",
  },
  {
    title: "Second",
  },
  {
    title: "Third",
  },
]);

function addNew() {
  sheets.value.push({ title: "Extra" });
}

function triggerNavigation(
  event: MouseEvent,
  route: RouteLocationResolvedGeneric,
  navigate: (e?: MouseEvent) => Promise<void | NavigationFailure>
) {
  console.log(`From to ${router.currentRoute.value.params.id} ${route.params.id}`);
  animationName.value =
    parseInt(route.params.id as string) < current_id.value
      ? "sheet-view-right"
      : "sheet-view-left";

  navigate(event);
}
</script>

<style scoped>
.router-link {
  text-decoration: none;
  -webkit-user-drag: none;
}

.tabs-scroll {
  overflow-x: scroll;
}

.sheet-tab {
  padding: 0.5rem 0.75rem;
}

.sheet-create {
  margin: 0;
  border-left: 1px var(--p-surface-400) solid;
  border-radius: 0;
}

.sheet-create:hover {
  border-left: 1px var(--p-surface-400) solid;
}

.router-wrapper {
  overflow-x: hidden;
  overflow-y: hidden;
  position: relative;
  width: 100%;
  height: 100%;
}

.sheet-view-left-enter-active,
.sheet-view-left-leave-active,
.sheet-view-right-enter-active,
.sheet-view-right-leave-active {
  transition-duration: 0.8s;
  transition-property: transform opacity;
  transition-timing-function: cubic-bezier(0.27, 1.1, 0.46, 1.01);
}

.sheet-view-left-leave-active,
.sheet-view-right-leave-active {
  position: absolute;
  inset: 0;
}

.sheet-view-left-leave-to,
.sheet-view-right-enter-from {
  transform: translateX(-50%);
  opacity: 0;
}

.sheet-view-left-enter-from,
.sheet-view-right-leave-to {
  opacity: 0;
  transform: translateX(50%);
}
</style>

<style>
div.tablist {
  background-color: var(--p-surface-200);
}
</style>

<template>
  <div data-tauri-drag-region class="titlebar-base">
    <HFlex :gap="0" static>
      <div class="logo" />
      <div
        v-for="{ button, index } in model.start?.map((v, i) => ({ button: v, index: i }))"
      >
        <Button
          size="small"
          text
          :label="button.label"
          :icon="button.icon"
          :severity="button.severity"
          :plain="button.plain"
          @click="(event: MouseEvent) => toggleMenu(index, event)"
        />
        <Menu
          :model="toMenuBar(button.items)"
          v-if="button.items"
          popup
          :ref="setupRefs.bind(undefined, index)"
        />
      </div>
    </HFlex>
    <HFlex :gap="0" static>
      <div v-for="button in model.end">
        <Button
          text
          size="small"
          :label="button.label"
          :icon="button.icon"
          :severity="button.severity"
          :plain="button.plain"
          @click="
            (event) => {
              if (button.command) button.command({ original_event: event, item: button });
            }
          "
        />
      </div>
    </HFlex>
  </div>
</template>

<script setup lang="ts">
import { computed, onBeforeUpdate, onMounted, Ref, ref } from "vue";
import { TitlebarDef, TitlebarItem } from "./titlebar";
import HFlex from "./flex/HFlex.vue";
import { MenuItemCommandEvent } from "primevue/menuitem";
import { useRouter } from "vue-router";
import { appWindow } from "@tauri-apps/api/window";

const router = useRouter();

const menu_refs: Map<number, Ref<any>> = new Map();

const window_maxed = ref(false);

onMounted(() => {
  appWindow.isMaximized().then((maximized) => {
    window_maxed.value = maximized;
  });
});

appWindow.onMoved(() => {
  appWindow.isMaximized().then((maximized) => {
    window_maxed.value = maximized;
  });
});

const model = computed<TitlebarDef>(() => {
  return {
    start: [
      {
        label: "File",
        items: [
          {
            label: "Open File...",
            route: {
              name: "sheet",
              params: {
                id: "0",
              },
            },
          },
          {
            label: "Test",
            route: {
              name: "load",
            },
          },
        ],
      },
      {
        label: "Edit",
      },
      {
        label: "Help",
        items: [{ label: "About" }],
      },
    ],
    end: [
      {
        icon: "pi pi-minus",
        plain: true,
        command: () => {
          appWindow.minimize();
        },
      },
      {
        icon: window_maxed.value
          ? "pi pi-arrow-down-left-and-arrow-up-right-to-center"
          : "pi pi-arrow-up-right-and-arrow-down-left-from-center",
        plain: true,
        command: () => {
          appWindow.toggleMaximize();
        },
      },
      {
        icon: "pi pi-times",
        severity: "danger",
        command: () => {
          appWindow.close();
        },
      },
    ],
  };
});

function toMenuBar(items: TitlebarItem[]): import("primevue/menuitem").MenuItem[] {
  return items.map(
    ({
      label,
      separator,
      items: source_items,
      icon,
      route,
      command: source_command,
      disabled,
    }) => {
      const items = (() => {
        if (source_items === undefined) {
          return undefined;
        } else {
          return toMenuBar(source_items);
        }
      })();

      const command = (() => {
        if (source_command === undefined && route === undefined) {
          return undefined;
        } else {
          const commands: ((event: MenuItemCommandEvent) => void)[] = [];
          if (route !== undefined) {
            commands.push(() => {
              router.push(route);
            });
          }
          if (source_command !== undefined) {
            commands.push((event: any) => {
              source_command({ original_event: event.originalEvent, item: event.item });
            });
          }
          return (event: MenuItemCommandEvent) =>
            commands.forEach((comm) => {
              comm(event);
            });
        }
      })();
      return {
        label,
        separator,
        items,
        icon,
        disabled,
        command,
      };
    }
  );
}

function toggleMenu(index: number, event: MouseEvent) {
  menu_refs.get(index)?.value.toggle(event);
}

onBeforeUpdate(() => {
  menu_refs.clear();
});

function setupRefs(index: number, reference: any) {
  menu_refs.set(index, ref(reference));
}
</script>

<style scoped>
.titlebar-base {
  position: relative;
  z-index: 100;
  background-color: var(--p-surface-0);
  width: 100%;
  margin: 0;
  border-bottom: 1px var(--p-surface-400) solid;
  display: flex;
  white-space: nowrap;
  justify-content: space-between;
}

.h-space {
  width: 20px;
}
</style>

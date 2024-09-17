<template>
  <div :class="'horizontal-scroller'">
    <HFlex static :class="'base'" :gap="10">
      <div v-for="data in file_stats" :class="{ 'list-end': data.end }">
        <CFlex v-if="data.end">
          <template #content>
            <div>Nothing to see here...</div>
          </template>
        </CFlex>

        <FileCard
          v-model:favourite="data.favourite"
          :title="data.title"
          :stats="data.stats"
          :class="['card', active_item?.full == `${prefix}-${data.id}` ? 'selected' : '']"
          @clicked="select(prefix, data.id)"
          v-else
        />
      </div>
    </HFlex>
  </div>
</template>

<script setup lang="ts">
import HFlex from "../flex/HFlex.vue";
import CFlex from "../flex/CFlex.vue";
import FileCard from "./FileCard.vue";
import { FileStat, FileStatEnd } from "./file_stats";

const { select, prefix, active_item } = defineProps<{
  select: (prefix: string, id: string) => void;
  prefix: string;
  active_item: { full: string; id: string } | undefined;
}>();

const file_stats = defineModel<(FileStat | FileStatEnd)[]>("file_stats", {
  required: true,
});
</script>

<style scoped>
.h-flex.base {
  margin: 0 5px;
}

.horizontal-scroller {
  padding: 0 0 1rem 0;
  overflow-x: scroll;
  margin: 10px 0;
}

.list-end {
  white-space: nowrap;
  flex-grow: 1;
  color: var(--p-text-muted-color);
}

.list-end > div > div {
  padding: 0 100px;
}

.card {
  cursor: pointer;
  position: relative;
  transition: all 320ms ease-in-out;
  outline: 2px solid transparent;
}

.card.selected {
  transition: all 320ms ease-in-out;
  transform: translateY(-0.5rem);
  box-shadow: 0 1rem 0.5rem var(--p-surface-300);
  outline: 2px solid var(--p-primary-color);
  filter: brightness(1.05);
}

.card:active:not(:has(.favourite-icon:hover)):not(.selected) {
  filter: brightness(0.75);
}

.card:hover:not(:has(.favourite-icon:hover)):not(.selected):not(:active) {
  filter: brightness(0.95);
}
</style>

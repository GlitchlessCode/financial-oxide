<template>
  <div class="file-loader-split">
    <Card :pt:root:class="'file-loader-info-card'">
      <template #header>
        <HFlex>
          <Button @click="runDone" fluid>Open File...</Button>
          <b>OR</b>
          <Button @click="runDone" fluid disabled>Open Selected</Button>
        </HFlex>
      </template>
      <template #content>
        <CFlex>
          <template #content>
            <div>Nothing to see here...</div>
          </template>
        </CFlex>
      </template>
    </Card>
    <div>
      <h2>Recents</h2>
      <FileCardSet
        v-model:file_stats="recent_file_stats"
        :select="select"
        prefix="recent"
        :active_item="active_item"
      />
      <Divider />
      <h2>Favourites</h2>
      <FileCardSet
        v-model:file_stats="favourite_file_stats"
        :select="select"
        prefix="favourite"
        :active_item="active_item"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import FileCardSet from "./FileCardSet.vue";
import { ref } from "vue";
import { FileStat, FileStatEnd } from "./file_stats";
import HFlex from "../flex/HFlex.vue";

const active_item = ref<{ id: string; full: string }>();

const { done } = defineProps({
  done: { type: Function, required: true },
});

const recent_file_stats = ref<(FileStat | FileStatEnd)[]>([
  {
    id: "a",
    title: "TextA",
    stats: [
      { title: "Sheets", value: "3" },
      { title: "Rows", value: "1024" },
      { title: "Total", value: "$11 243" },
      { title: "Last Opened", value: "September 9, 2024" },
    ],
    favourite: true,
    end: false,
  },
  {
    id: "b",
    title: "TextB",
    stats: [
      { title: "Sheets", value: "2" },
      { title: "Rows", value: "472" },
      { title: "Total", value: "$6 298" },
      { title: "Last Opened", value: "August 17, 2024" },
    ],
    favourite: false,
    end: false,
  },
  {
    id: "c",
    title: "TextB",
    stats: [
      { title: "Sheets", value: "2" },
      { title: "Rows", value: "472" },
      { title: "Total", value: "$6 298" },
      { title: "Last Opened", value: "August 17, 2024" },
    ],
    favourite: false,
    end: false,
  },
  {
    id: "d",
    title: "TextB",
    stats: [
      { title: "Sheets", value: "2" },
      { title: "Rows", value: "472" },
      { title: "Total", value: "$6 298" },
      { title: "Last Opened", value: "August 17, 2024" },
    ],
    favourite: false,
    end: false,
  },
  {
    id: "e",
    title: "TextB",
    stats: [
      { title: "Sheets", value: "2" },
      { title: "Rows", value: "472" },
      { title: "Total", value: "$6 298" },
      { title: "Last Opened", value: "August 17, 2024" },
    ],
    favourite: false,
    end: false,
  },
  {
    id: "f",
    title: "TextB",
    stats: [
      { title: "Sheets", value: "2" },
      { title: "Rows", value: "472" },
      { title: "Total", value: "$6 298" },
      { title: "Last Opened", value: "August 17, 2024" },
    ],
    favourite: false,
    end: false,
  },
  {
    end: true,
  },
]);

const favourite_file_stats = ref<(FileStat | FileStatEnd)[]>([
  {
    id: "a",
    title: "TextA",
    stats: [
      { title: "Sheets", value: "3" },
      { title: "Rows", value: "1024" },
      { title: "Total", value: "$11 243" },
      { title: "Last Opened", value: "September 9, 2024" },
    ],
    favourite: true,
    end: false,
  },
  { end: true },
]);

function runDone() {
  done();
}

function select(prefix: string, id: string) {
  active_item.value = { id, full: `${prefix}-${id}` };
}
</script>

<style scoped>
.file-loader-split {
  display: grid;
  grid-template-columns: 2fr 5fr;
  height: 100%;
  gap: 0.3rem;
}

.file-loader-split > div {
  min-width: 0;
  overflow-y: auto;
}

.horizontal-scroller {
  width: 100%;
}

.file-loader-info-card {
  margin: 0.3rem;
}

.file-loader-info-card .h-flex {
  padding: 1.25rem;
  white-space: nowrap;
}

.file-loader-info-card .c-flex {
  white-space: nowrap;
  color: var(--p-text-muted-color);
}

h2 {
  margin: 0.5rem 0;
}
</style>

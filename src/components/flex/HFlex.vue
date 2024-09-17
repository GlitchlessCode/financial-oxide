<template>
  <div :class="['h-flex', { static: static }]" :style="style">
    <slot />
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";

const props = defineProps({
  static: {
    type: Boolean,
    default: false,
  },
  stretch: {
    type: Boolean,
    default: false,
  },
  gap: {
    type: Number,
    default: 8,
  },
});

const style = computed(
  () => `--gap:${props.gap}px; --align-type: ${props.stretch ? "stretch" : "center"}`
);
</script>

<style scoped>
.h-flex {
  display: flex;
  flex-direction: row;
  gap: var(--gap);
  align-items: var(--align-type);
}

@media screen and (max-aspect-ratio: 4/3) {
  .h-flex:not(.static) {
    flex-direction: column;
  }
}
</style>

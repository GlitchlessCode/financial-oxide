<template>
  <div class="v-flex" :style="style">
    <slot />
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";

const props = defineProps({
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
.v-flex {
  display: flex;
  flex-direction: column;
  gap: var(--gap);
  align-items: var(--align-type);
}
</style>

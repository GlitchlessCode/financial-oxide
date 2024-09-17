<template>
  <div>
    <h2>Sheet</h2>
    <Tag rounded :value="props.id"></Tag>
    <div class="fill"></div>
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{ id: string }>();
</script>

<style scoped>
.fill {
  width: 50%;
  height: 5rem;
  background-color: var(--p-surface-400);
}
</style>

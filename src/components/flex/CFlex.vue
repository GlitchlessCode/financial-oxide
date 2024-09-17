<template>
  <div class="c-flex">
    <slot name="content" />
  </div>
</template>

<script setup lang="ts"></script>

<style scoped>
.c-flex {
  display: flex;
  height: 100%;
  justify-content: center;
  align-items: center;
}
</style>

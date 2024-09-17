import { defineStore } from "pinia";
import { ref } from "vue";

export const useActiveFileStore = defineStore("active_file", () => {
  const active = ref();

  const has_active = ref(false);

  return { active, has_active };
});

<template>
  <Card :pt:root:class="'card-base'" @click.left="toggle_clicked" @mousemove="mousemove">
    <template #header>
      <div class="header">
        <div class="favourite-icon-wrapper">
          <i
            :class="['favourite-icon', 'pi', favourite ? 'pi-star-fill' : 'pi-star']"
            v-ripple
            @click.capture.stop.prevent="toggle_favourite"
          />
        </div>

        <img src="https://placehold.co/1280x720" />
      </div>
    </template>
    <template #title>{{ title }}</template>
    <template #content>
      <div class="card-grid">
        <HFlex
          static
          stretch
          style="justify-content: space-between"
          :gap="10"
          class="grid-row"
          v-for="stat in stats"
        >
          <div>
            <b>{{ stat.title }}</b>
          </div>
          <Divider />
          <div>
            <Tag rounded :value="stat.value" severity="contrast" />
          </div>
        </HFlex>
      </div>
    </template>
  </Card>
</template>

<script setup lang="ts">
const { title, stats } = defineProps<{
  title: string;
  stats: { title: string; value: string }[];
}>();

const favourite = defineModel<boolean>("favourite", { required: true });

const emit = defineEmits<{ clicked: [] }>();

function toggle_clicked() {
  emit("clicked");
}

function toggle_favourite() {
  favourite.value = !favourite.value;
}

function mousemove(event: MouseEvent) {
  if (event.currentTarget instanceof HTMLDivElement) {
    const card = event.currentTarget;
    const bounds = card.getBoundingClientRect();

    const mouse = { x: event.clientX - bounds.x, y: event.clientY - bounds.y };

    const middle = { x: bounds.width / 2, y: bounds.height / 2 };
    const offset = {
      x: ((mouse.x - middle.x) / middle.x) * 6,
      y: ((mouse.y - middle.y) / middle.y) * -6,
    };
    card.style.setProperty("--rotateX", `${offset.x}deg`);
    card.style.setProperty("--rotateY", `${offset.y}deg`);
  }
}
</script>

<style scoped>
.card-base {
  overflow: hidden;
  transform: perspective(1500px) rotateY(0) rotateX(0);
  transition: transform 1s ease-out;
}

.card-base:hover {
  transform: perspective(1500px) rotateY(var(--rotateX)) rotateX(var(--rotateY));
  transition: transform 0s;
}

.card-grid {
  min-width: 300px;
  display: grid;
  grid-template-rows: 1fr 1fr 1fr 1fr;
}

.card-grid .h-flex > div {
  white-space: nowrap;
}

.grid-row > div {
  display: flex;
  justify-content: center;
  align-items: center;
}

.header {
  width: 100%;
  height: 16rem;
  overflow: hidden;
  position: relative;
}

.header img {
  position: absolute;
  left: 50%;
  top: 0;
  height: 100%;
  transform: translateX(-50%);
  z-index: 0;
  pointer-events: none;
}

.header .favourite-icon-wrapper {
  position: absolute;
  top: 0.5rem;
  right: 0.5rem;
  z-index: 6;
}

.favourite-icon {
  padding: 0.5rem;
  border-radius: 50%;
  font-size: larger;
  color: var(--p-primary-color);
  cursor: pointer;
}

.favourite-icon:hover {
  background: #33333333;
}
</style>

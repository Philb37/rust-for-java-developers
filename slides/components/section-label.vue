<script setup lang="ts">
// Subtle running indicator of the current section, pinned vertically
// to the left edge of content slides. Pass the section name via `name`.
defineProps<{ name?: string }>()
</script>

<template>
  <div v-if="name" class="section-label">{{ name }}</div>
</template>

<style scoped>
.section-label {
  position: fixed;
  left: 0.55rem;
  top: 50%;
  transform: translateY(-50%) rotate(180deg);
  writing-mode: vertical-rl;
  font-family: 'Space Grotesk', ui-sans-serif, system-ui, sans-serif;
  font-size: 0.72rem;
  font-weight: 600;
  letter-spacing: 0.22em;
  text-transform: uppercase;
  color: var(--talk-purple, #c94ee5);
  opacity: 0.45;
  user-select: none;
  pointer-events: none;
  z-index: 90;
}
</style>

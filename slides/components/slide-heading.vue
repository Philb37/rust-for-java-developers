<script setup lang="ts">
// Visible, centered slide title for slides that don't use the `topic` layout
// (e.g. two-column slides, where it goes in the header slot). Shares the global
// `.top-title` look so every slide title is identical.
defineProps<{ text?: string }>()
</script>

<template>
  <h2 class="top-title">{{ text }}</h2>
</template>

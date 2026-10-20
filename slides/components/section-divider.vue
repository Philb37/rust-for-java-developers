<script setup lang="ts">
// Centered section title card. Use on a `layout: center` slide:
//   <SectionDivider index="02" name="Why Rust ?" />
// The <h1> picks up the global gradient + heading font automatically.
defineProps<{ index?: string; name?: string }>()
</script>

<template>
  <div class="sd">
    <div v-if="index" class="sd__index">{{ index }}</div>
    <h1 class="sd__title">{{ name }}</h1>
    <div class="sd__rule" />
  </div>
</template>

<style scoped>
.sd {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1.1rem;
}
.sd__index {
  font-family: 'Space Grotesk', ui-sans-serif, system-ui, sans-serif;
  font-size: 1rem;
  font-weight: 600;
  letter-spacing: 0.35em;
  opacity: 0.45;
}
.sd__title {
  font-size: 3.6rem;
  line-height: 1.05;
  margin: 0;
  text-align: center;
}
.sd__rule {
  width: 4rem;
  height: 4px;
  border-radius: 9999px;
  background: linear-gradient(90deg, #c94ee5, #e39d41);
}
</style>

<script setup lang="ts">
import { computed } from 'vue'
import { useNav } from '@slidev/client'

// Slidev's navigation store — reactive, updates automatically on every slide change.
const { currentPage, total } = useNav()

// Hide the counter on the cover slide (page 1).
const visible = computed(() => currentPage.value > 1)

// Progress as a percentage, used for the slim top bar.
const progress = computed(() =>
  total.value ? (currentPage.value / total.value) * 100 : 0,
)
</script>

<template>
  <!-- Slim progress bar pinned to the top of every slide -->
  <div
    v-if="visible"
    class="slide-number-progress-bar"
    :style="{ width: `${progress}%` }"
  />

  <!-- Page counter, bottom-right corner -->
  <div class="slide-number-slide-counter">
    {{ currentPage }}<span class="sep">/</span>{{ total }}
  </div>
</template>

<style scoped>
.slide-number-progress-bar {
  position: fixed;
  top: 0;
  left: 0;
  height: 3px;
  background: linear-gradient(90deg, #c94ee5, #e39d41);
  z-index: 100;
  transition: width 0.3s ease;
  pointer-events: none;
}

.slide-number-slide-counter {
  position: fixed;
  right: 0.9rem;
  bottom: 0.55rem;
  font-family: 'Inter', ui-sans-serif, system-ui, sans-serif;
  font-size: 0.8rem;
  font-variant-numeric: tabular-nums;
  letter-spacing: 0.02em;
  opacity: 0.55;
  z-index: 100;
  pointer-events: none;
  user-select: none;
}

.slide-number-slide-counter .sep {
  margin: 0 0.25em;
  opacity: 0.5;
}
</style>

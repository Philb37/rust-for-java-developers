<script setup lang="ts">
// A small twoslash-like block to show compiler errors / program output
// underneath a code block. Works for any language (the messages are static).
defineProps<{
  type?: 'error' | 'stdout'
  text?: string
}>()
</script>

<template>
  <div class="code-output" :class="`code-output--${type ?? 'stdout'}`">
    <span class="code-output__tag">{{ type === 'error' ? 'rustc' : 'stdout' }}</span>
    <pre class="code-output__body">{{ text }}</pre>
  </div>
</template>

<style scoped>
.code-output {
  display: flex;
  align-items: flex-start;
  gap: 0.6em;
  font-family: var(--slidev-code-font-family, 'Fira Code', monospace);
  font-size: 0.66em;
  line-height: 1.45;
  padding: 0.45em 0.7em;
  border-radius: 6px;
  border-left: 3px solid;
}

.code-output__tag {
  flex: none;
  font-size: 0.85em;
  text-transform: uppercase;
  letter-spacing: 0.06em;
  opacity: 0.65;
  padding-top: 0.15em;
  user-select: none;
}

.code-output__body {
  margin: 0;
  padding: 0 !important;
  background: none !important;
  white-space: pre-wrap;
  overflow: visible;
}

.code-output--error {
  background: rgba(248, 81, 73, 0.13);
  border-color: #f85149;
  color: #ff9492;
}

.code-output--stdout {
  background: rgba(110, 118, 129, 0.14);
  border-color: #6e7681;
  color: #b6b8b9;
}
</style>

<template>
  <div class="slidev-layout topic-layout">
    <h2 v-if="$frontmatter.heading || $frontmatter.title" class="top-title">
      {{ $frontmatter.heading || $frontmatter.title }}
    </h2>
    <div class="topic-body">
      <slot />
    </div>
  </div>
</template>

<style scoped>
.topic-layout {
  height: 100%;
  display: flex;
  flex-direction: column;
}
.topic-body {
  flex: 1 1 0;
  min-height: 0;
  display: flex;
  flex-direction: column;
  justify-content: center;
}
</style>

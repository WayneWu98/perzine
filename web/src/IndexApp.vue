<script setup lang="ts">
import { privideThemeMode } from './hooks/useThemeMode';

privideThemeMode();
</script>

<template>
  <div>
    <h1 class="title">Hello, Index.</h1>
  </div>
</template>

<style lang="less" scoped>
.title {
  @apply text-primary;
}
</style>

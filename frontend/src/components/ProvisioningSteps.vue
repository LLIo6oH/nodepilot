<script setup lang="ts">
defineProps<{
  currentStep: number
  steps: string[]
}>()
</script>

<template>
  <ol class="steps card">
    <li v-for="(step, index) in steps" :key="step" :class="{ active: index <= currentStep }">
      <span class="dot" :class="index <= currentStep ? 'cool pulse' : 'idle'"></span>
      <span>{{ step }}</span>
    </li>
  </ol>
</template>

<script setup lang="ts">
defineProps<{
  currentStep: number
  steps: string[]
}>()
</script>

<template>
  <ol class="steps">
    <li v-for="(step, index) in steps" :key="step" :class="{ active: index <= currentStep }">
      <span class="step-dot" :class="index < currentStep ? 'done' : index === currentStep ? 'active' : 'idle'"></span>
      <span>{{ step }}</span>
      <small class="mono">{{ index < currentStep ? 'OK' : index === currentStep ? '...' : '' }}</small>
    </li>
  </ol>
</template>

<script setup lang="ts">
import GlowCore from '../components/GlowCore.vue'
import ProvisioningSteps from '../components/ProvisioningSteps.vue'

const props = defineProps<{
  currentStep: number
  steps: string[]
  ready: boolean
  logs: string[]
  expanded: boolean
}>()

const emit = defineEmits<{
  (e: 'openWorkspace'): void
  (e: 'toggleLogs'): void
}>()
</script>

<template>
  <section class="provision-screen">
    <GlowCore />
    <h2>Launching Atlas Runtime</h2>
    <ProvisioningSteps :steps="props.steps" :current-step="props.currentStep" />

    <button class="toggle" @click="emit('toggleLogs')">{{ expanded ? 'Hide' : 'Show' }} event stream</button>
    <pre v-if="expanded" class="event-log card mono">{{ props.logs.join('\n') }}</pre>

    <button class="cta" :disabled="!ready" @click="emit('openWorkspace')">Open workspace</button>
  </section>
</template>

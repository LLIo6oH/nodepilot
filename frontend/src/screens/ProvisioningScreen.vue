<script setup lang="ts">
import { computed } from 'vue'
import GlowCore from '../components/GlowCore.vue'
import ProvisioningSteps from '../components/ProvisioningSteps.vue'

const props = defineProps<{
  currentStep: number
  steps: string[]
  ready: boolean
  logs: string[]
  expanded: boolean
  startingProvision: boolean
  error: string
}>()

const emit = defineEmits<{
  (e: 'toggleLogs'): void
}>()

const progress = computed(() => Math.round(((Math.min(props.currentStep + 1, props.steps.length)) / props.steps.length) * 100))
</script>

<template>
  <section class="provision-layout">
    <header class="provision-topbar">
      <div class="brand-lockup">NodePilot</div>
      <div class="top-pills">
        <span class="status-pill">
          <span class="dot" :class="ready ? 'good' : 'warm pulse'"></span>
          {{ ready ? 'Atlas online' : 'Provisioning' }}
        </span>
        <button class="ghost" @click="emit('toggleLogs')">{{ expanded ? 'Hide' : 'Show' }} logs</button>
      </div>
    </header>

    <div class="provision-content">
      <div class="provision-main card rise">
        <GlowCore />
        <div class="eyebrow mono txt-up">{{ ready ? 'Handover' : 'Preparing Atlas session' }}</div>
        <h2>{{ props.steps[Math.min(props.currentStep, props.steps.length - 1)] || 'Loading Atlas runtime' }}</h2>
        <p class="txt-mute">{{ ready ? 'Atlas is online. Opening workspace...' : (props.startingProvision ? 'Preparing runtime session...' : 'Streaming runtime events...') }}</p>

        <div class="progress-wrap">
          <div class="progress-head mono">
            <span>Step {{ Math.min(props.currentStep + 1, props.steps.length) }} of {{ props.steps.length }}</span>
            <span>{{ progress }}%</span>
          </div>
          <div class="progress-bar"><span :style="{ width: `${progress}%` }"></span></div>
        </div>

        <p v-if="props.error" class="ui-error">{{ props.error }}</p>
      </div>

      <aside class="provision-side">
        <div class="card panel">
          <div class="panel-head">
            <span>Provisioning steps</span>
            <span class="mono">session atlas</span>
          </div>
          <ProvisioningSteps :steps="props.steps" :current-step="props.currentStep" />
        </div>

        <div class="card panel log-panel" v-if="expanded">
          <div class="panel-head">
            <span>Runtime log</span>
            <span class="mono"><span class="dot cool pulse"></span>streaming</span>
          </div>
          <pre class="event-log mono">{{ props.logs.join('\n') }}</pre>
        </div>
      </aside>
    </div>
  </section>
</template>

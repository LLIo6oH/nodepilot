<script setup lang="ts">
import ChatPanel from '../components/ChatPanel.vue'
import RuntimePanel from '../components/RuntimePanel.vue'
import Sidebar from '../components/Sidebar.vue'

type Message = {
  role: 'user' | 'assistant' | 'error'
  content: string
  toolUsed?: string
}

const props = defineProps<{
  status: string
  messages: Message[]
  latestTool: string
  runtimeCollapsed: boolean
  pending: boolean
  latestEvents: string[]
  error: string
}>()

const emit = defineEmits<{
  (e: 'toggleRuntime'): void
  (e: 'reset'): void
  (e: 'send', value: string): void
}>()
</script>

<template>
  <section class="workspace-layout">
    <Sidebar :status="props.status" />
    <main>
      <header class="workspace-header card">
        <div>
          <h1>Atlas</h1>
          <p>{{ props.status }} · personal runtime channel</p>
        </div>
        <div class="header-actions">
          <button @click="emit('toggleRuntime')">Toggle runtime panel</button>
          <button @click="emit('reset')">Reset demo</button>
        </div>
      </header>
      <p v-if="props.error" class="ui-error">{{ props.error }}</p>
      <ChatPanel :messages="props.messages" :pending="props.pending" @send="emit('send', $event)" />
    </main>
    <RuntimePanel
      :status="props.status"
      :latest-tool="props.latestTool"
      :collapsed="props.runtimeCollapsed"
      :latest-events="props.latestEvents"
    />
  </section>
</template>

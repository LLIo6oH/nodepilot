<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import AppShell from './components/AppShell.vue'
import { ClientError, api, eventsUrl } from './api/client'
import type { EventPayload } from './api/types'
import ProvisioningScreen from './screens/ProvisioningScreen.vue'
import SignInScreen from './screens/SignInScreen.vue'
import WorkspaceScreen from './screens/WorkspaceScreen.vue'

type Screen = 'signin' | 'provisioning' | 'workspace'
type ChatMessage = { role: 'user' | 'assistant' | 'error'; content: string; toolUsed?: string }

const STORAGE_KEY = 'nodepilot_agent_id'

const screen = ref<Screen>('signin')
const agentId = ref<string | null>(null)
const status = ref('requested')
const eventLogs = ref<string[]>([])
const stepIndex = ref(0)
const showLogs = ref(false)
const runtimeCollapsed = ref(true)
const chatPending = ref(false)
const messages = ref<ChatMessage[]>([])
const latestTool = ref('')

const steps = [
  'Requesting instance',
  'Launching runtime',
  'Booting Atlas',
  'Configuring workspace',
  'Connecting tools',
  'Atlas is online'
]

let source: EventSource | null = null

const ready = computed(() => status.value === 'ready')

function mapStep(message: string): number {
  const lower = message.toLowerCase()
  if (lower.includes('allocating')) return 1
  if (lower.includes('preparing')) return 2
  if (lower.includes('installing')) return 3
  if (lower.includes('connecting')) return 4
  if (lower.includes('ready')) return 5
  return stepIndex.value
}

function connectEvents(id: string) {
  source?.close()
  source = new EventSource(eventsUrl(id))
  source.addEventListener('agent_event', (event) => {
    const payload = JSON.parse((event as MessageEvent).data) as EventPayload
    status.value = payload.status === 'tool_event' ? status.value : payload.status
    stepIndex.value = mapStep(payload.message)
    eventLogs.value.push(`[${payload.timestamp}] ${payload.status}: ${payload.message}`)
  })
}

async function launchAtlas() {
  try {
    const agent = await api.createAgent('Atlas')
    agentId.value = agent.id
    localStorage.setItem(STORAGE_KEY, agent.id)
    screen.value = 'provisioning'
    await api.provisionAgent(agent.id)
    connectEvents(agent.id)
  } catch (error) {
    const message = error instanceof ClientError ? error.message : 'failed to launch atlas'
    messages.value.push({ role: 'error', content: message })
  }
}

function openWorkspace() {
  screen.value = 'workspace'
}

async function sendChat(message: string) {
  if (!agentId.value) return
  messages.value.push({ role: 'user', content: message })
  chatPending.value = true
  try {
    const response = await api.chat(agentId.value, message)
    latestTool.value = response.tool_used
    messages.value.push({ role: 'assistant', content: response.agent_response, toolUsed: response.tool_used })
  } catch (error) {
    const text = error instanceof ClientError ? error.message : 'chat request failed'
    messages.value.push({ role: 'error', content: text })
  } finally {
    chatPending.value = false
  }
}

function resetDemo() {
  localStorage.removeItem(STORAGE_KEY)
  source?.close()
  source = null
  screen.value = 'signin'
  agentId.value = null
  status.value = 'requested'
  messages.value = []
  eventLogs.value = []
  stepIndex.value = 0
  latestTool.value = ''
}

onMounted(async () => {
  const saved = localStorage.getItem(STORAGE_KEY)
  if (!saved) return

  agentId.value = saved
  try {
    const agents = await api.listAgents()
    const found = agents.find((value) => value.id === saved)
    if (!found) {
      resetDemo()
      return
    }

    status.value = found.status
    if (found.status === 'ready') {
      screen.value = 'workspace'
    } else {
      screen.value = 'provisioning'
    }
    connectEvents(saved)
  } catch {
    resetDemo()
  }
})
</script>

<template>
  <AppShell>
    <div class="app-frame">
      <SignInScreen v-if="screen === 'signin'" @launch="launchAtlas" />
      <ProvisioningScreen
        v-else-if="screen === 'provisioning'"
        :current-step="stepIndex"
        :steps="steps"
        :ready="ready"
        :logs="eventLogs"
        :expanded="showLogs"
        @toggle-logs="showLogs = !showLogs"
        @open-workspace="openWorkspace"
      />
      <WorkspaceScreen
        v-else
        :status="status"
        :messages="messages"
        :latest-tool="latestTool"
        :runtime-collapsed="runtimeCollapsed"
        :pending="chatPending"
        @send="sendChat"
        @toggle-runtime="runtimeCollapsed = !runtimeCollapsed"
        @reset="resetDemo"
      />
    </div>
  </AppShell>
</template>

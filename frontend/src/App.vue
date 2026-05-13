<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref, watch } from 'vue'
import AppShell from './components/AppShell.vue'
import { ClientError, api, eventsUrl } from './api/client'
import type { EventPayload } from './api/types'
import ProvisioningScreen from './screens/ProvisioningScreen.vue'
import SignInScreen from './screens/SignInScreen.vue'
import SubmissionLandingScreen from './screens/SubmissionLandingScreen.vue'
import WorkspaceScreen from './screens/WorkspaceScreen.vue'

type Screen = 'signin' | 'provisioning' | 'workspace'
type RouteView = 'landing' | 'app'
type ChatMessage = { role: 'user' | 'assistant' | 'error'; content: string; toolUsed?: string }

const STORAGE_KEY = 'nodepilot_agent_id'
const APP_PATH = '/app'

const screen = ref<Screen>('signin')
const routeView = ref<RouteView>('landing')
const agentId = ref<string | null>(null)
const backendStatus = ref('requested')
const eventLogs = ref<string[]>([])
const stepIndex = ref(0)
const showLogs = ref(false)
const runtimeCollapsed = ref(true)
const chatPending = ref(false)
const launchPending = ref(false)
const provisionPending = ref(false)
const uiError = ref('')
const messages = ref<ChatMessage[]>([])
const latestTool = ref('')
const localStatus = ref<'working' | 'error' | null>(null)

const steps = [
  'Preparing runtime session',
  'Loading Atlas runtime',
  'Booting Atlas',
  'Preparing workspace',
  'Connecting tools',
  'Atlas is online'
]

let source: EventSource | null = null
let readyTransitionTimer: ReturnType<typeof setTimeout> | null = null

const displayStatus = computed(() => localStatus.value ?? backendStatus.value)
const ready = computed(() => backendStatus.value === 'ready')
const latestEvents = computed(() => eventLogs.value.slice(-6).reverse())

function cleanupEvents() {
  source?.close()
  source = null
}

function clearReadyTransitionTimer() {
  if (readyTransitionTimer) {
    clearTimeout(readyTransitionTimer)
    readyTransitionTimer = null
  }
}

function mapStep(message: string): number {
  const lower = message.toLowerCase()
  if (lower.includes('preparing runtime session')) return 0
  if (lower.includes('loading atlas runtime')) return 1
  if (lower.includes('booting atlas') || lower.includes('creating workspace')) return 2
  if (lower.includes('preparing workspace') || lower.includes('mounting workspace')) return 3
  if (lower.includes('connecting tools') || lower.includes('registering tools')) return 4
  if (lower.includes('online') || lower.includes('ready')) return 5
  return stepIndex.value
}

function syncRouteView() {
  routeView.value = window.location.pathname === APP_PATH ? 'app' : 'landing'
}

function openApp() {
  if (window.location.pathname !== APP_PATH) {
    window.history.pushState({}, '', APP_PATH)
  }
  routeView.value = 'app'
}

function connectEvents(id: string) {
  cleanupEvents()
  source = new EventSource(eventsUrl(id))

  source.addEventListener('agent_event', (event) => {
    const payload = JSON.parse((event as MessageEvent).data) as EventPayload
    if (payload.status !== 'tool_event') {
      backendStatus.value = payload.status
      localStatus.value = null
    }
    stepIndex.value = mapStep(payload.message)
    eventLogs.value.push(`[${payload.timestamp}] ${payload.status}: ${payload.message}`)
  })

  source.onerror = () => {
    uiError.value = 'event stream connection lost; retry by resetting demo or reopening provisioning'
    eventLogs.value.push(`[${new Date().toISOString()}] error: SSE connection issue`)
  }
}

async function launchAtlas() {
  uiError.value = ''
  launchPending.value = true
  try {
    const agent = await api.createAgent('Atlas')
    agentId.value = agent.id
    localStorage.setItem(STORAGE_KEY, agent.id)
    screen.value = 'provisioning'
    provisionPending.value = true
    await api.provisionAgent(agent.id)
    connectEvents(agent.id)
  } catch (error) {
    uiError.value = error instanceof ClientError ? error.message : 'failed to launch atlas'
  } finally {
    launchPending.value = false
    provisionPending.value = false
  }
}

function openWorkspace() {
  clearReadyTransitionTimer()
  cleanupEvents()
  screen.value = 'workspace'
}

async function sendChat(message: string) {
  if (!agentId.value) return

  uiError.value = ''
  localStatus.value = 'working'
  messages.value.push({ role: 'user', content: message })
  chatPending.value = true

  try {
    const response = await api.chat(agentId.value, message)
    latestTool.value = response.tool_used
    messages.value.push({ role: 'assistant', content: response.agent_response, toolUsed: response.tool_used })
    localStatus.value = null
  } catch (error) {
    const text = error instanceof ClientError ? error.message : 'chat request failed'
    messages.value.push({ role: 'error', content: text })
    uiError.value = text
    localStatus.value = 'error'
  } finally {
    chatPending.value = false
    if (backendStatus.value === 'ready' && localStatus.value === 'working') {
      localStatus.value = null
    }
  }
}

function resetDemo() {
  clearReadyTransitionTimer()
  localStorage.removeItem(STORAGE_KEY)
  cleanupEvents()
  screen.value = 'signin'
  agentId.value = null
  backendStatus.value = 'requested'
  messages.value = []
  eventLogs.value = []
  stepIndex.value = 0
  latestTool.value = ''
  uiError.value = ''
  localStatus.value = null
  launchPending.value = false
  provisionPending.value = false
}

onMounted(async () => {
  syncRouteView()
  window.addEventListener('popstate', syncRouteView)

  if (routeView.value !== 'app') return

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

    backendStatus.value = found.status
    screen.value = found.status === 'ready' ? 'workspace' : 'provisioning'
    if (screen.value === 'provisioning') {
      connectEvents(saved)
    }
  } catch {
    resetDemo()
  }
})

watch([ready, screen], ([isReady, currentScreen]) => {
  if (!isReady || currentScreen !== 'provisioning') {
    clearReadyTransitionTimer()
    return
  }
  clearReadyTransitionTimer()
  readyTransitionTimer = setTimeout(() => {
    openWorkspace()
  }, 700)
})

onBeforeUnmount(() => {
  window.removeEventListener('popstate', syncRouteView)
  clearReadyTransitionTimer()
  cleanupEvents()
})
</script>

<template>
  <AppShell>
    <div class="app-frame">
      <SubmissionLandingScreen v-if="routeView === 'landing'" @open-demo="openApp" />
      <template v-else>
        <SignInScreen v-if="screen === 'signin'" :loading="launchPending" :error="uiError" @launch="launchAtlas" />
        <ProvisioningScreen
          v-else-if="screen === 'provisioning'"
          :current-step="stepIndex"
          :steps="steps"
          :ready="ready"
          :logs="eventLogs"
          :expanded="showLogs"
          :starting-provision="provisionPending"
          :error="uiError"
          @toggle-logs="showLogs = !showLogs"
        />
        <WorkspaceScreen
          v-else
          :status="displayStatus"
          :messages="messages"
          :latest-tool="latestTool"
          :runtime-collapsed="runtimeCollapsed"
          :pending="chatPending"
          :latest-events="latestEvents"
          :error="uiError"
          @send="sendChat"
          @toggle-runtime="runtimeCollapsed = !runtimeCollapsed"
          @reset="resetDemo"
        />
      </template>
    </div>
  </AppShell>
</template>

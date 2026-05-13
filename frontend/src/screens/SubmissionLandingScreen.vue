<script setup lang="ts">
import { onBeforeUnmount, onMounted, ref } from 'vue'

const emit = defineEmits<{
  (e: 'openDemo'): void
}>()

const architectureExpanded = ref(false)

function openArchitecture() {
  architectureExpanded.value = true
}

function closeArchitecture() {
  architectureExpanded.value = false
}

function onKeydown(event: KeyboardEvent) {
  if (event.key === 'Escape' && architectureExpanded.value) {
    closeArchitecture()
  }
}

onMounted(() => {
  window.addEventListener('keydown', onKeydown)
})

onBeforeUnmount(() => {
  window.removeEventListener('keydown', onKeydown)
})

const demonstrates = [
  {
    title: 'Full-stack product',
    text: 'Vue 3 frontend with a Rust/Axum API and an end-to-end agent workflow.'
  },
  {
    title: 'AWS deployment',
    text: 'EC2 + Security Groups + EBS + Docker Compose for single-host MVP operations.'
  },
  {
    title: 'Provisioning lifecycle',
    text: 'Event-driven backend stages streamed to UI over SSE during Atlas session startup.'
  },
  {
    title: 'Docker-backed runtime',
    text: 'Each provisioned Atlas agent receives an isolated runtime container + workspace scope.'
  },
  {
    title: 'Tool execution boundaries',
    text: 'File and shell tools run with scoped boundaries and a safety policy for blocked commands.'
  },
  {
    title: 'Cohesive product UX',
    text: 'Submission landing, launch/provisioning flow, and runtime workspace dashboard.'
  }
]

const checklist = [
  ['Original brand and product identity', 'Complete'],
  ['Distinct UI/UX', 'Complete'],
  ['AWS infrastructure', 'Complete'],
  ['Free-tier-oriented deployment', 'Complete'],
  ['Live deployment link', 'Complete'],
  ['Full-stack web application', 'Complete'],
  ['Agent-based interaction system', 'Strong MVP'],
  ['Agent-backed compute/runtime environment', 'Strong MVP'],
  ['Working account/authentication flow', 'Demo-level'],
  ['Real MCP integrations', 'Deferred'],
  ['Voice/SMS/Slack/Telegram channels', 'Deferred'],
  ['Production auth, billing, observability', 'Deferred']
]

const flowSteps = [
  'User clicks Launch Atlas',
  'Rust backend creates agent record',
  'Backend creates /workspaces/{agent_id}',
  'Backend starts nodepilot-runtime-{agent_id}',
  'Runtime mounts shared workspace volume',
  'Tools are registered',
  'User enters workspace/chat',
  'Run pwd executes inside /workspaces/{agent_id}'
]
</script>

<template>
  <section class="submission-page">
    <header class="submission-hero card rise">
      <div class="eyebrow mono txt-up"><span class="dot cool pulse"></span>Technical submission · Senior Platform Engineer</div>
      <h1>NodePilot</h1>
      <h2>An AWS-deployed, Docker-backed personal AI runtime MVP.</h2>
      <p>
        Built as an original take-home implementation for an AI agent platform concept. NodePilot lets a user
        launch Atlas, an agent backed by an isolated Docker runtime session, then interact with it through a web workspace.
      </p>
      <div class="submission-cta-row">
        <button class="cta" @click="emit('openDemo')">Open Live Demo</button>
        <a href="/api/health" class="sub-link">Health Check</a>
        <a href="https://github.com/LLIo6oH/nodepilot" target="_blank" rel="noreferrer" class="sub-link">GitHub Repository</a>
        <a href="#architecture" class="sub-link">Architecture</a>
      </div>
    </header>

    <section class="submission-grid-3">
      <article v-for="card in demonstrates" :key="card.title" class="card sub-card">
        <h3>{{ card.title }}</h3>
        <p>{{ card.text }}</p>
      </article>
    </section>

    <section class="card sub-section" id="checklist">
      <div class="sub-head">
        <h3>Requirement checklist</h3>
        <span class="mono txt-up">MVP scope status</span>
      </div>
      <div class="check-table">
        <div v-for="row in checklist" :key="row[0]" class="check-row">
          <span>{{ row[0] }}</span>
          <strong :class="['check-status', row[1].toLowerCase().replace(/\s+/g, '-')]">{{ row[1] }}</strong>
        </div>
      </div>
    </section>

    <section class="submission-split" id="architecture">
      <article class="card sub-section">
        <div class="sub-head">
          <h3>Architecture</h3>
          <span class="mono txt-up">Current MVP blueprint</span>
        </div>
        <button class="arch-button" @click="openArchitecture">
          <img src="/nodepilot-architecture.png" alt="NodePilot MVP architecture" class="arch-image" />
        </button>
        <div class="arch-actions">
          <button class="soft" @click="openArchitecture">View full architecture</button>
        </div>
        <p class="txt-mute">
          MVP architecture: Vue frontend, Rust API gateway, event-driven provisioning, Docker-backed runtime sessions,
          SQLite storage, and isolated workspaces on AWS EC2.
        </p>
        <p class="txt-mute">
          Runtime sessions are provisioned per agent as `nodepilot-runtime-{agent_id}` with workspace scope at
          `/workspaces/{agent_id}`.
        </p>
      </article>

      <article class="card sub-section flow-panel">
        <div class="sub-head">
          <h3>Runtime provisioning flow</h3>
          <span class="mono txt-up">Core technical path</span>
        </div>
        <ol class="flow-list">
          <li v-for="(step, idx) in flowSteps" :key="step">
            <span class="flow-index mono">{{ String(idx + 1).padStart(2, '0') }}</span>
            <p>{{ step }}</p>
          </li>
        </ol>
      </article>
    </section>

    <section class="submission-split">
      <article class="card sub-section">
        <div class="sub-head">
          <h3>What I intentionally left out</h3>
          <span class="mono txt-up">Deliberate MVP cuts</span>
        </div>
        <ul class="scope-list">
          <li>Production authentication/session management</li>
          <li>Real Gmail/Notion MCP integrations</li>
          <li>Voice activation/listening</li>
          <li>SMS/Slack/Telegram channels</li>
          <li>Dedicated EC2/ECS task per user</li>
          <li>Domain/TLS setup</li>
          <li>Billing/token analytics</li>
          <li>Full observability and cleanup jobs</li>
        </ul>
        <p class="txt-mute">
          I prioritized the core runtime architecture: cloud deployment, agent provisioning, isolated execution environment,
          tool boundaries, and product UX.
        </p>
      </article>

      <article class="card sub-section">
        <div class="sub-head">
          <h3>Production roadmap</h3>
          <span class="mono txt-up">Next milestones</span>
        </div>
        <ul class="scope-list">
          <li>Replace demo auth with secure session cookies/OAuth</li>
          <li>Move runtime orchestration to ECS/Fargate, Firecracker, or dedicated orchestrator</li>
          <li>Add MCP tool registry</li>
          <li>Add persistent memory/vector storage</li>
          <li>Add cleanup jobs and observability</li>
          <li>Add HTTPS/domain</li>
          <li>Add voice and messaging channels</li>
        </ul>
      </article>
    </section>

    <footer class="card submission-final">
      <h3>Ready to launch Atlas?</h3>
      <button class="cta" @click="emit('openDemo')">Open NodePilot Demo</button>
    </footer>

    <div
      v-if="architectureExpanded"
      class="arch-modal"
      role="dialog"
      aria-modal="true"
      aria-label="Full architecture diagram"
      @click.self="closeArchitecture"
    >
      <div class="arch-modal-card">
        <button class="arch-close" @click="closeArchitecture" aria-label="Close architecture view">×</button>
        <div class="arch-modal-body">
          <img src="/nodepilot-architecture.png" alt="NodePilot MVP architecture full view" class="arch-modal-image" />
        </div>
      </div>
    </div>
  </section>
</template>

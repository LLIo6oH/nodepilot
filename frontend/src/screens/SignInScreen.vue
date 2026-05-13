<script setup lang="ts">
import { ref } from 'vue'

const props = defineProps<{
  loading: boolean
  error: string
}>()

const emit = defineEmits<{
  (e: 'launch'): void
}>()

const mode = ref<'signup' | 'signin'>('signup')
</script>

<template>
  <section class="signin-layout">
    <div class="signin-hero card rise">
      <div class="hero-grid" aria-hidden="true"></div>
      <div class="brand-lockup">NodePilot</div>

      <div class="hero-copy">
        <div class="eyebrow mono txt-up">
          <span class="dot cool pulse"></span>
          Personal AI runtime · build 1.0
        </div>
        <h1>
          Pilot your own
          <span>AI runtime.</span>
        </h1>
        <p>
          NodePilot provisions a dedicated agent — Atlas — with its own cloud runtime, tools, and workspace.
          Talk to it, give it work, and watch it act.
        </p>

        <div class="capability-row">
          <div>
            <div class="mono txt-up">Private</div>
            <div>Sandboxed runtime</div>
          </div>
          <div>
            <div class="mono txt-up">Capable</div>
            <div>Files · shell · web</div>
          </div>
          <div>
            <div class="mono txt-up">Available</div>
            <div>Chat · voice · API</div>
          </div>
        </div>
      </div>

      <div class="hero-footer mono">NODEPILOT · cloud-native agent platform</div>
    </div>

    <div class="signin-auth card rise">
      <div class="status-corner mono">
        <span class="dot good"></span>
        us-west-2 · all systems nominal
      </div>

      <form class="auth-inner" @submit.prevent="emit('launch')">
        <div class="auth-toggle" role="tablist" aria-label="auth mode">
          <button type="button" :class="{ active: mode === 'signup' }" @click="mode = 'signup'">Create account</button>
          <button type="button" :class="{ active: mode === 'signin' }" @click="mode = 'signin'">Sign in</button>
        </div>

        <h2>{{ mode === 'signup' ? 'Create your NodePilot' : 'Welcome back' }}</h2>
        <p class="txt-mute">
          {{ mode === 'signup' ? 'Atlas will be provisioned the moment you finish.' : 'Atlas remembers everything. Pick up where you left off.' }}
        </p>

        <div class="social-actions">
          <button type="button" class="soft" :disabled="props.loading" @click="emit('launch')">Continue with Google</button>
          <button type="button" class="soft" :disabled="props.loading" @click="emit('launch')">Continue with GitHub</button>
        </div>

        <div class="divider mono txt-up">or with email</div>

        <label class="field">
          <span class="mono txt-up">Email</span>
          <input placeholder="you@domain.com" />
        </label>

        <label class="field">
          <span class="mono txt-up">Password</span>
          <input placeholder="••••••••••" type="password" />
        </label>

        <p v-if="props.error" class="ui-error">{{ props.error }}</p>

        <button type="submit" class="cta" :disabled="props.loading">
          {{ props.loading ? 'Launching Atlas...' : mode === 'signup' ? 'Create account & launch Atlas' : 'Sign in' }}
        </button>

        <p class="terms">
          By continuing, you agree to NodePilot terms and privacy policy. A dedicated runtime will be allocated to your account.
        </p>
      </form>
    </div>
  </section>
</template>

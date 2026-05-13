<script setup lang="ts">
const props = defineProps<{
  loading: boolean
  error: string
}>()

const emit = defineEmits<{
  (e: 'launch'): void
}>()
</script>

<template>
  <section class="signin-screen card">
    <div class="brand-line">NodePilot</div>
    <h1>Pilot your own AI runtime.</h1>
    <p>
      NodePilot provisions a dedicated agent — Atlas — with its own runtime, tools, and workspace.
    </p>

    <div class="social-actions">
      <button>Continue with Google</button>
      <button>Continue with GitHub</button>
    </div>

    <div class="form-grid">
      <input placeholder="Email" />
      <input placeholder="Password" type="password" />
    </div>

    <p v-if="props.error" class="ui-error">{{ props.error }}</p>
    <button class="cta" :disabled="props.loading" @click="emit('launch')">
      {{ props.loading ? 'Launching Atlas...' : 'Create account & launch Atlas' }}
    </button>
  </section>
</template>

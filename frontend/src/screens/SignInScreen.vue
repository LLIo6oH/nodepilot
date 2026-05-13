<script setup lang="ts">
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

    <button class="cta" @click="emit('launch')">Create account &amp; launch Atlas</button>
  </section>
</template>

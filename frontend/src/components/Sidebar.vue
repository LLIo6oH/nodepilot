<script setup lang="ts">
defineProps<{
  status: string
}>()

const chats = ['Roadmap planning', 'Shell checks', 'Email draft']
</script>

<template>
  <aside class="sidebar card">
    <div>
      <div class="brand">NodePilot</div>
      <div class="sub">Private AI Runtime</div>
    </div>
    <div class="status">
      <span class="dot" :class="status === 'ready' ? 'good' : 'warm pulse'"></span>
      <span>Atlas {{ status }}</span>
    </div>
    <div>
      <div class="section-title">Recent Threads</div>
      <ul class="recent">
        <li v-for="chat in chats" :key="chat">{{ chat }}</li>
      </ul>
    </div>
  </aside>
</template>

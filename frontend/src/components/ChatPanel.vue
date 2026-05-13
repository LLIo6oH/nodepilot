<script setup lang="ts">
import { ref } from 'vue'

type Message = {
  role: 'user' | 'assistant' | 'error'
  content: string
  toolUsed?: string
}

const props = defineProps<{
  messages: Message[]
  pending: boolean
}>()

const emit = defineEmits<{
  (e: 'send', value: string): void
}>()

const input = ref('')
const suggestions = [
  'Create roadmap.txt',
  'Read roadmap.txt',
  'Write follow-up email',
  'Run pwd',
  'Run rm -rf /'
]

function submit() {
  const value = input.value.trim()
  if (!value) return
  emit('send', value)
  input.value = ''
}

function useSuggestion(value: string) {
  input.value = value
}
</script>

<template>
  <section class="chat-panel card">
    <header class="chat-header">
      <div>
        <h2>Atlas</h2>
        <p>Dedicated runtime channel</p>
      </div>
    </header>

    <div class="chat-body">
      <div v-if="messages.length === 0" class="empty">
        <h3>Atlas is ready when you are.</h3>
        <p>
          Ask anything. Atlas can read and write files, run safe shell commands in its sandbox, and
          draft emails.
        </p>
        <div class="suggestions">
          <button v-for="s in suggestions" :key="s" @click="useSuggestion(s)">{{ s }}</button>
        </div>
      </div>
      <div v-else class="messages">
        <article v-for="(message, idx) in props.messages" :key="idx" :class="['msg', message.role]">
          <div class="role">{{ message.role }}</div>
          <div>{{ message.content }}</div>
          <small v-if="message.toolUsed">tool: {{ message.toolUsed }}</small>
        </article>
      </div>
    </div>

    <footer class="chat-input-wrap">
      <input v-model="input" placeholder="Ask Atlas to run a tool..." @keydown.enter="submit" />
      <button :disabled="pending" @click="submit">{{ pending ? 'Sending...' : 'Send' }}</button>
    </footer>
  </section>
</template>

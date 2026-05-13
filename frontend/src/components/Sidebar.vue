<script setup lang="ts">
defineProps<{
  status: string
}>()

const agents = [
  { name: 'Atlas', hint: 'Personal · primary', active: true },
  { name: 'Scout', hint: 'Research · paused', active: false }
]

const chats = ['Investor email · draft v3', 'Roadmap restructure', 'Standup notes Q2', 'Refactor billing module']
</script>

<template>
  <aside class="sidebar card">
    <div class="sidebar-head">
      <div class="brand-lockup">NodePilot</div>
      <button class="icon-btn">⚙</button>
    </div>

    <button class="search-btn mono">Quick find <span>⌘K</span></button>

    <div class="sidebar-section">
      <div class="section-title">Agents</div>
      <div class="agent-list">
        <button v-for="agent in agents" :key="agent.name" :class="['agent-row', { active: agent.active }]">
          <div class="agent-glyph">◈</div>
          <div>
            <div class="agent-title">{{ agent.name }} <span class="dot" :class="agent.active ? (status === 'ready' ? 'good' : 'warm pulse') : 'idle'"></span></div>
            <div class="mono agent-sub">{{ agent.hint }}</div>
          </div>
        </button>
      </div>
    </div>

    <div class="sidebar-section">
      <div class="section-title">Recent chats</div>
      <div class="chat-list">
        <button v-for="(chat, idx) in chats" :key="chat" :class="['chat-row', { active: idx === 0 }]">{{ chat }}</button>
      </div>
    </div>

    <div class="sidebar-footer card">
      <div class="avatar">JK</div>
      <div>
        <div class="agent-title">Jamie Kessler</div>
        <div class="mono agent-sub">pro · 8h21m uptime</div>
      </div>
    </div>
  </aside>
</template>

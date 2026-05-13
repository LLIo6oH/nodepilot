<script setup lang="ts">
defineProps<{
  status: string
  latestTool: string
  collapsed: boolean
  latestEvents: string[]
}>()
</script>

<template>
  <aside v-if="!collapsed" class="runtime-panel card mono">
    <h3>Runtime</h3>
    <p>Status: {{ status }}</p>
    <p>Region: us-east-1 (MVP)</p>
    <p>Workspace: workspaces/&lt;agent_id&gt;/</p>
    <p>Latest tool: {{ latestTool || 'none' }}</p>
    <div>
      <div class="section-title">Enabled tools</div>
      <ul>
        <li>file.write</li>
        <li>file.read</li>
        <li>email.compose</li>
        <li>shell.run_limited</li>
      </ul>
    </div>
    <div class="event-mini">
      <div class="section-title">Activity</div>
      <ul>
        <li v-for="(event, index) in latestEvents" :key="`${index}-${event}`">{{ event }}</li>
      </ul>
    </div>
  </aside>
</template>

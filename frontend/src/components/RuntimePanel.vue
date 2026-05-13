<script setup lang="ts">
defineProps<{
  status: string
  latestTool: string
  collapsed: boolean
  latestEvents: string[]
}>()
</script>

<template>
  <aside v-if="!collapsed" class="runtime-panel">
    <div class="card runtime-card">
      <div class="panel-head">
        <span>Atlas runtime</span>
        <span class="status-pill"><span class="dot" :class="status === 'ready' ? 'good' : 'warm pulse'"></span>{{ status }}</span>
      </div>
      <div class="runtime-grid mono">
        <div><span>UPTIME</span><strong>8h 21m</strong></div>
        <div><span>RUNTIME</span><strong>np-1.4.2</strong></div>
        <div><span>REGION</span><strong>us-east-1</strong></div>
        <div><span>LATENCY</span><strong>38 ms</strong></div>
        <div><span>WORKSPACE</span><strong>/workspace</strong></div>
        <div><span>SANDBOX</span><strong>enabled</strong></div>
      </div>
    </div>

    <div class="card runtime-card">
      <div class="panel-head"><span>Current task</span><span class="mono">active</span></div>
      <div class="task-pill"><span class="dot warm pulse"></span>Drafting workflow response</div>
    </div>

    <div class="card runtime-card">
      <div class="panel-head"><span>Connection</span><span class="mono">stable</span></div>
      <ul class="conn-list">
        <li><span><span class="dot good"></span>Control plane</span><small class="mono">wss · 38 ms</small></li>
        <li><span><span class="dot good"></span>Memory store</span><small class="mono">vector · 12 ms</small></li>
        <li><span><span class="dot good"></span>Tool gateway</span><small class="mono">6 active</small></li>
      </ul>
    </div>

    <div class="card runtime-card">
      <div class="panel-head"><span>Activity</span><span class="mono">latest</span></div>
      <ul class="event-list mono">
        <li v-for="(event, index) in latestEvents" :key="`${index}-${event}`">{{ event }}</li>
      </ul>
      <p class="mono latest-tool">Latest tool: {{ latestTool || 'none' }}</p>
    </div>
  </aside>
</template>

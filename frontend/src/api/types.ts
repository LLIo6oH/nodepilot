export type ApiError = {
  error: string
}

export type Agent = {
  id: string
  user_id: string
  name: string
  status: 'requested' | 'launching' | 'booting' | 'configuring' | 'ready'
  runtime_kind: string
  created_at: string
  updated_at: string
}

export type ChatResponse = {
  agent_id: string
  user_message: string
  agent_response: string
  tool_used: string
  timestamp: string
}

export type EventPayload = {
  agent_id: string
  status: string
  message: string
  timestamp: string
}

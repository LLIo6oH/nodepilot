import type { Agent, ApiError, ChatResponse } from './types'

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL ?? 'http://localhost:8080'

export class ClientError extends Error {
  status: number

  constructor(message: string, status: number) {
    super(message)
    this.status = status
  }
}

async function request<T>(path: string, init?: RequestInit): Promise<T> {
  const response = await fetch(`${API_BASE_URL}${path}`, {
    headers: {
      'Content-Type': 'application/json',
      ...(init?.headers ?? {})
    },
    ...init
  })

  const text = await response.text()
  const data = text ? JSON.parse(text) : null

  if (!response.ok) {
    const errorMessage = (data as ApiError | null)?.error ?? `request failed with status ${response.status}`
    throw new ClientError(errorMessage, response.status)
  }

  return data as T
}

export const api = {
  createAgent(name: string) {
    return request<Agent>('/agents', {
      method: 'POST',
      body: JSON.stringify({ name })
    })
  },
  listAgents() {
    return request<Agent[]>('/agents')
  },
  provisionAgent(id: string) {
    return request<null>(`/agents/${id}/provision`, { method: 'POST' })
  },
  chat(id: string, message: string) {
    return request<ChatResponse>(`/agents/${id}/chat`, {
      method: 'POST',
      body: JSON.stringify({ message })
    })
  }
}

export function eventsUrl(agentId: string): string {
  return `${API_BASE_URL}/agents/${agentId}/events`
}

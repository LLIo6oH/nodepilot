#![allow(dead_code)]

use sqlx::SqlitePool;

use crate::models::{Agent, AgentEvent, Message, User};

pub async fn create_user(pool: &SqlitePool, user: &User) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO users (id, email, password_hash, created_at)
        VALUES (?1, ?2, ?3, ?4)
        "#,
    )
    .bind(user.id.to_string())
    .bind(&user.email)
    .bind(&user.password_hash)
    .bind(&user.created_at)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn create_agent(pool: &SqlitePool, agent: &Agent) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO agents (id, user_id, name, status, runtime_kind, created_at, updated_at)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
        "#,
    )
    .bind(agent.id.to_string())
    .bind(agent.user_id.to_string())
    .bind(&agent.name)
    .bind(agent.status.as_str())
    .bind(&agent.runtime_kind)
    .bind(&agent.created_at)
    .bind(&agent.updated_at)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn create_message(pool: &SqlitePool, message: &Message) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO messages (id, agent_id, role, content, created_at)
        VALUES (?1, ?2, ?3, ?4, ?5)
        "#,
    )
    .bind(message.id.to_string())
    .bind(message.agent_id.to_string())
    .bind(&message.role)
    .bind(&message.content)
    .bind(&message.created_at)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn create_agent_event(pool: &SqlitePool, event: &AgentEvent) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO agent_events (id, agent_id, event_type, message, created_at)
        VALUES (?1, ?2, ?3, ?4, ?5)
        "#,
    )
    .bind(event.id.to_string())
    .bind(event.agent_id.to_string())
    .bind(&event.event_type)
    .bind(&event.message)
    .bind(&event.created_at)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn ping(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query("SELECT 1").execute(pool).await?;
    Ok(())
}

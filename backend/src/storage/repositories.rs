use chrono::Utc;
use sqlx::{Row, SqlitePool};
use uuid::Uuid;

use crate::models::{Agent, AgentEvent, AgentStatus, Message, User};

#[allow(dead_code)]
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

pub async fn list_agents(pool: &SqlitePool) -> Result<Vec<Agent>, sqlx::Error> {
    let rows = sqlx::query(
        r#"
        SELECT id, user_id, name, status, runtime_kind, created_at, updated_at
        FROM agents
        ORDER BY created_at ASC
        "#,
    )
    .fetch_all(pool)
    .await?;

    let agents = rows
        .into_iter()
        .filter_map(|row| row_to_agent(&row))
        .collect();

    Ok(agents)
}

pub async fn get_agent(pool: &SqlitePool, agent_id: Uuid) -> Result<Option<Agent>, sqlx::Error> {
    let row = sqlx::query(
        r#"
        SELECT id, user_id, name, status, runtime_kind, created_at, updated_at
        FROM agents
        WHERE id = ?1
        LIMIT 1
        "#,
    )
    .bind(agent_id.to_string())
    .fetch_optional(pool)
    .await?;

    Ok(row.and_then(|value| row_to_agent(&value)))
}

pub async fn update_agent_status(
    pool: &SqlitePool,
    agent_id: Uuid,
    status: AgentStatus,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE agents
        SET status = ?1, updated_at = ?2
        WHERE id = ?3
        "#,
    )
    .bind(status.as_str())
    .bind(Utc::now().to_rfc3339())
    .bind(agent_id.to_string())
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

fn row_to_agent(row: &sqlx::sqlite::SqliteRow) -> Option<Agent> {
    let id = Uuid::parse_str(row.get::<&str, _>("id")).ok()?;
    let user_id = Uuid::parse_str(row.get::<&str, _>("user_id")).ok()?;
    let status = AgentStatus::from_str(row.get::<&str, _>("status"))?;

    Some(Agent {
        id,
        user_id,
        name: row.get("name"),
        status,
        runtime_kind: row.get("runtime_kind"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}

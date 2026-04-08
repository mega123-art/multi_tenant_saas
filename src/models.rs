use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use uuid::Uuid;

//TENANT

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Tenant {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub created_at: Option<DateTime<Utc>>,
}

//USER

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub email: String,
    pub role: String,
    pub created_at: Option<DateTime<Utc>>,
}

//PROJECT

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Project {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub status: String,
    pub created_at: Option<DateTime<Utc>>,
}

//
// ===== TASK =====
//

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Task {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub project_id: Uuid,
    pub parent_task_id: Option<Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub priority: String,
    pub metadata: Value,
    pub version: i32,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

//JOB

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Job {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub job_type: String,
    pub payload: Value,
    pub status: String,
    pub attempts: i32,
    pub max_attempts: i32,
    pub error_message: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

// For recursive CTE results
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct TaskTreeRow {
    pub id: uuid::Uuid,
    pub title: String,
    pub status: String,
    pub priority: String,
    pub parent_task_id: Option<uuid::Uuid>,
    pub depth: i32,
}
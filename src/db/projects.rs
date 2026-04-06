use sqlx::PgPool;
use uuid::Uuid;
use futures_util::FutureExt;

use crate::models::Project;
use crate::errors::ApiError;
use crate::db::utils::with_tenant;


//CREATE PROJECT 


pub async fn create_project(
    pool: &PgPool,
    tenant_id: Uuid,
    name: String,
    description: Option<String>,
) -> Result<Project, ApiError> {
    let project = with_tenant(pool, tenant_id, |tx| async move {
        let project = sqlx::query_as!(
            Project,
            r#"
            INSERT INTO projects (tenant_id, name, description)
            VALUES ($1, $2, $3)
            RETURNING id, tenant_id, name, description, status, created_at
            "#,
            tenant_id,
            name,
            description
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(project)
    }.boxed())
    .await?;

    Ok(project)
}


//LIST PROJECTS 


pub async fn list_projects(
    pool: &PgPool,
    tenant_id: Uuid,
) -> Result<Vec<Project>, ApiError> {
    let projects = with_tenant(pool, tenant_id, |tx| async move {
        let projects = sqlx::query_as!(
            Project,
            r#"
            SELECT id, tenant_id, name, description, status, created_at
            FROM projects
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&mut **tx)
        .await?;

        Ok(projects)
    }.boxed())
    .await?;

    Ok(projects)
}

//
// ===== GET PROJECT =====
//

pub async fn get_project(
    pool: &PgPool,
    tenant_id: Uuid,
    project_id: Uuid,
) -> Result<Project, ApiError> {
    let project = with_tenant(pool, tenant_id, |tx| async move {
        let project = sqlx::query_as!(
            Project,
            r#"
            SELECT id, tenant_id, name, description, status, created_at
            FROM projects
            WHERE id = $1
            "#,
            project_id
        )
        .fetch_optional(&mut **tx)
        .await?;

        match project {
            Some(p) => Ok(p),
            None => Err(ApiError::NotFound),
        }
    }.boxed())
    .await?;

    Ok(project)
}


//DELETE PROJECT

pub async fn delete_project(
    pool: &PgPool,
    tenant_id: Uuid,
    project_id: Uuid,
) -> Result<(), ApiError> {
    with_tenant(pool, tenant_id, |tx| async move {
        let res = sqlx::query!(
            "DELETE FROM projects WHERE id = $1",
            project_id
        )
        .execute(&mut **tx)
        .await?;

        if res.rows_affected() == 0 {
            return Err(ApiError::NotFound);
        }

        Ok(())
    }.boxed())
    .await?;

    Ok(())
}
use crate::{
    error::AppError,
    models::{
        CreateDocumentRequest, Document, DocumentPermission, DocumentType, PermissionType,
        UpdateDocumentRequest,
    },
};
use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn create_document(
    pool: &SqlitePool,
    doc: CreateDocumentRequest,
) -> Result<Document, AppError> {
    let document = sqlx::query_as!(
        Document,
        r#"
            INSERT INTO documents
                (id, title, content, doc_type, user_id, is_active, metadata)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            RETURNING id as "id: Uuid", title, content, user_id as "user_id: Uuid", is_active, doc_type as "doc_type: String", metadata as "metadata: Value", created_at as "created_at: DateTime<Utc>", updated_at as "updated_at: DateTime<Utc>"
        "#,
        doc.id,
        doc.title,
        doc.content,
        doc.doc_type,
        doc.user_id,
        doc.is_active,
        doc.metadata
    )
    .fetch_one(pool)
    .await?;

    Ok(document)
}

pub async fn update_document(
    pool: &SqlitePool,
    id: Uuid,
    doc: UpdateDocumentRequest,
) -> Result<(), AppError> {
    sqlx::query!(
        r#"UPDATE documents SET 
            title = COALESCE(?, title),
            content = COALESCE(?, content),
            is_active = COALESCE(?, is_active),
            doc_type = COALESCE(?, doc_type),
            metadata = COALESCE(?, metadata),
            updated_at = CURRENT_TIMESTAMP
        WHERE id = ?"#,
        doc.title,
        doc.content,
        doc.is_active,
        doc.doc_type,
        doc.metadata,
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_document_with_permission(
    pool: &SqlitePool,
    id: Uuid,
    user_id: Uuid,
) -> Result<(Document, Option<String>), AppError> {
    let result = sqlx::query!(
        r#"SELECT
            d.id as "id: Uuid", d.title, d.content, d.is_active, d.doc_type as "doc_type: String", d.metadata as "metadata: Value", d.user_id as "user_id: Uuid", d.created_at as "created_at: DateTime<Utc>", d.updated_at as "updated_at: DateTime<Utc>", dp.permission_type as "permission_type: String"
        FROM documents d
        LEFT JOIN document_permissions dp ON d.id = dp.document_id AND dp.user_id = d.user_id
        WHERE d.id = ? AND (d.user_id = ? OR dp.id IS NOT NULL)
        "#,
        id,
        user_id
    )
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Document not found or no permission".to_string()))?;

    let document = Document {
        id: result.id,
        user_id: result.user_id,
        title: result.title,
        content: result.content,
        is_active: result.is_active,
        doc_type: DocumentType::from(result.doc_type),
        metadata: result.metadata,
        created_at: result.created_at,
        updated_at: result.updated_at,
    };

    Ok((document, result.permission_type))
}

pub async fn add_document_permission(
    pool: &SqlitePool,
    document_id: Uuid,
    user_id: Uuid,
    permission_type: PermissionType,
    parameters: Option<Value>,
) -> Result<DocumentPermission, AppError> {
    let id = Uuid::new_v4();
    let permission_type_str = permission_type.to_string();
    let parameters_str = parameters.map(|p| serde_json::to_string(&p).unwrap());
    let permission = sqlx::query_as!(
        DocumentPermission,
        r#"
        INSERT INTO document_permissions (id, document_id, user_id, permission_type, parameters)
        VALUES (?, ?, ?, ?, ?)
        RETURNING id as "id: Uuid", document_id as "document_id: Uuid", user_id as "user_id: Uuid", permission_type as "permission_type: String", parameters as "parameters: Value", created_at as "created_at: DateTime<Utc>", updated_at as "updated_at: DateTime<Utc>"
        "#,
        id,
        document_id,
        user_id,
        permission_type_str,
        parameters_str
    )
    .fetch_one(pool)
    .await?;

    Ok(permission)
}

pub async fn delete_document(pool: &SqlitePool, id: Uuid) -> Result<(), AppError> {
    sqlx::query!("DELETE FROM documents WHERE id = ?", id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn list_documents(pool: &SqlitePool) -> Result<Vec<Document>, AppError> {
    let documents = sqlx::query_as!(
        Document,
        r#"SELECT id as "id: Uuid", title, content, user_id as "user_id: Uuid", is_active, doc_type as "doc_type: String", metadata as "metadata: Value", created_at as "created_at: DateTime<Utc>", updated_at as "updated_at: DateTime<Utc>" FROM documents"#
    )
    .fetch_all(pool)
    .await?;
    Ok(documents)
}

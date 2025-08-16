use sqlx::{SqlitePool, Result};
use uuid::Uuid;
use crate::models::Document;

pub async fn insert_document(pool: &SqlitePool, document: Document) -> Result<Document> {
    let row = sqlx::query!(
        r#"
        INSERT INTO documents (
            id, filename, content_type, file_size, original_text, extracted_text,
            document_type, language, client_id, case_reference, processing_status,
            error_message, created_at, updated_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
        RETURNING *
        "#,
        document.id,
        document.filename,
        document.content_type,
        document.file_size,
        document.original_text,
        document.extracted_text,
        document.document_type,
        document.language,
        document.client_id,
        document.case_reference,
        document.processing_status,
        document.error_message,
        document.created_at,
        document.updated_at
    )
    .fetch_one(pool)
    .await?;

    Ok(Document {
        id: row.id,
        filename: row.filename,
        content_type: row.content_type,
        file_size: row.file_size,
        original_text: row.original_text,
        extracted_text: row.extracted_text,
        document_type: row.document_type,
        language: row.language,
        client_id: row.client_id,
        case_reference: row.case_reference,
        processing_status: row.processing_status,
        error_message: row.error_message,
        created_at: row.created_at,
        updated_at: row.updated_at,
    })
}

pub async fn get_document_by_id(pool: &SqlitePool, id: Uuid) -> Result<Option<Document>> {
    let row = sqlx::query!(
        "SELECT * FROM documents WHERE id = $1",
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| Document {
        id: r.id,
        filename: r.filename,
        content_type: r.content_type,
        file_size: r.file_size,
        original_text: r.original_text,
        extracted_text: r.extracted_text,
        document_type: r.document_type,
        language: r.language,
        client_id: r.client_id,
        case_reference: r.case_reference,
        processing_status: r.processing_status,
        error_message: r.error_message,
        created_at: r.created_at,
        updated_at: r.updated_at,
    }))
}

pub async fn get_documents(
    pool: &SqlitePool, 
    limit: Option<i32>, 
    offset: Option<i32>, 
    status: Option<&str>
) -> Result<Vec<Document>> {
    let limit = limit.unwrap_or(50);
    let offset = offset.unwrap_or(0);

    let rows = if let Some(status_filter) = status {
        sqlx::query!(
            "SELECT * FROM documents WHERE processing_status = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3",
            status_filter,
            limit as i64,
            offset as i64
        )
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query!(
            "SELECT * FROM documents ORDER BY created_at DESC LIMIT $1 OFFSET $2",
            limit as i64,
            offset as i64
        )
        .fetch_all(pool)
        .await?
    };

    Ok(rows.into_iter().map(|r| Document {
        id: r.id,
        filename: r.filename,
        content_type: r.content_type,
        file_size: r.file_size,
        original_text: r.original_text,
        extracted_text: r.extracted_text,
        document_type: r.document_type,
        language: r.language,
        client_id: r.client_id,
        case_reference: r.case_reference,
        processing_status: r.processing_status,
        error_message: r.error_message,
        created_at: r.created_at,
        updated_at: r.updated_at,
    }).collect())
}

pub async fn update_document_status(pool: &SqlitePool, id: Uuid, status: &str) -> Result<()> {
    sqlx::query!(
        "UPDATE documents SET processing_status = $1, updated_at = datetime('now') WHERE id = $2",
        status,
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn update_document_text(pool: &SqlitePool, id: Uuid, extracted_text: &str) -> Result<()> {
    sqlx::query!(
        "UPDATE documents SET extracted_text = $1, updated_at = datetime('now') WHERE id = $2",
        extracted_text,
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_documents_by_client(pool: &SqlitePool, client_id: &str) -> Result<Vec<Document>> {
    let rows = sqlx::query!(
        "SELECT * FROM documents WHERE client_id = $1 ORDER BY created_at DESC",
        client_id
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| Document {
        id: r.id,
        filename: r.filename,
        content_type: r.content_type,
        file_size: r.file_size,
        original_text: r.original_text,
        extracted_text: r.extracted_text,
        document_type: r.document_type,
        language: r.language,
        client_id: r.client_id,
        case_reference: r.case_reference,
        processing_status: r.processing_status,
        error_message: r.error_message,
        created_at: r.created_at,
        updated_at: r.updated_at,
    }).collect())
}

pub async fn get_documents_by_type(pool: &SqlitePool, document_type: &str) -> Result<Vec<Document>> {
    let rows = sqlx::query!(
        "SELECT * FROM documents WHERE document_type = $1 ORDER BY created_at DESC",
        document_type
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| Document {
        id: r.id,
        filename: r.filename,
        content_type: r.content_type,
        file_size: r.file_size,
        original_text: r.original_text,
        extracted_text: r.extracted_text,
        document_type: r.document_type,
        language: r.language,
        client_id: r.client_id,
        case_reference: r.case_reference,
        processing_status: r.processing_status,
        error_message: r.error_message,
        created_at: r.created_at,
        updated_at: r.updated_at,
    }).collect())
}
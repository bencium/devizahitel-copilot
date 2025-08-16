use sqlx::{PgPool, Result};
use uuid::Uuid;
use crate::models::{ExtractedClause, ClausePattern};

pub async fn insert_extracted_clause(pool: &PgPool, clause: ExtractedClause) -> Result<ExtractedClause> {
    let row = sqlx::query!(
        r#"
        INSERT INTO extracted_clauses (
            id, document_id, clause_type, clause_text, original_language,
            english_translation, start_position, end_position, confidence_score,
            risk_level, created_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        RETURNING *
        "#,
        clause.id,
        clause.document_id,
        clause.clause_type,
        clause.clause_text,
        clause.original_language,
        clause.english_translation,
        clause.start_position,
        clause.end_position,
        clause.confidence_score,
        clause.risk_level,
        clause.created_at
    )
    .fetch_one(pool)
    .await?;

    Ok(ExtractedClause {
        id: row.id,
        document_id: row.document_id,
        clause_type: row.clause_type,
        clause_text: row.clause_text,
        original_language: row.original_language,
        english_translation: row.english_translation,
        start_position: row.start_position,
        end_position: row.end_position,
        confidence_score: row.confidence_score,
        risk_level: row.risk_level,
        created_at: row.created_at,
    })
}

pub async fn get_clauses_by_document(pool: &PgPool, document_id: Uuid) -> Result<Vec<ExtractedClause>> {
    let rows = sqlx::query!(
        "SELECT * FROM extracted_clauses WHERE document_id = $1 ORDER BY start_position",
        document_id
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| ExtractedClause {
        id: r.id,
        document_id: r.document_id,
        clause_type: r.clause_type,
        clause_text: r.clause_text,
        original_language: r.original_language,
        english_translation: r.english_translation,
        start_position: r.start_position,
        end_position: r.end_position,
        confidence_score: r.confidence_score,
        risk_level: r.risk_level,
        created_at: r.created_at,
    }).collect())
}

pub async fn get_clauses_by_type(pool: &PgPool, clause_type: &str) -> Result<Vec<ExtractedClause>> {
    let rows = sqlx::query!(
        "SELECT * FROM extracted_clauses WHERE clause_type = $1 ORDER BY confidence_score DESC",
        clause_type
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| ExtractedClause {
        id: r.id,
        document_id: r.document_id,
        clause_type: r.clause_type,
        clause_text: r.clause_text,
        original_language: r.original_language,
        english_translation: r.english_translation,
        start_position: r.start_position,
        end_position: r.end_position,
        confidence_score: r.confidence_score,
        risk_level: r.risk_level,
        created_at: r.created_at,
    }).collect())
}

pub async fn get_high_risk_clauses(pool: &PgPool) -> Result<Vec<ExtractedClause>> {
    let rows = sqlx::query!(
        "SELECT * FROM extracted_clauses WHERE risk_level IN ('high', 'critical') ORDER BY confidence_score DESC"
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| ExtractedClause {
        id: r.id,
        document_id: r.document_id,
        clause_type: r.clause_type,
        clause_text: r.clause_text,
        original_language: r.original_language,
        english_translation: r.english_translation,
        start_position: r.start_position,
        end_position: r.end_position,
        confidence_score: r.confidence_score,
        risk_level: r.risk_level,
        created_at: r.created_at,
    }).collect())
}

pub async fn insert_clause_pattern(pool: &PgPool, pattern: ClausePattern) -> Result<ClausePattern> {
    let row = sqlx::query!(
        r#"
        INSERT INTO clause_patterns (
            id, name, pattern_type, pattern_text, language, clause_category,
            severity, description, legal_basis, is_active, created_at, updated_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        RETURNING *
        "#,
        pattern.id,
        pattern.name,
        pattern.pattern_type,
        pattern.pattern_text,
        pattern.language,
        pattern.clause_category,
        pattern.severity,
        pattern.description,
        pattern.legal_basis,
        pattern.is_active,
        pattern.created_at,
        pattern.updated_at
    )
    .fetch_one(pool)
    .await?;

    Ok(ClausePattern {
        id: row.id,
        name: row.name,
        pattern_type: row.pattern_type,
        pattern_text: row.pattern_text,
        language: row.language,
        clause_category: row.clause_category,
        severity: row.severity,
        description: row.description,
        legal_basis: row.legal_basis,
        is_active: row.is_active,
        created_at: row.created_at,
        updated_at: row.updated_at,
    })
}

pub async fn get_active_patterns(pool: &PgPool) -> Result<Vec<ClausePattern>> {
    let rows = sqlx::query!(
        "SELECT * FROM clause_patterns WHERE is_active = true ORDER BY clause_category, severity DESC"
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| ClausePattern {
        id: r.id,
        name: r.name,
        pattern_type: r.pattern_type,
        pattern_text: r.pattern_text,
        language: r.language,
        clause_category: r.clause_category,
        severity: r.severity,
        description: r.description,
        legal_basis: r.legal_basis,
        is_active: r.is_active,
        created_at: r.created_at,
        updated_at: r.updated_at,
    }).collect())
}

pub async fn search_clauses(pool: &PgPool, search_term: &str) -> Result<Vec<ExtractedClause>> {
    let rows = sqlx::query!(
        r#"
        SELECT * FROM extracted_clauses 
        WHERE clause_text ILIKE $1 
           OR english_translation ILIKE $1
        ORDER BY confidence_score DESC
        LIMIT 100
        "#,
        format!("%{}%", search_term)
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| ExtractedClause {
        id: r.id,
        document_id: r.document_id,
        clause_type: r.clause_type,
        clause_text: r.clause_text,
        original_language: r.original_language,
        english_translation: r.english_translation,
        start_position: r.start_position,
        end_position: r.end_position,
        confidence_score: r.confidence_score,
        risk_level: r.risk_level,
        created_at: r.created_at,
    }).collect())
}

pub async fn update_clause_translation(pool: &PgPool, clause_id: Uuid, translation: &str) -> Result<()> {
    sqlx::query!(
        "UPDATE extracted_clauses SET english_translation = $1 WHERE id = $2",
        translation,
        clause_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_clause_statistics(pool: &PgPool) -> Result<ClauseStatistics> {
    let stats = sqlx::query!(
        r#"
        SELECT 
            COUNT(*) as total_clauses,
            COUNT(CASE WHEN clause_type = 'fx_risk' THEN 1 END) as fx_risk_count,
            COUNT(CASE WHEN clause_type = 'transparency' THEN 1 END) as transparency_count,
            COUNT(CASE WHEN risk_level = 'critical' THEN 1 END) as critical_count,
            COUNT(CASE WHEN risk_level = 'high' THEN 1 END) as high_risk_count,
            AVG(confidence_score) as avg_confidence
        FROM extracted_clauses
        "#
    )
    .fetch_one(pool)
    .await?;

    Ok(ClauseStatistics {
        total_clauses: stats.total_clauses.unwrap_or(0) as i32,
        fx_risk_count: stats.fx_risk_count.unwrap_or(0) as i32,
        transparency_count: stats.transparency_count.unwrap_or(0) as i32,
        critical_count: stats.critical_count.unwrap_or(0) as i32,
        high_risk_count: stats.high_risk_count.unwrap_or(0) as i32,
        avg_confidence: stats.avg_confidence.unwrap_or(0.0) as f32,
    })
}

#[derive(Debug, serde::Serialize)]
pub struct ClauseStatistics {
    pub total_clauses: i32,
    pub fx_risk_count: i32,
    pub transparency_count: i32,
    pub critical_count: i32,
    pub high_risk_count: i32,
    pub avg_confidence: f32,
}
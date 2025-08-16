use sqlx::{PgPool, Result};
use uuid::Uuid;
use crate::models::{ResearchSheet, GeneratedPleading};

pub async fn insert_research_sheet(pool: &PgPool, sheet: ResearchSheet) -> Result<ResearchSheet> {
    let row = sqlx::query!(
        r#"
        INSERT INTO research_sheets (
            id, document_id, case_reference, client_name, generated_at,
            analysis_summary, legal_findings, precedent_citations,
            recommended_actions, draft_pleading, confidence_score,
            created_at, updated_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
        RETURNING *
        "#,
        sheet.id,
        sheet.document_id,
        sheet.case_reference,
        sheet.client_name,
        sheet.generated_at,
        sheet.analysis_summary,
        sheet.legal_findings,
        sheet.precedent_citations,
        sheet.recommended_actions,
        sheet.draft_pleading,
        sheet.confidence_score,
        sheet.created_at,
        sheet.updated_at
    )
    .fetch_one(pool)
    .await?;

    Ok(ResearchSheet {
        id: row.id,
        document_id: row.document_id,
        case_reference: row.case_reference,
        client_name: row.client_name,
        generated_at: row.generated_at,
        analysis_summary: row.analysis_summary,
        legal_findings: row.legal_findings,
        precedent_citations: row.precedent_citations,
        recommended_actions: row.recommended_actions,
        draft_pleading: row.draft_pleading,
        confidence_score: row.confidence_score,
        created_at: row.created_at,
        updated_at: row.updated_at,
    })
}

pub async fn get_research_sheet_by_document(pool: &PgPool, document_id: Uuid) -> Result<Option<ResearchSheet>> {
    let row = sqlx::query!(
        "SELECT * FROM research_sheets WHERE document_id = $1 ORDER BY created_at DESC LIMIT 1",
        document_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| ResearchSheet {
        id: r.id,
        document_id: r.document_id,
        case_reference: r.case_reference,
        client_name: r.client_name,
        generated_at: r.generated_at,
        analysis_summary: r.analysis_summary,
        legal_findings: r.legal_findings,
        precedent_citations: r.precedent_citations,
        recommended_actions: r.recommended_actions,
        draft_pleading: r.draft_pleading,
        confidence_score: r.confidence_score,
        created_at: r.created_at,
        updated_at: r.updated_at,
    }))
}

pub async fn get_research_sheet_by_id(pool: &PgPool, id: Uuid) -> Result<Option<ResearchSheet>> {
    let row = sqlx::query!(
        "SELECT * FROM research_sheets WHERE id = $1",
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| ResearchSheet {
        id: r.id,
        document_id: r.document_id,
        case_reference: r.case_reference,
        client_name: r.client_name,
        generated_at: r.generated_at,
        analysis_summary: r.analysis_summary,
        legal_findings: r.legal_findings,
        precedent_citations: r.precedent_citations,
        recommended_actions: r.recommended_actions,
        draft_pleading: r.draft_pleading,
        confidence_score: r.confidence_score,
        created_at: r.created_at,
        updated_at: r.updated_at,
    }))
}

pub async fn update_research_sheet(pool: &PgPool, sheet: ResearchSheet) -> Result<ResearchSheet> {
    let row = sqlx::query!(
        r#"
        UPDATE research_sheets SET
            case_reference = $2,
            client_name = $3,
            analysis_summary = $4,
            legal_findings = $5,
            precedent_citations = $6,
            recommended_actions = $7,
            draft_pleading = $8,
            confidence_score = $9,
            updated_at = NOW()
        WHERE id = $1
        RETURNING *
        "#,
        sheet.id,
        sheet.case_reference,
        sheet.client_name,
        sheet.analysis_summary,
        sheet.legal_findings,
        sheet.precedent_citations,
        sheet.recommended_actions,
        sheet.draft_pleading,
        sheet.confidence_score
    )
    .fetch_one(pool)
    .await?;

    Ok(ResearchSheet {
        id: row.id,
        document_id: row.document_id,
        case_reference: row.case_reference,
        client_name: row.client_name,
        generated_at: row.generated_at,
        analysis_summary: row.analysis_summary,
        legal_findings: row.legal_findings,
        precedent_citations: row.precedent_citations,
        recommended_actions: row.recommended_actions,
        draft_pleading: row.draft_pleading,
        confidence_score: row.confidence_score,
        created_at: row.created_at,
        updated_at: row.updated_at,
    })
}

pub async fn insert_generated_pleading(pool: &PgPool, pleading: GeneratedPleading) -> Result<GeneratedPleading> {
    let row = sqlx::query!(
        r#"
        INSERT INTO generated_pleadings (
            id, research_sheet_id, pleading_type, title, content, citations, generated_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING *
        "#,
        pleading.id,
        pleading.research_sheet_id,
        pleading.pleading_type,
        pleading.title,
        pleading.content,
        serde_json::to_value(&pleading.citations).unwrap(),
        pleading.generated_at
    )
    .fetch_one(pool)
    .await?;

    let citations: Vec<crate::models::Citation> = serde_json::from_value(row.citations).unwrap_or_default();

    Ok(GeneratedPleading {
        id: row.id,
        research_sheet_id: row.research_sheet_id,
        pleading_type: row.pleading_type,
        title: row.title,
        content: row.content,
        citations,
        generated_at: row.generated_at,
    })
}

pub async fn get_pleadings_by_research_sheet(pool: &PgPool, research_sheet_id: Uuid) -> Result<Vec<GeneratedPleading>> {
    let rows = sqlx::query!(
        "SELECT * FROM generated_pleadings WHERE research_sheet_id = $1 ORDER BY generated_at DESC",
        research_sheet_id
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| {
        let citations: Vec<crate::models::Citation> = serde_json::from_value(r.citations).unwrap_or_default();
        
        GeneratedPleading {
            id: r.id,
            research_sheet_id: r.research_sheet_id,
            pleading_type: r.pleading_type,
            title: r.title,
            content: r.content,
            citations,
            generated_at: r.generated_at,
        }
    }).collect())
}

pub async fn get_all_research_sheets(pool: &PgPool, limit: Option<i32>) -> Result<Vec<ResearchSheet>> {
    let limit = limit.unwrap_or(50);
    
    let rows = sqlx::query!(
        "SELECT * FROM research_sheets ORDER BY created_at DESC LIMIT $1",
        limit as i64
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| ResearchSheet {
        id: r.id,
        document_id: r.document_id,
        case_reference: r.case_reference,
        client_name: r.client_name,
        generated_at: r.generated_at,
        analysis_summary: r.analysis_summary,
        legal_findings: r.legal_findings,
        precedent_citations: r.precedent_citations,
        recommended_actions: r.recommended_actions,
        draft_pleading: r.draft_pleading,
        confidence_score: r.confidence_score,
        created_at: r.created_at,
        updated_at: r.updated_at,
    }).collect())
}

pub async fn search_research_sheets(pool: &PgPool, search_term: &str) -> Result<Vec<ResearchSheet>> {
    let rows = sqlx::query!(
        r#"
        SELECT * FROM research_sheets 
        WHERE analysis_summary ILIKE $1 
           OR case_reference ILIKE $1
           OR client_name ILIKE $1
        ORDER BY confidence_score DESC, created_at DESC
        LIMIT 100
        "#,
        format!("%{}%", search_term)
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| ResearchSheet {
        id: r.id,
        document_id: r.document_id,
        case_reference: r.case_reference,
        client_name: r.client_name,
        generated_at: r.generated_at,
        analysis_summary: r.analysis_summary,
        legal_findings: r.legal_findings,
        precedent_citations: r.precedent_citations,
        recommended_actions: r.recommended_actions,
        draft_pleading: r.draft_pleading,
        confidence_score: r.confidence_score,
        created_at: r.created_at,
        updated_at: r.updated_at,
    }).collect())
}
use sqlx::{PgPool, Result, Row};
use uuid::Uuid;
use crate::models::{LegalCase, CaseSearchRequest, CaseMatch};

pub async fn insert_case(pool: &PgPool, case: LegalCase) -> Result<LegalCase> {
    let row = sqlx::query!(
        r#"
        INSERT INTO legal_cases (
            id, case_number, case_name, country, date, currency, 
            key_ruling, full_text, court, case_type, significance_score,
            created_at, updated_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
        RETURNING *
        "#,
        case.id,
        case.case_number,
        case.case_name,
        case.country,
        case.date,
        case.currency,
        case.key_ruling,
        case.full_text,
        case.court,
        case.case_type,
        case.significance_score,
        case.created_at,
        case.updated_at
    )
    .fetch_one(pool)
    .await?;

    Ok(LegalCase {
        id: row.id,
        case_number: row.case_number,
        case_name: row.case_name,
        country: row.country,
        date: row.date,
        currency: row.currency,
        key_ruling: row.key_ruling,
        full_text: row.full_text,
        court: row.court,
        case_type: row.case_type,
        significance_score: row.significance_score,
        embedding: None, // Will be populated separately
        created_at: row.created_at,
        updated_at: row.updated_at,
    })
}

pub async fn get_case_by_id(pool: &PgPool, id: Uuid) -> Result<Option<LegalCase>> {
    let row = sqlx::query!(
        "SELECT * FROM legal_cases WHERE id = $1",
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| LegalCase {
        id: r.id,
        case_number: r.case_number,
        case_name: r.case_name,
        country: r.country,
        date: r.date,
        currency: r.currency,
        key_ruling: r.key_ruling,
        full_text: r.full_text,
        court: r.court,
        case_type: r.case_type,
        significance_score: r.significance_score,
        embedding: None,
        created_at: r.created_at,
        updated_at: r.updated_at,
    }))
}

pub async fn get_all_cases(pool: &PgPool, limit: Option<i32>) -> Result<Vec<LegalCase>> {
    let limit = limit.unwrap_or(100);
    
    let rows = sqlx::query!(
        "SELECT * FROM legal_cases ORDER BY date DESC LIMIT $1",
        limit as i64
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| LegalCase {
        id: r.id,
        case_number: r.case_number,
        case_name: r.case_name,
        country: r.country,
        date: r.date,
        currency: r.currency,
        key_ruling: r.key_ruling,
        full_text: r.full_text,
        court: r.court,
        case_type: r.case_type,
        significance_score: r.significance_score,
        embedding: None,
        created_at: r.created_at,
        updated_at: r.updated_at,
    }).collect())
}

pub async fn search_cases(pool: &PgPool, request: CaseSearchRequest) -> Result<Vec<LegalCase>> {
    let mut query = "SELECT * FROM legal_cases WHERE 1=1".to_string();
    let mut params: Vec<Box<dyn sqlx::Encode<'_, sqlx::Postgres> + Send + Sync>> = Vec::new();
    let mut param_count = 0;

    // Add search filters
    if !request.query.is_empty() {
        param_count += 1;
        query.push_str(&format!(
            " AND (case_name ILIKE ${} OR key_ruling ILIKE ${} OR full_text ILIKE ${})",
            param_count, param_count, param_count
        ));
        let search_term = format!("%{}%", request.query);
        params.push(Box::new(search_term));
    }

    if let Some(country) = request.country {
        param_count += 1;
        query.push_str(&format!(" AND country = ${}", param_count));
        params.push(Box::new(country));
    }

    if let Some(currency) = request.currency {
        param_count += 1;
        query.push_str(&format!(" AND currency ILIKE ${}", param_count));
        let currency_pattern = format!("%{}%", currency);
        params.push(Box::new(currency_pattern));
    }

    if let Some(date_from) = request.date_from {
        param_count += 1;
        query.push_str(&format!(" AND date >= ${}", param_count));
        params.push(Box::new(date_from));
    }

    if let Some(date_to) = request.date_to {
        param_count += 1;
        query.push_str(&format!(" AND date <= ${}", param_count));
        params.push(Box::new(date_to));
    }

    if let Some(case_type) = request.case_type {
        param_count += 1;
        query.push_str(&format!(" AND case_type = ${}", param_count));
        params.push(Box::new(case_type));
    }

    query.push_str(" ORDER BY significance_score DESC NULLS LAST, date DESC");
    
    let limit = request.limit.unwrap_or(50);
    param_count += 1;
    query.push_str(&format!(" LIMIT ${}", param_count));
    params.push(Box::new(limit as i64));

    // Execute dynamic query
    let mut query_builder = sqlx::query(&query);
    for param in params {
        // Note: This is a simplified approach. In production, use query builders or macros
        // for type-safe dynamic queries
    }

    // For now, use a simplified search
    let rows = sqlx::query!(
        r#"
        SELECT * FROM legal_cases 
        WHERE case_name ILIKE $1 
           OR key_ruling ILIKE $1 
           OR full_text ILIKE $1
        ORDER BY significance_score DESC NULLS LAST, date DESC
        LIMIT $2
        "#,
        format!("%{}%", request.query),
        limit as i64
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| LegalCase {
        id: r.id,
        case_number: r.case_number,
        case_name: r.case_name,
        country: r.country,
        date: r.date,
        currency: r.currency,
        key_ruling: r.key_ruling,
        full_text: r.full_text,
        court: r.court,
        case_type: r.case_type,
        significance_score: r.significance_score,
        embedding: None,
        created_at: r.created_at,
        updated_at: r.updated_at,
    }).collect())
}

pub async fn get_fx_related_cases(pool: &PgPool) -> Result<Vec<LegalCase>> {
    let rows = sqlx::query!(
        r#"
        SELECT * FROM legal_cases 
        WHERE currency ~ '(CHF|EUR|USD|GBP)' 
           OR case_name ILIKE '%foreign%currency%'
           OR case_name ILIKE '%deviza%'
           OR key_ruling ILIKE '%currency%'
           OR key_ruling ILIKE '%exchange%rate%'
        ORDER BY date DESC
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| LegalCase {
        id: r.id,
        case_number: r.case_number,
        case_name: r.case_name,
        country: r.country,
        date: r.date,
        currency: r.currency,
        key_ruling: r.key_ruling,
        full_text: r.full_text,
        court: r.court,
        case_type: r.case_type,
        significance_score: r.significance_score,
        embedding: None,
        created_at: r.created_at,
        updated_at: r.updated_at,
    }).collect())
}

pub async fn update_case_embedding(pool: &PgPool, case_id: Uuid, embedding: Vec<f32>) -> Result<()> {
    sqlx::query!(
        "UPDATE legal_cases SET embedding = $1 WHERE id = $2",
        &embedding,
        case_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn find_similar_cases(pool: &PgPool, embedding: Vec<f32>, limit: i32) -> Result<Vec<CaseMatch>> {
    // Note: This requires pgvector extension for efficient similarity search
    // For now, we'll use a simplified approach
    let cases = get_fx_related_cases(pool).await?;
    
    // In production, this would use vector similarity search
    let matches: Vec<CaseMatch> = cases.into_iter()
        .take(limit as usize)
        .map(|case| CaseMatch {
            similarity_score: 0.8, // Placeholder
            matching_clauses: vec!["currency risk".to_string()],
            relevance_explanation: "Related to foreign currency mortgage terms".to_string(),
            case,
        })
        .collect();

    Ok(matches)
}
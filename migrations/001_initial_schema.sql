-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Enable pgvector extension for embeddings (if available)
-- CREATE EXTENSION IF NOT EXISTS vector;

-- Legal Cases table (precedents)
CREATE TABLE legal_cases (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    case_number VARCHAR(50) NOT NULL UNIQUE,
    case_name TEXT NOT NULL,
    country VARCHAR(100) NOT NULL,
    date TIMESTAMPTZ NOT NULL,
    currency VARCHAR(20) NOT NULL,
    key_ruling TEXT NOT NULL,
    full_text TEXT,
    court VARCHAR(100),
    case_type VARCHAR(50) NOT NULL DEFAULT 'unknown',
    significance_score REAL,
    embedding REAL[], -- Vector embedding for similarity search
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Documents table (client documents)
CREATE TABLE documents (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    filename VARCHAR(255) NOT NULL,
    content_type VARCHAR(100) NOT NULL,
    file_size BIGINT NOT NULL DEFAULT 0,
    original_text TEXT,
    extracted_text TEXT,
    document_type VARCHAR(50) NOT NULL,
    language VARCHAR(10) NOT NULL DEFAULT 'unknown',
    client_id VARCHAR(100),
    case_reference VARCHAR(100),
    processing_status VARCHAR(20) NOT NULL DEFAULT 'uploaded',
    error_message TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Clause patterns for extraction
CREATE TABLE clause_patterns (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    pattern_type VARCHAR(20) NOT NULL, -- regex, keyword, semantic
    pattern_text TEXT NOT NULL,
    language VARCHAR(20) NOT NULL,
    clause_category VARCHAR(50) NOT NULL,
    severity VARCHAR(20) NOT NULL DEFAULT 'medium',
    description TEXT NOT NULL,
    legal_basis TEXT,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Extracted clauses from documents
CREATE TABLE extracted_clauses (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    document_id UUID NOT NULL REFERENCES documents(id) ON DELETE CASCADE,
    clause_type VARCHAR(50) NOT NULL,
    clause_text TEXT NOT NULL,
    original_language VARCHAR(10) NOT NULL,
    english_translation TEXT,
    start_position INTEGER,
    end_position INTEGER,
    confidence_score REAL NOT NULL DEFAULT 0.0,
    risk_level VARCHAR(20) NOT NULL DEFAULT 'medium',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Precedent matches for clauses
CREATE TABLE precedent_matches (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    clause_id UUID NOT NULL REFERENCES extracted_clauses(id) ON DELETE CASCADE,
    case_id UUID NOT NULL REFERENCES legal_cases(id) ON DELETE CASCADE,
    similarity_score REAL NOT NULL,
    matching_elements TEXT[],
    applicability_assessment TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Research sheets
CREATE TABLE research_sheets (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    document_id UUID NOT NULL REFERENCES documents(id) ON DELETE CASCADE,
    case_reference VARCHAR(100),
    client_name VARCHAR(255),
    generated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    analysis_summary TEXT NOT NULL DEFAULT '',
    legal_findings JSONB NOT NULL DEFAULT '{}',
    precedent_citations JSONB NOT NULL DEFAULT '[]',
    recommended_actions JSONB NOT NULL DEFAULT '[]',
    draft_pleading TEXT,
    confidence_score REAL NOT NULL DEFAULT 0.0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Generated pleadings
CREATE TABLE generated_pleadings (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    research_sheet_id UUID NOT NULL REFERENCES research_sheets(id) ON DELETE CASCADE,
    pleading_type VARCHAR(50) NOT NULL,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    citations JSONB NOT NULL DEFAULT '[]',
    generated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes for better performance
CREATE INDEX idx_legal_cases_country ON legal_cases(country);
CREATE INDEX idx_legal_cases_currency ON legal_cases(currency);
CREATE INDEX idx_legal_cases_date ON legal_cases(date);
CREATE INDEX idx_legal_cases_case_type ON legal_cases(case_type);
CREATE INDEX idx_legal_cases_case_number ON legal_cases(case_number);

CREATE INDEX idx_documents_document_type ON documents(document_type);
CREATE INDEX idx_documents_processing_status ON documents(processing_status);
CREATE INDEX idx_documents_client_id ON documents(client_id);
CREATE INDEX idx_documents_case_reference ON documents(case_reference);

CREATE INDEX idx_extracted_clauses_document_id ON extracted_clauses(document_id);
CREATE INDEX idx_extracted_clauses_clause_type ON extracted_clauses(clause_type);
CREATE INDEX idx_extracted_clauses_risk_level ON extracted_clauses(risk_level);

CREATE INDEX idx_precedent_matches_clause_id ON precedent_matches(clause_id);
CREATE INDEX idx_precedent_matches_case_id ON precedent_matches(case_id);
CREATE INDEX idx_precedent_matches_similarity_score ON precedent_matches(similarity_score);

CREATE INDEX idx_research_sheets_document_id ON research_sheets(document_id);
CREATE INDEX idx_research_sheets_case_reference ON research_sheets(case_reference);

-- Full-text search indexes
CREATE INDEX idx_legal_cases_full_text ON legal_cases USING gin(to_tsvector('english', case_name || ' ' || key_ruling || ' ' || COALESCE(full_text, '')));
CREATE INDEX idx_extracted_clauses_full_text ON extracted_clauses USING gin(to_tsvector('english', clause_text));

-- Create updated_at trigger function
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Apply updated_at triggers
CREATE TRIGGER update_legal_cases_updated_at BEFORE UPDATE ON legal_cases FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_documents_updated_at BEFORE UPDATE ON documents FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_clause_patterns_updated_at BEFORE UPDATE ON clause_patterns FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_research_sheets_updated_at BEFORE UPDATE ON research_sheets FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
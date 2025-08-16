-- SQLite Schema for Hungarian FX Mortgage Legal Research System

-- Legal Cases table (precedents)
CREATE TABLE legal_cases (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    case_number TEXT NOT NULL UNIQUE,
    case_name TEXT NOT NULL,
    country TEXT NOT NULL,
    date TEXT NOT NULL, -- Using TEXT for DATETIME in SQLite
    currency TEXT NOT NULL,
    key_ruling TEXT NOT NULL,
    full_text TEXT,
    court TEXT,
    case_type TEXT NOT NULL DEFAULT 'unknown',
    significance_score REAL,
    embedding TEXT, -- JSON string of vector embedding
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Documents table (client documents)
CREATE TABLE documents (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    filename TEXT NOT NULL,
    content_type TEXT NOT NULL,
    file_size INTEGER NOT NULL DEFAULT 0,
    original_text TEXT,
    extracted_text TEXT,
    document_type TEXT NOT NULL,
    language TEXT NOT NULL DEFAULT 'unknown',
    client_id TEXT,
    case_reference TEXT,
    processing_status TEXT NOT NULL DEFAULT 'uploaded',
    error_message TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Clause patterns for extraction
CREATE TABLE clause_patterns (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    name TEXT NOT NULL,
    pattern_type TEXT NOT NULL, -- regex, keyword, semantic
    pattern_text TEXT NOT NULL,
    language TEXT NOT NULL,
    clause_category TEXT NOT NULL,
    severity TEXT NOT NULL DEFAULT 'medium',
    description TEXT NOT NULL,
    legal_basis TEXT,
    is_active INTEGER NOT NULL DEFAULT 1, -- SQLite uses INTEGER for BOOLEAN
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Extracted clauses from documents
CREATE TABLE extracted_clauses (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    document_id TEXT NOT NULL REFERENCES documents(id) ON DELETE CASCADE,
    clause_type TEXT NOT NULL,
    clause_text TEXT NOT NULL,
    original_language TEXT NOT NULL,
    english_translation TEXT,
    start_position INTEGER,
    end_position INTEGER,
    confidence_score REAL NOT NULL DEFAULT 0.0,
    risk_level TEXT NOT NULL DEFAULT 'medium',
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Precedent matches for clauses
CREATE TABLE precedent_matches (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    clause_id TEXT NOT NULL REFERENCES extracted_clauses(id) ON DELETE CASCADE,
    case_id TEXT NOT NULL REFERENCES legal_cases(id) ON DELETE CASCADE,
    similarity_score REAL NOT NULL,
    matching_elements TEXT, -- JSON string instead of array
    applicability_assessment TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Research sheets
CREATE TABLE research_sheets (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    document_id TEXT NOT NULL REFERENCES documents(id) ON DELETE CASCADE,
    case_reference TEXT,
    client_name TEXT,
    generated_at TEXT NOT NULL DEFAULT (datetime('now')),
    analysis_summary TEXT NOT NULL DEFAULT '',
    legal_findings TEXT NOT NULL DEFAULT '{}', -- JSON string
    precedent_citations TEXT NOT NULL DEFAULT '[]', -- JSON string
    recommended_actions TEXT NOT NULL DEFAULT '[]', -- JSON string
    draft_pleading TEXT,
    confidence_score REAL NOT NULL DEFAULT 0.0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Generated pleadings
CREATE TABLE generated_pleadings (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    research_sheet_id TEXT NOT NULL REFERENCES research_sheets(id) ON DELETE CASCADE,
    pleading_type TEXT NOT NULL,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    citations TEXT NOT NULL DEFAULT '[]', -- JSON string
    generated_at TEXT NOT NULL DEFAULT (datetime('now'))
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

-- SQLite FTS (Full-Text Search) tables
CREATE VIRTUAL TABLE legal_cases_fts USING fts5(
    case_name, key_ruling, full_text, content='legal_cases', content_rowid='rowid'
);

CREATE VIRTUAL TABLE extracted_clauses_fts USING fts5(
    clause_text, content='extracted_clauses', content_rowid='rowid'
);

-- Triggers to keep FTS tables in sync
CREATE TRIGGER legal_cases_fts_insert AFTER INSERT ON legal_cases BEGIN
    INSERT INTO legal_cases_fts(rowid, case_name, key_ruling, full_text) 
    VALUES (new.rowid, new.case_name, new.key_ruling, new.full_text);
END;

CREATE TRIGGER legal_cases_fts_update AFTER UPDATE ON legal_cases BEGIN
    UPDATE legal_cases_fts SET 
        case_name = new.case_name, 
        key_ruling = new.key_ruling, 
        full_text = new.full_text 
    WHERE rowid = new.rowid;
END;

CREATE TRIGGER legal_cases_fts_delete AFTER DELETE ON legal_cases BEGIN
    DELETE FROM legal_cases_fts WHERE rowid = old.rowid;
END;

CREATE TRIGGER extracted_clauses_fts_insert AFTER INSERT ON extracted_clauses BEGIN
    INSERT INTO extracted_clauses_fts(rowid, clause_text) 
    VALUES (new.rowid, new.clause_text);
END;

CREATE TRIGGER extracted_clauses_fts_update AFTER UPDATE ON extracted_clauses BEGIN
    UPDATE extracted_clauses_fts SET clause_text = new.clause_text WHERE rowid = new.rowid;
END;

CREATE TRIGGER extracted_clauses_fts_delete AFTER DELETE ON extracted_clauses BEGIN
    DELETE FROM extracted_clauses_fts WHERE rowid = old.rowid;
END;

-- Updated_at triggers for SQLite
CREATE TRIGGER update_legal_cases_updated_at 
    AFTER UPDATE ON legal_cases FOR EACH ROW
    WHEN OLD.updated_at = NEW.updated_at
BEGIN
    UPDATE legal_cases SET updated_at = datetime('now') WHERE id = NEW.id;
END;

CREATE TRIGGER update_documents_updated_at 
    AFTER UPDATE ON documents FOR EACH ROW
    WHEN OLD.updated_at = NEW.updated_at
BEGIN
    UPDATE documents SET updated_at = datetime('now') WHERE id = NEW.id;
END;

CREATE TRIGGER update_clause_patterns_updated_at 
    AFTER UPDATE ON clause_patterns FOR EACH ROW
    WHEN OLD.updated_at = NEW.updated_at
BEGIN
    UPDATE clause_patterns SET updated_at = datetime('now') WHERE id = NEW.id;
END;

CREATE TRIGGER update_research_sheets_updated_at 
    AFTER UPDATE ON research_sheets FOR EACH ROW
    WHEN OLD.updated_at = NEW.updated_at
BEGIN
    UPDATE research_sheets SET updated_at = datetime('now') WHERE id = NEW.id;
END;
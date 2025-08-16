# Legal Research System - Foreign Currency Mortgage Analysis

A Rust-based legal research system designed to analyze foreign currency mortgage contracts and find relevant legal precedents, specifically focusing on the Central and Eastern European FX mortgage crisis.

## Features

- **Document Processing**: Upload and analyze PDF, Word, image, and text documents
- **Clause Extraction**: Extract and categorize contract clauses using multilingual regex patterns
- **Precedent Matching**: Match contract clauses against CJEU and national court precedents
- **Legal Drafting**: Auto-generate legal pleadings based on extracted clauses and matched precedents
- **Multilingual Support**: Hungarian, English, Czech, Polish, Romanian, Croatian, and Slovenian
- **Web Interface**: Modern HTML/CSS/JavaScript frontend for easy interaction

## Architecture

```
src/
├── main.rs              # Application entry point
├── models/              # Data models (cases, documents, clauses, research)
├── extractors/          # Text processing and clause extraction
├── matching/            # Precedent matching and similarity algorithms
├── api/                 # REST API endpoints
└── db/                  # Database operations
```

## Database Schema

The system uses PostgreSQL with the following main tables:
- `legal_cases` - CJEU and national court precedents
- `documents` - Uploaded client documents
- `extracted_clauses` - Contract clauses extracted from documents
- `precedent_matches` - Matches between clauses and precedents
- `research_sheets` - Complete legal research analysis
- `generated_pleadings` - Auto-drafted legal documents

## Prerequisites

1. **Rust** (latest stable version)
2. **PostgreSQL** (v12 or later) or **Supabase** account
3. **Optional**: Tesseract OCR for image processing
4. **Optional**: OpenAI API key for advanced embeddings

## Setup Instructions

### 1. Clone and Install Dependencies

```bash
git clone <repository-url>
cd devizahitel_legal_research
cargo build
```

### 2. Database Setup

#### Option A: Local PostgreSQL
```bash
# Install PostgreSQL and create database
createdb devizahitel_legal_research

# Set environment variable
export DATABASE_URL="postgresql://username:password@localhost:5432/devizahitel_legal_research"
```

#### Option B: Supabase (Recommended)
1. Create a new project at [supabase.com](https://supabase.com)
2. Copy the database URL from your project settings
3. Set environment variable:
```bash
export DATABASE_URL="postgresql://postgres:password@db.your-project.supabase.co:5432/postgres"
```

### 3. Environment Configuration

Copy the example environment file and configure:
```bash
cp .env.example .env
```

Edit `.env` with your settings:
```env
DATABASE_URL=your_database_url_here
PORT=8080
RUST_LOG=info
OPENAI_API_KEY=your_openai_key_here  # Optional
```

### 4. Database Migration

The application will automatically run migrations on startup. The migration includes:
- Creating all necessary tables
- Setting up indexes for performance
- Creating triggers for timestamp updates

### 5. Run the Application

```bash
cargo run
```

The server will start on `http://localhost:8080`

## Usage

### Web Interface

1. Open `http://localhost:8080` in your browser
2. Upload a document (PDF, Word, image, or text)
3. The system will:
   - Extract text from the document
   - Identify and categorize contract clauses
   - Match clauses against legal precedents
   - Generate a draft legal pleading

### API Endpoints

#### Document Management
- `POST /api/documents` - Upload and process document
- `GET /api/documents` - List all documents
- `GET /api/documents/{id}` - Get specific document with clauses

#### Legal Cases
- `GET /api/cases` - List legal precedents
- `GET /api/cases/{id}` - Get specific case
- `POST /api/cases/search` - Search cases with filters

#### Research Workflow
- `POST /api/research/extract-clauses` - Extract clauses from document
- `POST /api/research/match-precedents` - Find matching precedents
- `POST /api/research/generate-draft` - Generate legal pleading
- `GET /api/research/sheet/{document_id}` - Get complete research analysis

### Example API Usage

```bash
# Upload a document
curl -X POST http://localhost:8080/api/documents \
  -H "Content-Type: application/json" \
  -d '{
    "filename": "mortgage_contract.pdf",
    "content_type": "application/pdf",
    "file_data": "base64_encoded_file_content",
    "document_type": "mortgage_contract",
    "language": "hu"
  }'

# Search for FX-related cases
curl -X POST http://localhost:8080/api/cases/search \
  -H "Content-Type: application/json" \
  -d '{
    "query": "foreign currency",
    "country": "Hungary",
    "limit": 10
  }'
```

## Legal Precedents Database

The system comes pre-loaded with key CJEU precedents:

1. **C-186/16 Andriciuc v Banca Românească** (2017) - Information disclosure requirements
2. **C-520/21 Szcześniak v Bank M.** (2023) - No bank compensation for invalid contracts
3. **C-705/21 MJ v AxFina Hungary** (2023) - Full restitution for unfair terms
4. **C-26/13 Kásler v OTP Bank** (2014) - Transparency requirements
5. **C-630/23 ZH, KN v AxFina Hungary** (2025) - Latest precedent on contract invalidation

## Clause Extraction Patterns

The system recognizes several types of clauses:

- **FX Risk Clauses**: Foreign currency risk allocation terms
- **Transparency Clauses**: Information disclosure requirements
- **Interest Rate Clauses**: Variable interest rate terms
- **Penalty Clauses**: Fees and penalty provisions
- **Unfair Terms**: General unfair contract terms

## Generated Legal Documents

The system can generate:
- **Complaints**: Initial court filings citing relevant precedents
- **Motions**: Legal motions for contract invalidation
- **Appeals**: Appellate briefs with precedent analysis

## Development

### Adding New Precedents

Add cases to the precedent database by modifying:
```rust
// src/db/mod.rs - get_default_precedent_cases()
```

### Adding New Language Support

1. Add language patterns to `src/extractors/language_detector.rs`
2. Add clause extraction patterns to `src/extractors/clause_extractor.rs`
3. Update the language detection logic

### Extending Clause Types

1. Add new patterns to `src/models/clause.rs`
2. Update extraction logic in `src/extractors/clause_extractor.rs`
3. Add matching logic in `src/matching/precedent_matcher.rs`

## Testing

```bash
# Run all tests
cargo test

# Run with logging
RUST_LOG=debug cargo test

# Test specific module
cargo test --package devizahitel_legal_research --lib models::case::tests
```

## Performance Considerations

- **Database Indexing**: Full-text search indexes on case text and clause content
- **Caching**: Consider Redis for frequently accessed precedents
- **Embeddings**: OpenAI API calls are rate-limited; implement local caching
- **OCR Processing**: Tesseract can be slow; consider background job processing

## Legal Disclaimer

This system is designed for legal research assistance only. Generated legal documents should be reviewed by qualified legal professionals before use in actual legal proceedings.

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/new-feature`)
3. Commit your changes (`git commit -m 'Add new feature'`)
4. Push to the branch (`git push origin feature/new-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Support

For issues and questions:
1. Check the existing GitHub issues
2. Create a new issue with detailed description
3. Include logs and reproduction steps

## Roadmap

- [ ] Machine learning-based clause classification
- [ ] Integration with legal databases (Westlaw, LexisNexis)
- [ ] Advanced document comparison features
- [ ] Client portal with case management
- [ ] Mobile application
- [ ] Integration with court filing systems
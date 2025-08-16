# Hungarian FX Mortgage Legal Research System - Setup Guide

🏛️ **Complete installation guide for the AI-powered legal research assistant**

---

## 🎯 System Overview

This system helps lawyers and bank victims analyze foreign currency mortgage contracts, extract violation clauses, match legal precedents, and calculate financial damages for litigation.

**Technology Stack**: Rust backend, Python OCR processor, Mistral AI, local SQLite + Chroma vector database

---

## 📋 Prerequisites

### Required Software
1. **Rust** (latest stable) - [Install from rustup.rs](https://rustup.rs/)
2. **Python 3.8+** - [Download from python.org](https://python.org)
3. **Git** - For cloning the repository

### Required API Access
1. **Mistral API Key** - [Get from platform.mistral.ai](https://platform.mistral.ai)

### Optional (for enhanced OCR)
1. **Tesseract OCR** - For fallback document processing

---

## 🚀 Quick Start (5 Minutes)

### 1. Clone Repository
```bash
git clone https://github.com/bencium/devizahitel-copilot.git
cd devizahitel-copilot
```

### 2. Setup Environment Variables
```bash
# Copy environment template
cp .env.example .env

# Edit .env with your Mistral API key
echo "MISTRAL_API_KEY=your_mistral_api_key_here" >> .env
echo "DATABASE_URL=sqlite://legal_research.db" >> .env
echo "RUST_LOG=info" >> .env
```

### 3. Install Rust Dependencies
```bash
cargo build --release
```

### 4. Setup Python OCR Processor
```bash
cd mistral_ocr_processor
cp .env.example .env
echo "MISTRAL_API_KEY=your_mistral_api_key_here" >> .env

# Install Python dependencies
pip install -r requirements.txt
cd ..
```

### 5. Initialize Database
```bash
# Run database migrations
cargo run --bin setup_databases

# Optional: Populate with legal precedents
cargo run --bin populate_precedents
```

### 6. Start the System
```bash
# Start the legal research server
cargo run

# Server will be available at http://localhost:8080
```

---

## 📖 Detailed Setup Instructions

### Step 1: Environment Setup

#### 1.1 Install Rust
```bash
# Install Rust via rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Verify installation
rustc --version
cargo --version
```

#### 1.2 Install Python Dependencies
```bash
# Check Python version (3.8+ required)
python3 --version

# Install virtual environment (recommended)
python3 -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate

# Install OCR processor dependencies
cd mistral_ocr_processor
pip install -r requirements.txt
cd ..
```

#### 1.3 Get Mistral API Key
1. Visit [platform.mistral.ai](https://platform.mistral.ai)
2. Create account and generate API key
3. Copy the API key for configuration

### Step 2: Project Configuration

#### 2.1 Environment Variables
Create `.env` file in project root:
```env
# Mistral AI Configuration
MISTRAL_API_KEY=your_actual_mistral_api_key_here
MISTRAL_MODEL_LARGE=mistral-large-latest
MISTRAL_MODEL_SMALL=mistral-small-latest

# Database Configuration
DATABASE_URL=sqlite://legal_research.db

# Application Configuration
PORT=8080
RUST_LOG=info

# OCR Configuration
MAX_FILE_SIZE_MB=10
REQUEST_TIMEOUT=120
```

#### 2.2 OCR Processor Configuration
Create `mistral_ocr_processor/.env`:
```env
# Mistral OCR Configuration
MISTRAL_API_KEY=your_actual_mistral_api_key_here
```

### Step 3: Database Setup

#### 3.1 Initialize SQLite Database
```bash
# Create database and run migrations
cargo run --bin setup_databases

# This creates:
# - legal_research.db (SQLite database)
# - chroma_db/ (vector database)
# - All required tables and indexes
```

#### 3.2 Populate Legal Precedents (Optional)
```bash
# Load CJEU and national court precedents
cargo run --bin populate_precedents

# This adds:
# - CJEU cases (C-26/13, C-186/16, C-630/23, etc.)
# - Hungarian Kúria decisions
# - Polish Supreme Court precedents
# - Banking Act and Civil Code references
```

### Step 4: System Verification

#### 4.1 Test Rust Backend
```bash
# Build and test the system
cargo build --release
cargo test

# Start the server
cargo run

# Should see:
# ✅ Database connected (SQLite)
# ✅ Vector store initialized (Chroma)
# ✅ Mistral API verified
# 🌐 Server running at http://localhost:8080
```

#### 4.2 Test OCR Processor
```bash
cd mistral_ocr_processor

# Test OCR with example document
python3 main.py --help

# Should display usage instructions without errors
```

#### 4.3 Web Interface Test
```bash
# Open browser to http://localhost:8080
# Should see legal research interface
# Try uploading a sample document
```

---

## 🔧 Usage Instructions

### Document Processing Workflow

#### 1. Prepare Documents
```bash
# Create input directory for documents
mkdir documents_to_process
cd documents_to_process

# Add your documents:
# - PDF contracts (mortgage agreements)
# - Scanned images (contract pages)
# - Word documents (legal correspondence)
# - Email files (bank communications)
```

#### 2. Run OCR Processing
```bash
cd mistral_ocr_processor

# Process all documents in folder
python3 main.py \
  --input-dir ../documents_to_process \
  --output-dir ../ocr_output \
  --verbose

# This will:
# - Extract text from PDFs and images
# - Create .md files with processed content
# - Generate processing summary
```

#### 3. Legal Analysis
```bash
# Start the legal research server
cargo run

# Open http://localhost:8080 in browser
# System will automatically:
# - Detect processed documents
# - Extract contract clauses
# - Match against legal precedents
# - Generate legal analysis report
```

### Web Interface Usage

#### Document Upload
1. Navigate to `http://localhost:8080`
2. Click "Upload Document"
3. Select contract files (PDF, images, Word docs)
4. Wait for processing completion

#### Legal Analysis
1. View extracted clauses in "Contract Analysis" tab
2. Review matched precedents in "Legal Precedents" tab
3. Check financial damages in "Damage Calculator" tab
4. Download legal package in "Reports" tab

#### Generated Outputs
- **Legal Complaint Draft** (Hungarian/English)
- **Evidence Summary** with clause extractions
- **Precedent Citations** with paragraph references
- **Financial Damage Report** with calculations
- **Settlement Negotiation Package**

---

## 🛠️ Troubleshooting

### Common Issues

#### "Mistral API Key Not Found"
```bash
# Check environment variable
echo $MISTRAL_API_KEY

# If empty, set it:
export MISTRAL_API_KEY="your_key_here"

# Or add to .env file:
echo "MISTRAL_API_KEY=your_key_here" >> .env
```

#### "Database Connection Failed"
```bash
# Check database file exists
ls -la legal_research.db

# If missing, initialize:
cargo run --bin setup_databases

# Check permissions
chmod 664 legal_research.db
```

#### "Rust Build Errors"
```bash
# Update Rust toolchain
rustup update

# Clean and rebuild
cargo clean
cargo build --release

# Check Rust version (should be 1.70+)
rustc --version
```

#### "Python Dependencies Failed"
```bash
# Upgrade pip
pip install --upgrade pip

# Install with verbose output
pip install -r requirements.txt --verbose

# Try with virtual environment
python3 -m venv venv
source venv/bin/activate
pip install -r requirements.txt
```

#### "OCR Processing Fails"
```bash
# Check Mistral API key
cd mistral_ocr_processor
python3 -c "import os; print(os.getenv('MISTRAL_API_KEY'))"

# Test with small file first
python3 main.py --input-dir test_docs --output-dir test_output --verbose

# Install fallback OCR (optional)
# Ubuntu/Debian: sudo apt-get install tesseract-ocr
# macOS: brew install tesseract
# Windows: Download from UB Mannheim
```

### Performance Optimization

#### Large Document Processing
```bash
# Increase file size limits in config.py
MAX_FILE_SIZE_MB = 25

# Use batch processing for multiple files
python3 main.py --batch-size 5 --timeout 300
```

#### Memory Usage
```bash
# Monitor memory usage
cargo run --features="memory-profiling"

# For large precedent databases, consider:
# - Enabling vector compression in Chroma
# - Using pagination for large result sets
```

### Development Mode

#### Enable Debug Logging
```bash
# Set detailed logging
export RUST_LOG=debug

# Run with logging
cargo run 2>&1 | tee system.log
```

#### Database Inspection
```bash
# Install SQLite CLI
sudo apt-get install sqlite3  # Ubuntu/Debian
brew install sqlite3         # macOS

# Inspect database
sqlite3 legal_research.db
.tables
.schema legal_cases
SELECT COUNT(*) FROM legal_cases;
```

---

## 📊 System Features

### Document Processing
- **Multi-format Support**: PDF, Word, images, text files
- **OCR Integration**: Mistral OCR API + Tesseract fallback
- **Language Detection**: Hungarian, English, multilingual documents
- **Batch Processing**: Handle multiple documents simultaneously

### Legal Analysis
- **Clause Extraction**: 9 types of contract violations
  - FX risk disclosure failures
  - Exchange rate spread abuse
  - Broker liability violations
  - Insurance assignment breaches
  - Contract modification abuse
- **Precedent Matching**: Hybrid vector + keyword search
- **Financial Calculator**: Comprehensive damage calculations
- **Multilingual Output**: Hungarian and English legal documents

### Database & Search
- **Local Storage**: SQLite + Chroma vector database
- **Privacy-First**: No external data transmission
- **Fast Search**: Sub-second precedent matching
- **Comprehensive**: 50+ CJEU and national court cases

---

## 🏛️ Legal Context

### Supported Case Types
- **CHF Mortgages**: Swiss Franc loans (2006-2016)
- **EUR Mortgages**: Euro-denominated loans
- **FX Risk Violations**: Inadequate currency risk disclosure
- **Broker Liability**: Financial intermediary negligence
- **Insurance Violations**: Assignment requirement breaches

### Legal Framework Coverage
- **CJEU Precedents**: C-630/23, C-186/16, C-26/13
- **Hungarian Law**: Banking Act 219/A-B§, Civil Code 241§
- **Consumer Protection**: EU Directive 93/13/EEC
- **Recent Updates**: Post-April 2025 CJEU ruling compliance

### Generated Legal Documents
- **Court Complaints**: Structured legal filings
- **Settlement Demands**: Negotiation packages
- **Evidence Summaries**: Organized violation proofs
- **Damage Calculations**: Detailed financial analysis

---

## 🔐 Security & Privacy

### Data Protection
- **Local Processing**: All documents processed locally
- **No Cloud Storage**: SQLite and Chroma databases local only
- **API Security**: Mistral API calls use secure HTTPS
- **Privacy by Design**: No personal data leaves your system

### File Security
- **Secure Deletion**: Temporary files automatically cleaned
- **Access Control**: Database file permissions restricted
- **Audit Logging**: All operations logged for review

---

## 🆘 Support

### Getting Help
1. **Documentation**: Check this SETUP.md file
2. **GitHub Issues**: [Create an issue](https://github.com/bencium/devizahitel-copilot/issues)
3. **Logs**: Include system logs with any bug reports

### Reporting Issues
When reporting problems, include:
- Operating system and version
- Rust version (`rustc --version`)
- Python version (`python3 --version`)
- Error messages and logs
- Steps to reproduce the issue

### Contributing
1. Fork the repository
2. Create feature branch
3. Test thoroughly
4. Submit pull request

---

## 📝 Legal Disclaimer

This system is designed for legal research assistance only. Generated legal documents should be reviewed by qualified legal professionals before use in actual legal proceedings. The system provides research support but does not constitute legal advice.

---

**🎉 System Ready!** 

Your Hungarian FX Mortgage Legal Research System is now installed and ready to help analyze contracts, find precedents, and generate legal documents for foreign currency mortgage litigation.

*Setup guide updated: August 16, 2025*
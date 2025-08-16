# Hungarian FX Mortgage Legal Research System

**System architected, designed, programmed & ideated by [https://www.bencium.io/](https://www.bencium.io/)**

## ⚠️ **IMPORTANT LEGAL DISCLAIMER**

**🚨 THIS SOFTWARE IS NOT FINANCIAL OR LEGAL ADVICE 🚨**

**By using this open-source software, you acknowledge and agree:**
- This system provides research assistance only, not professional legal or financial advice
- You use this software entirely at your own risk and responsibility  
- Bencium and contributors are NOT LIABLE for any damages, losses, or legal consequences
- All generated documents must be reviewed by qualified legal professionals before use
- This software does not guarantee legal outcomes or financial recovery
- You assume full responsibility for any legal actions based on this system's output

**⚖️ Professional Review Required:** Generated legal documents should always be reviewed by qualified legal professionals before use in actual legal proceedings.

---

## 🤖 **NEW: AI-Powered Case-Agnostic System**

🏛️ **Intelligent legal research assistant for ANY Hungarian foreign-currency mortgage case**

This system has been completely transformed from a hardcoded demo into a **production-ready, AI-powered legal research assistant** that can intelligently analyze ANY Hungarian FX mortgage case - regardless of bank, currency, or loan type.

## ✨ NEW AI-Powered Features

### 🧠 Intelligent Case Analysis
- **Universal Bank Support**: Works with ANY Hungarian bank (Erste, Aegon, OTP, K&H, CIB, Raiffeisen, UniCredit, etc.)
- **Multi-Currency Handling**: Analyzes CHF, EUR, USD, JPY, GBP, HUF loans automatically
- **Dynamic Document Processing**: AI understands any contract format or structure
- **Multi-Case Support**: Handles bank switching and multiple concurrent loans

### 🤖 AI-Driven Capabilities
- **Mistral AI Integration**: Advanced document understanding and legal analysis
- **Dynamic Damage Calculation**: AI calculates damages based on YOUR specific case facts
- **Personalized Legal Documents**: AI generates case-specific complaints and submissions
- **Real-Time File Monitoring**: Automatic reanalysis when new documents are added
- **User Override System**: Correct any AI misinterpretations with easy interface

## 🎯 Core Features

- **🔍 Document Analysis**: OCR processing of ANY mortgage documents via Mistral AI
- **⚖️ Legal Clause Extraction**: AI identifies violations specific to YOUR contracts
- **📚 Precedent Matching**: Intelligent matching against CJEU and Hungarian court cases
- **💰 Financial Calculator**: Case-specific damage calculations (no hardcoded values!)
- **📄 Legal Document Generation**: Personalized complaints, PBT submissions, lawyer letters
- **🌐 Bilingual Support**: Complete Hungarian and English language processing
- **🔒 Privacy-First**: Local deployment, your documents never leave your machine

## 🏗️ System Architecture

```
Hungarian FX Mortgage Legal Research System
├── 🤖 AI Analysis Engine (NEW)
│   ├── src/ai/           # AI-powered analysis
│   │   ├── mistral_client.rs  # Mistral AI integration
│   │   ├── case_analyzer.rs   # Intelligent case analysis
│   │   └── file_watcher.rs    # Real-time file monitoring
│   └── src/api/case_analysis.rs # Dynamic API endpoints
├── 🦀 Rust Backend
│   ├── src/api/          # REST API endpoints
│   ├── src/extractors/   # Contract clause extraction
│   ├── src/matching/     # Legal precedent matching
│   └── src/models/       # Data structures
├── 🐍 Python OCR Processor
│   ├── mistral_client.py # Mistral AI integration
│   └── main.py          # Document processing pipeline
├── 📊 Local Databases
│   ├── SQLite           # Case metadata and analysis
│   └── Chroma           # Vector similarity search
└── 🌐 Dynamic Web Interface # AI-powered UI
```

## 💾 Database Schema

**Local SQLite Database:**
- `legal_cases` - CJEU and national court precedents
- `documents` - Processed contract documents  
- `extracted_clauses` - Identified contract violations
- `precedent_matches` - Legal precedent similarity scores
- `financial_calculations` - Damage and restitution amounts
- `generated_reports` - Complete legal analysis packages

## 📋 Prerequisites

1. **🦀 Rust** (1.70+) - [Install from rustup.rs](https://rustup.rs/)
2. **🐍 Python** (3.8+) - For OCR document processing  
3. **🟢 Node.js** (18+) - **Required for local server** - [Download from nodejs.org](https://nodejs.org)
4. **🔑 Mistral API Key** - [Get from platform.mistral.ai](https://platform.mistral.ai)
5. **📄 Tesseract OCR** (Optional) - For fallback document processing

## 🚀 Quick Start

### 🤖 **AI-Powered System (NEW)**

```bash
# 1. Clone repository
git clone https://github.com/bencium/devizahitel-copilot.git
cd devizahitel-copilot

# 2. Configure Mistral API (REQUIRED for AI features)
cp .env.example .env
# Edit .env and add your Mistral API key

# 3. Start the AI-powered system
./start_ai.sh

# 4. Open http://localhost:8080 
# ✅ AI-powered interface ready for ANY case!
```

### 📖 **[Complete Setup Guide → SETUP.md](./SETUP.md)**

**For detailed instructions, troubleshooting, and advanced configuration, see [SETUP.md](./SETUP.md)**

## 🎯 How It Works - AI Workflow

### 1. Add Your Documents
```bash
# Place your documents in the OCR output folder
cp your_contracts/*.pdf ocr_output/
# System automatically detects new files!
```

### 2. AI Analysis Process
1. **Open**: `http://localhost:8080`
2. **Click**: "Analyze Case with AI" button
3. **AI Processing**:
   - 🤖 Identifies ALL banks and currencies in your case
   - 📄 Extracts loan amounts, dates, payment history
   - 💰 Calculates personalized damage amounts
   - ⚖️ Matches relevant legal precedents
4. **Review & Correct**: Override any AI misinterpretations
5. **Generate**: Download personalized legal documents

### 3. User Override System
- **Problem**: AI extracted wrong amount?
- **Solution**: Use Corrections tab to fix it
- **Result**: System immediately recalculates everything!

### 4. API Integration
```bash
# Health check
curl http://localhost:8080/health

# Get legal precedents  
curl http://localhost:8080/api/cases

# Process document
curl -X POST http://localhost:8080/api/research/analyze \
  -F "file=@contract.pdf" \
  -F "language=hu"
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

## 🏗️ Architecture & Development

**System Architecture, Design, Programming & Ideation by [Bencium.io](https://www.bencium.io/)**

This comprehensive legal research system represents innovative AI-first architecture combining:
- Advanced legal document processing with Mistral AI
- Sophisticated precedent matching algorithms
- Comprehensive financial damage calculation models
- Privacy-first local deployment strategy

## 📄 Legal Disclaimer

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
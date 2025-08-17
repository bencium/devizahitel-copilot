# Hungarian FX Mortgage Legal Research System - System Charter

**Single Source of Truth for Technical Stack, Privacy Modes, Database Choice, and Supported Jurisdictions**

---

## 🎯 **System Mission**

The Hungarian Foreign-Currency Mortgage Legal Research Assistant is an AI-powered tool designed to help lawyers efficiently analyze legal documents, extract contract clauses, match precedents, and calculate financial damages for 2006-2016 foreign-currency mortgages where banks failed to warn on exchange-rate risk.

**Core Business Domain**: Consumer protection and contract law (EU Directive 93/13/EEC unfair terms) focusing on CHF/EUR denominated mortgages with inadequate FX risk disclosure.

---

## 🏗️ **Technology Stack**

### Core Architecture
- **Backend**: Rust (primary), FastAPI (secondary)
- **Frontend**: Simple HTML UI for legal research interface
- **AI/ML**: Mistral AI integration, multilingual embeddings via sentence-transformers
- **OCR Processing**: Mistral OCR API for document processing

### Database Architecture
- **Primary Database**: Local SQLite for metadata and case tracking
- **Vector Database**: Chroma vector database for embeddings storage and similarity search
- **No Cloud Dependencies**: All processing and storage local only
- **No PostgreSQL**: System uses SQLite + Chroma exclusively

### Privacy & Security Modes
- **Local Processing**: All documents processed locally - no external data transmission
- **Privacy-First**: No cloud storage, SQLite and Chroma databases local only
- **API Security**: Mistral API calls use secure HTTPS for OCR/AI processing only
- **Data Protection**: Client documents never leave local system

---

## 🌍 **Supported Jurisdictions**

### Primary Coverage
- **EU/CJEU**: European Court of Justice precedents (C-630/23, C-186/16, C-26/13)
- **Hungary**: Kúria + lower courts, Hungarian Civil Code, Banking Act
- **Poland**: Supreme Court + appellate courts, Polish consumer protection law

### Secondary Coverage
- **Croatia**: Consumer protection for FX mortgages
- **Romania**: EU directive implementation cases
- **Spain**: "Multidivisa" mortgage precedents

### Legal Framework Priority
1. CJEU > National Supreme Courts > Appellate > First Instance
2. Recent 2025 precedents prioritized (C-630/23 AxFina ruling)
3. Cross-border precedent application for EU consumer protection

---

## 📊 **System Features & Capabilities**

### Document Processing
- **Multi-format Support**: PDF, Word, images, text files
- **OCR Integration**: Mistral OCR API + Tesseract fallback
- **Language Detection**: Hungarian, English, multilingual documents
- **Batch Processing**: Handle multiple documents simultaneously

### Legal Analysis Engine
- **Enhanced Clause Extraction** (9 types):
  - FX-risk disclosure failures
  - Exchange rate spread abuse
  - Broker liability violations (Banking Act 219/A-B§)
  - Insurance assignment breaches
  - Contract modification abuse (Civil Code 241§)
  - Consumer protection acknowledgments
  - Notarization clause abuse
  - Unilateral contract modifications
  - Fair dealing obligations

### Financial Harm Calculator
**PLEADABLE DAMAGES (Court-Ready with Legal Precedent):**
- Exchange rate loss vs. MNB central rates
- Professional costs (legal, accounting, appraisal)
- Administrative costs (documented expenses)
- Broker commission damages (Banking Act violations)
- Insurance assignment penalties (Civil Code breaches)
- Contract modification fee recovery
- Court and legal proceeding costs
- Property foreclosure losses

**EXPLORATORY DAMAGES (Research/Negotiation - Jurisdiction Dependent):**
- Lost interest calculations (compound interest over decades)
- Opportunity cost damages (economic theory)
- Inflation adjustments (requires specific legal basis)
- Credit rating impact assessment
- Moral/emotional damages (varies by jurisdiction)
- Future financial planning costs

### Precedent Matching System
- **Hybrid Retrieval**: BM25 + embedding similarity search
- **Weighted Scoring**: Jurisdiction hierarchy + legal issue alignment
- **Citation Accuracy**: Zero tolerance for hallucinated legal references
- **Evidence-Based Analysis**: No arbitrary case strength scores

---

## 🔧 **Deployment Configuration**

### Local Development Setup
```bash
# Technology Requirements
- Rust 1.70+ (primary language)
- Python 3.8+ (OCR processor)
- Node.js 18+ (local server)
- SQLite 3 (database)

# API Requirements
- Mistral API key (AI processing)
- No other external dependencies

# Database Initialization
DATABASE_URL=sqlite://legal_research.db
```

### Build Commands
- `cargo run`: Run the legal research assistant
- `cargo test`: Run test suite
- `cargo run --bin process_documents`: Process legal documents with OCR
- `cargo run --bin extract_clauses`: Extract contract clauses

### Offline/Online Behavior
- **Offline Capable**: All core analysis functions work without internet
- **Online Requirements**: Mistral API for OCR processing and AI analysis only
- **Hybrid Mode**: Critical functions maintain offline capability with degraded features

---

## ⚖️ **Legal Compliance & Professional Responsibility**

### Tool Boundaries
- **Research Assistance Only**: No automated legal advice
- **Lawyer Supervision Required**: All outputs require professional review
- **Prominent Disclaimers**: Clear tool limitation warnings
- **Audit Trail**: Complete logging of queries and outputs

### Data Handling Standards
- **Attorney-Client Privilege**: All client documents treated as confidential
- **Citation Accuracy**: Manual verification required for all legal precedents
- **Multilingual Accuracy**: Preserve original language legal terms
- **Data Retention**: Follow legal practice data retention requirements

### Success Metrics
- **Precedent Recall@10**: ≥ 0.8 on curated gold set
- **Clause Extraction Precision**: ≥ 0.9 for FX-risk disclosure clauses
- **Time to Research Packet**: ≤ 30 minutes per client
- **Zero Hallucinated Citations**: In top-10 results

---

## 🚀 **Operational Workflows**

### Daily Usage Pattern
1. **Start System**: `cargo run --bin devizahitel_legal_research`
2. **Process Documents**: Upload contracts to `ocr_output/` folder
3. **AI Analysis**: Click "Analyze Case with AI" in web interface
4. **Review Results**: Validate extracted clauses and precedent matches
5. **Generate Reports**: Download legal packages for court/negotiation use

### Document Types Supported
- **Erste Bank Contracts**: CHF-denominated mortgage contracts (2006 focus)
- **Aegon Contracts**: EUR-denominated mortgage contracts
- **Property Documents**: Land registry, valuations
- **Legal Correspondence**: Email communications with banks
- **Supporting Documents**: Insurance policies, property transfers

### Output Generation
- **Legal Complaint Drafts** (Hungarian/English)
- **Evidence Summaries** with clause extractions
- **Precedent Citations** with paragraph references
- **Financial Damage Reports** with detailed calculations
- **Settlement Negotiation Packages**

---

## 📈 **Performance & Quality Standards**

### Processing Benchmarks
- **Document Processing**: <5 minutes for typical case (10-15 documents)
- **Legal Analysis**: <3 minutes for complete analysis
- **Precedent Search**: <500ms response time
- **Report Generation**: <1 minute for complete legal package

### Quality Assurance
- **Legal Research Accuracy**: 90%+ precision on violation detection
- **Precedent Relevance**: 85%+ relevance score for top 5 matches
- **Citation Verification**: 100% verification against official sources
- **Financial Calculations**: ±2% accuracy for damage estimates

---

## 🔮 **System Evolution & Updates**

### Current Version (V1)
- ✅ Core legal research and analysis
- ✅ AI-powered document processing
- ✅ Financial damage calculations
- ✅ Legal document generation

### Planned Enhancements (V2)
- Cross-encoder re-ranker for improved relevance
- Per-paragraph pin-cite extraction
- Timeline visualization features
- Batch processing for multi-client workflows

### Maintenance Schedule
- **Quarterly**: Legal precedent database updates
- **Monthly**: Performance optimization reviews
- **Weekly**: Error analysis with legal counsel
- **As-needed**: Response to new CJEU/national court rulings

---

## 📞 **Support & Governance**

### Technical Support
- **Documentation**: Complete setup guides in `SETUP.md`
- **Issue Tracking**: GitHub issues for bug reports
- **Performance Monitoring**: Built-in system health checks

### Legal Review Process
- **Weekly Error Review**: With qualified legal counsel
- **Precedent Validation**: Against official court sources
- **Output Quality Control**: Lawyer approval for all generated documents

### Risk Management
- **False Certainty Mitigation**: Show rationale and uncertainty indicators
- **Translation Accuracy**: Language-specific patterns, source text preservation
- **Scope Control**: Hard UI limits, no unauthorized long-form outputs

---

**System Charter Version**: 1.0  
**Last Updated**: August 17, 2025  
**Next Review**: Completion of first 10 real cases  

*This charter serves as the definitive technical and operational reference for the Hungarian FX Mortgage Legal Research System.*
# Local Server Setup Guide / Helyi Szerver Telepítési Útmutató

## Quick Start / Gyors Indítás

### English Instructions

#### Prerequisites
- Rust installed: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Python 3.8+ installed (for OCR processing)
- Git installed

#### Step 1: Clone and Build
```bash
# Clone the repository (if open source)
git clone <repository-url>
cd legal-fx-research-assistant

# Or if developing locally, navigate to project directory
cd /Users/bencium/_devizahitel

# Build the Rust backend
cargo build --release
```

#### Step 2: Install Dependencies
```bash
# Install Python dependencies for OCR
cd mistral_ocr_processor
pip install -r requirements.txt
cd ..

# Download required language models
cargo run --bin download_models
```

#### Step 3: Initialize Database
```bash
# Create local SQLite database and Chroma vector store
cargo run --bin init_db

# Import any existing precedent documents
cargo run --bin import_precedents --path ./Precedents/
```

#### Step 4: Start the Server
```bash
# Start the local web server
cargo run --release

# Or for development with hot reload
cargo run
```

#### Step 5: Access the Web Interface
```
Open your browser and navigate to:
http://localhost:8080

The interface will show:
- Document upload area
- Case analysis dashboard  
- Financial calculator
- Action steps guidance
- Export functions
```

### Magyar Útmutató

#### Előfeltételek
- Rust telepítése: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Python 3.8+ telepítése (OCR feldolgozáshoz)
- Git telepítése

#### 1. Lépés: Klónozás és Fordítás
```bash
# Repository klónozása (ha nyílt forráskódú)
git clone <repository-url>
cd legal-fx-research-assistant

# Vagy ha helyben fejleszti, navigáljon a projekt könyvtárba
cd /Users/bencium/_devizahitel

# Rust backend fordítása
cargo build --release
```

#### 2. Lépés: Függőségek Telepítése
```bash
# Python függőségek telepítése OCR-hez
cd mistral_ocr_processor
pip install -r requirements.txt
cd ..

# Szükséges nyelvi modellek letöltése
cargo run --bin download_models
```

#### 3. Lépés: Adatbázis Inicializálása
```bash
# Helyi SQLite adatbázis és Chroma vektor tároló létrehozása
cargo run --bin init_db

# Meglévő precedens dokumentumok importálása
cargo run --bin import_precedents --path ./Precedents/
```

#### 4. Lépés: Szerver Indítása
```bash
# Helyi webszerver indítása
cargo run --release

# Vagy fejlesztéshez automatikus újratöltéssel
cargo run
```

#### 5. Lépés: Webes Felület Elérése
```
Nyissa meg a böngészőjét és navigáljon ide:
http://localhost:8080

A felület megjeleníti:
- Dokumentum feltöltési területet
- Ügy elemzési irányítópultot
- Pénzügyi kalkulátort
- Akció lépések útmutatót
- Export funkciókat
```

---

## Server Configuration / Szerver Konfiguráció

### Environment Variables / Környezeti Változók
Create a `.env` file in the project root:
```env
# Server configuration
SERVER_HOST=127.0.0.1
SERVER_PORT=8080

# Database paths
SQLITE_DB_PATH=./data/cases.db
CHROMA_DB_PATH=./data/chroma_db

# Exchange rate API
EXCHANGE_RATE_API_URL=https://api.exchangerate-api.com/v4/latest/

# OCR settings
OCR_PROCESSOR_PATH=./mistral_ocr_processor/cli.py
OCR_OUTPUT_PATH=./ocr_output

# Logging
LOG_LEVEL=info
LOG_FILE=./logs/server.log
```

### Port Configuration / Port Konfiguráció
If port 8080 is busy, change the port:
```bash
# Use different port
export SERVER_PORT=3000
cargo run --release

# Server will be available at http://localhost:3000
```

### Memory Settings / Memória Beállítások
For large document collections:
```bash
# Increase memory limits
export RUST_MAX_HEAP=4G
export CHROMA_MAX_MEMORY=2G
cargo run --release
```

---

## Development Mode / Fejlesztői Mód

### Hot Reload for Frontend
```bash
# Install cargo-watch for auto-rebuild
cargo install cargo-watch

# Start with hot reload
cargo watch -x run
```

### Debug Logging / Debug Naplózás
```bash
# Enable detailed logging
export RUST_LOG=debug
cargo run

# View logs in real-time
tail -f ./logs/server.log
```

### Database Reset / Adatbázis Visszaállítás
```bash
# Clear all data and reinitialize
rm -rf ./data/
cargo run --bin init_db
```

---

## Web Interface Features / Webes Felület Funkciói

### Main Dashboard / Fő Irányítópult
- **Case Overview**: Active cases and their status
- **Quick Actions**: Upload documents, start new analysis
- **Recent Activity**: Latest searches and calculations
- **System Status**: Database and OCR processor status

### Document Management / Dokumentum Kezelés
- **Upload Area**: Drag-and-drop for PDFs and images
- **OCR Status**: Real-time processing progress
- **Document Library**: Browse and search uploaded files
- **Metadata Editor**: Add case information and tags

### Legal Analysis / Jogi Elemzés
- **Clause Extraction**: Automated contract clause identification
- **Precedent Search**: Similar cases from local database
- **Similarity Scores**: Ranked relevance results
- **Legal Arguments**: Generated argument outlines

### Financial Calculator / Pénzügyi Kalkulátor
- **Loan Details Input**: Currency, amounts, dates
- **Payment History**: Upload or manual entry
- **Damage Categories**: All claim types with calculations
- **Export Options**: Excel, PDF, legal-ready formats

### Action Steps / Akció Lépések
- **Bilingual Interface**: Hungarian/English toggle
- **Progress Tracking**: Completed vs pending tasks
- **Deadline Alerts**: Time-sensitive reminders
- **Contact Integration**: Lawyer and court information

---

## API Endpoints / API Végpontok

### Core Endpoints
```
GET  /api/status          - Server health check
POST /api/upload          - Document upload
GET  /api/cases           - List all cases
POST /api/analyze         - Run legal analysis
GET  /api/precedents      - Search precedents
POST /api/calculate       - Financial calculations
GET  /api/export/:format  - Export reports
```

### Example Usage / Példa Használat
```bash
# Check server status
curl http://localhost:8080/api/status

# Upload document
curl -X POST -F "file=@contract.pdf" http://localhost:8080/api/upload

# Search precedents
curl -X GET "http://localhost:8080/api/precedents?query=CHF+loan+disclosure"
```

---

## Troubleshooting / Hibaelhárítás

### Common Issues / Gyakori Problémák

#### Server Won't Start / Szerver Nem Indul
```bash
# Check if port is in use
lsof -i :8080

# Kill process using port
kill -9 $(lsof -t -i:8080)

# Try different port
export SERVER_PORT=3000
cargo run --release
```

#### Database Connection Error / Adatbázis Kapcsolati Hiba
```bash
# Reset database
rm -rf ./data/cases.db
cargo run --bin init_db

# Check permissions
chmod 755 ./data/
chmod 644 ./data/cases.db
```

#### OCR Processing Fails / OCR Feldolgozás Sikertelen
```bash
# Check Python environment
cd mistral_ocr_processor
python --version
pip list | grep mistral

# Reinstall dependencies
pip install -r requirements.txt --force-reinstall
```

#### Out of Memory / Memória Elfogyott
```bash
# Reduce batch size for large documents
export CHROMA_BATCH_SIZE=100
export OCR_BATCH_SIZE=10
cargo run --release
```

### Performance Optimization / Teljesítmény Optimalizálás

#### For Large Document Collections
```bash
# Enable multi-threading
export RAYON_NUM_THREADS=4

# Increase database cache
export SQLITE_CACHE_SIZE=10000

# Use release build for production
cargo build --release
cargo run --release
```

#### Memory Management
```bash
# Monitor memory usage
htop

# Limit vector database memory
export CHROMA_MAX_MEMORY=1G

# Clear temporary files regularly
find ./temp/ -type f -mtime +7 -delete
```

---

## Security Considerations / Biztonsági Megfontolások

### Local Access Only / Csak Helyi Hozzáférés
```bash
# Bind to localhost only (default)
export SERVER_HOST=127.0.0.1

# For network access (use carefully)
export SERVER_HOST=0.0.0.0
```

### File Permissions / Fájl Jogosultságok
```bash
# Secure data directory
chmod 700 ./data/
chmod 600 ./data/cases.db

# Secure configuration
chmod 600 .env
```

### Data Backup / Adatmentés
```bash
# Backup database
cp ./data/cases.db ./backups/cases_$(date +%Y%m%d).db

# Backup vector database
tar -czf ./backups/chroma_$(date +%Y%m%d).tar.gz ./data/chroma_db/
```

---

*This server runs entirely locally with no external dependencies except the free exchange rate API. All sensitive legal documents remain on your machine.*
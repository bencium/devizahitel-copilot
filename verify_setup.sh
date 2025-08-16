#!/bin/bash

# Hungarian FX Mortgage Legal Research System - Setup Verification Script
# Verifies that all components are properly installed and configured

echo "🏛️ Hungarian FX Mortgage Legal Research System - Setup Verification"
echo "=================================================================="

# Color codes for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Track overall status
ERRORS=0

echo ""
echo "🔧 Checking System Prerequisites..."

# Check Rust installation
echo -n "Checking Rust installation... "
if command -v rustc &> /dev/null; then
    RUST_VERSION=$(rustc --version)
    echo -e "${GREEN}✅ Found: $RUST_VERSION${NC}"
else
    echo -e "${RED}❌ Rust not found. Install from https://rustup.rs/${NC}"
    ERRORS=$((ERRORS + 1))
fi

# Check Cargo
echo -n "Checking Cargo... "
if command -v cargo &> /dev/null; then
    CARGO_VERSION=$(cargo --version)
    echo -e "${GREEN}✅ Found: $CARGO_VERSION${NC}"
else
    echo -e "${RED}❌ Cargo not found${NC}"
    ERRORS=$((ERRORS + 1))
fi

# Check Python
echo -n "Checking Python 3... "
if command -v python3 &> /dev/null; then
    PYTHON_VERSION=$(python3 --version)
    echo -e "${GREEN}✅ Found: $PYTHON_VERSION${NC}"
else
    echo -e "${RED}❌ Python 3 not found${NC}"
    ERRORS=$((ERRORS + 1))
fi

# Check pip
echo -n "Checking pip... "
if command -v pip &> /dev/null || command -v pip3 &> /dev/null; then
    echo -e "${GREEN}✅ Found${NC}"
else
    echo -e "${RED}❌ pip not found${NC}"
    ERRORS=$((ERRORS + 1))
fi

echo ""
echo "📁 Checking Project Structure..."

# Check essential files
REQUIRED_FILES=(
    "Cargo.toml"
    "src/main.rs"
    "mistral_ocr_processor/main.py"
    "mistral_ocr_processor/requirements.txt"
    ".env.example"
    "static/index.html"
)

for file in "${REQUIRED_FILES[@]}"; do
    echo -n "Checking $file... "
    if [ -f "$file" ]; then
        echo -e "${GREEN}✅ Found${NC}"
    else
        echo -e "${RED}❌ Missing${NC}"
        ERRORS=$((ERRORS + 1))
    fi
done

echo ""
echo "🔑 Checking Environment Configuration..."

# Check .env file
echo -n "Checking .env file... "
if [ -f ".env" ]; then
    echo -e "${GREEN}✅ Found${NC}"
    
    # Check for Mistral API key
    echo -n "Checking MISTRAL_API_KEY... "
    if grep -q "MISTRAL_API_KEY=" .env && ! grep -q "MISTRAL_API_KEY=$" .env && ! grep -q "MISTRAL_API_KEY=your_" .env; then
        echo -e "${GREEN}✅ Configured${NC}"
    else
        echo -e "${YELLOW}⚠️  Not configured (add your Mistral API key)${NC}"
        ERRORS=$((ERRORS + 1))
    fi
else
    echo -e "${YELLOW}⚠️  Missing (copy from .env.example)${NC}"
    ERRORS=$((ERRORS + 1))
fi

# Check OCR processor .env
echo -n "Checking mistral_ocr_processor/.env... "
if [ -f "mistral_ocr_processor/.env" ]; then
    echo -e "${GREEN}✅ Found${NC}"
else
    echo -e "${YELLOW}⚠️  Missing (copy from mistral_ocr_processor/.env.example)${NC}"
    ERRORS=$((ERRORS + 1))
fi

echo ""
echo "📦 Checking Dependencies..."

# Check if Rust project builds
echo -n "Testing Rust compilation... "
if cargo check &> /dev/null; then
    echo -e "${GREEN}✅ Success${NC}"
else
    echo -e "${RED}❌ Failed (run 'cargo build' for details)${NC}"
    ERRORS=$((ERRORS + 1))
fi

# Check Python dependencies
echo -n "Checking Python dependencies... "
cd mistral_ocr_processor
if python3 -c "import requests, pathlib, argparse" &> /dev/null; then
    echo -e "${GREEN}✅ Core dependencies available${NC}"
else
    echo -e "${RED}❌ Missing dependencies (run 'pip install -r requirements.txt')${NC}"
    ERRORS=$((ERRORS + 1))
fi
cd ..

echo ""
echo "🗄️ Checking Database Setup..."

# Check if database directory exists
echo -n "Checking database directory... "
if [ -d "databases" ] || [ -f "legal_research.db" ]; then
    echo -e "${GREEN}✅ Database files present${NC}"
else
    echo -e "${YELLOW}⚠️  No database found (will be created on first run)${NC}"
fi

# Check migrations
echo -n "Checking migration files... "
if [ -d "migrations" ] && [ -f "migrations/001_initial_schema.sql" ]; then
    echo -e "${GREEN}✅ Found${NC}"
else
    echo -e "${RED}❌ Migration files missing${NC}"
    ERRORS=$((ERRORS + 1))
fi

echo ""
echo "🧪 Testing Core Components..."

# Test Mistral API connection (if configured)
if [ -f ".env" ] && grep -q "MISTRAL_API_KEY=" .env && ! grep -q "MISTRAL_API_KEY=$" .env; then
    echo -n "Testing Mistral API connection... "
    cd mistral_ocr_processor
    if python3 -c "
import os
import sys
sys.path.append('.')
try:
    from mistral_client import MistralOCRClient
    client = MistralOCRClient()
    print('API connection test passed')
except Exception as e:
    print(f'API connection failed: {e}')
    sys.exit(1)
    " &> /dev/null; then
        echo -e "${GREEN}✅ Connection successful${NC}"
    else
        echo -e "${YELLOW}⚠️  Connection failed (check API key)${NC}"
    fi
    cd ..
else
    echo -e "${YELLOW}⚠️  Skipping API test (no API key configured)${NC}"
fi

echo ""
echo "📊 Verification Summary"
echo "======================"

if [ $ERRORS -eq 0 ]; then
    echo -e "${GREEN}🎉 All checks passed! Your system is ready to use.${NC}"
    echo ""
    echo "Next steps:"
    echo "1. Run: cargo run"
    echo "2. Open: http://localhost:8080"
    echo "3. Upload a contract document to test"
else
    echo -e "${RED}❌ Found $ERRORS issue(s) that need attention.${NC}"
    echo ""
    echo "Please fix the issues above, then run this script again."
    echo "For detailed setup instructions, see: SETUP.md"
fi

echo ""
echo "📖 Resources:"
echo "- Setup Guide: SETUP.md"  
echo "- GitHub: https://github.com/bencium/devizahitel-copilot"
echo "- Architecture: https://www.bencium.io/"

exit $ERRORS
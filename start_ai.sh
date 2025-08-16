#!/bin/bash

# Hungarian FX Mortgage Legal Research System - AI Version Launcher
# This script starts the AI-powered case-agnostic version

echo "🤖 Hungarian FX Mortgage Legal Research System - AI Version"
echo "============================================================"
echo ""

# Check if .env file exists
if [ ! -f .env ]; then
    echo "❌ Error: .env file not found"
    echo "Please copy .env.example to .env and configure your Mistral API key"
    exit 1
fi

# Check if Mistral API key is set
if ! grep -q "MISTRAL_API_KEY=" .env || grep -q "MISTRAL_API_KEY=your_mistral_api_key_here" .env; then
    echo "❌ Error: Mistral API key not configured"
    echo "Please set your MISTRAL_API_KEY in the .env file"
    exit 1
fi

# Check OCR output directory
if [ ! -d "ocr_output" ]; then
    echo "⚠️  Warning: ocr_output directory not found, creating it..."
    mkdir -p ocr_output
fi

echo "🔍 Checking OCR documents..."
DOC_COUNT=$(find ocr_output -name "*.md" -o -name "*.txt" -o -name "*.rtf" | wc -l)
echo "📄 Found $DOC_COUNT documents in ocr_output folder"

if [ $DOC_COUNT -eq 0 ]; then
    echo "⚠️  Warning: No documents found in ocr_output folder"
    echo "   Please add your scanned/OCR'd documents before starting analysis"
fi

echo ""
echo "🚀 Starting AI-powered server..."
echo "📖 Access the interface at: http://127.0.0.1:8080"
echo "🤖 Features enabled:"
echo "   ✅ Case-agnostic document analysis"
echo "   ✅ Multi-bank and multi-currency support"
echo "   ✅ AI-powered damage calculations"
echo "   ✅ Real-time file monitoring"
echo "   ✅ Legal document generation"
echo "   ✅ User override capabilities"
echo ""

# Load environment variables and start the AI server
export $(cat .env | grep -v '^#' | xargs)
cargo run --bin devizahitel_legal_research_ai
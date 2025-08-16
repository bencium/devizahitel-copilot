#!/usr/bin/env python3
"""
Simple test script to process OCR documents and test AI analysis
This bypasses the Rust compilation issues and tests the core AI functionality
"""
import os
import json
import requests
from pathlib import Path
import sqlite3
from datetime import datetime

# Load environment variables
from dotenv import load_dotenv
load_dotenv()

MISTRAL_API_KEY = os.getenv('MISTRAL_API_KEY')
MISTRAL_API_URL = 'https://api.mistral.ai/v1/chat/completions'

def test_mistral_api():
    """Test basic Mistral API connectivity"""
    print("🔍 Testing Mistral API connection...")
    
    headers = {
        'Authorization': f'Bearer {MISTRAL_API_KEY}',
        'Content-Type': 'application/json'
    }
    
    data = {
        'model': 'mistral-small-latest',
        'messages': [{'role': 'user', 'content': 'Say "API test successful"'}],
        'max_tokens': 50
    }
    
    try:
        response = requests.post(MISTRAL_API_URL, headers=headers, json=data)
        response.raise_for_status()
        result = response.json()
        content = result['choices'][0]['message']['content']
        print(f"✅ Mistral API Response: {content}")
        return True
    except Exception as e:
        print(f"❌ Mistral API Error: {e}")
        return False

def analyze_document_with_ai(document_content):
    """Analyze a document using Mistral AI"""
    print("🤖 Analyzing document with AI...")
    
    prompt = f"""
    Analyze this Hungarian legal document and extract the following information:
    
    1. Bank name (which bank issued this document)
    2. Loan amount and currency 
    3. Document type (contract, statement, correspondence, etc.)
    4. Key dates mentioned
    5. Any FX risk disclosure information
    6. Interest rates or fees mentioned
    
    Document content:
    {document_content[:2000]}...
    
    Respond in JSON format with these fields: bank_name, loan_amount, currency, document_type, key_dates, fx_risk_disclosure, rates_fees
    """
    
    headers = {
        'Authorization': f'Bearer {MISTRAL_API_KEY}',
        'Content-Type': 'application/json'
    }
    
    data = {
        'model': 'mistral-large-latest',
        'messages': [{'role': 'user', 'content': prompt}],
        'max_tokens': 500
    }
    
    try:
        response = requests.post(MISTRAL_API_URL, headers=headers, json=data)
        response.raise_for_status()
        result = response.json()
        analysis = result['choices'][0]['message']['content']
        print(f"🔍 AI Analysis Result: {analysis[:200]}...")
        return analysis
    except Exception as e:
        print(f"❌ AI Analysis Error: {e}")
        return None

def process_ocr_documents():
    """Process OCR documents and store results in database"""
    print("📄 Processing OCR documents...")
    
    ocr_dir = Path('ocr_output')
    if not ocr_dir.exists():
        print("❌ OCR output directory not found")
        return
    
    # Connect to SQLite database
    conn = sqlite3.connect('legal_research.db')
    cursor = conn.cursor()
    
    # Count documents
    md_files = list(ocr_dir.glob('*.md'))
    print(f"📊 Found {len(md_files)} OCR processed documents")
    
    # Process first 10 documents as test
    processed_count = 0
    for md_file in md_files[:10]:
        print(f"\n📝 Processing: {md_file.name}")
        
        try:
            # Read document content
            with open(md_file, 'r', encoding='utf-8') as f:
                content = f.read()
            
            print(f"📄 Document length: {len(content)} characters")
            
            # Analyze with AI
            ai_analysis = analyze_document_with_ai(content)
            
            if ai_analysis:
                # Try to parse JSON analysis
                try:
                    analysis_data = json.loads(ai_analysis.strip('```json').strip('```'))
                    print(f"✅ Parsed analysis: {analysis_data}")
                except:
                    print(f"⚠️  Could not parse JSON, raw analysis: {ai_analysis[:100]}...")
                    analysis_data = {"raw_analysis": ai_analysis}
                
                # Store in database (simplified)
                cursor.execute("""
                    INSERT OR REPLACE INTO documents 
                    (id, filename, content_type, file_size, extracted_text, document_type, 
                     language, processing_status, created_at, updated_at)
                    VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                """, (
                    md_file.stem,  # Use filename as ID for now
                    md_file.name,
                    'text/markdown',
                    len(content),
                    content,
                    analysis_data.get('document_type', 'unknown'),
                    'hu',
                    'ai_analyzed',
                    datetime.now().isoformat(),
                    datetime.now().isoformat()
                ))
                
                processed_count += 1
                print(f"✅ Stored document analysis in database")
                
        except Exception as e:
            print(f"❌ Error processing {md_file.name}: {e}")
    
    conn.commit()
    conn.close()
    
    print(f"\n📊 Successfully processed {processed_count} documents")
    return processed_count

def test_database_storage():
    """Test database storage and retrieval"""
    print("\n💾 Testing database storage...")
    
    conn = sqlite3.connect('legal_research.db')
    cursor = conn.cursor()
    
    # Check document count
    cursor.execute("SELECT COUNT(*) FROM documents")
    doc_count = cursor.fetchone()[0]
    print(f"📊 Documents in database: {doc_count}")
    
    # Check recent documents
    cursor.execute("""
        SELECT filename, document_type, processing_status, file_size 
        FROM documents 
        WHERE processing_status = 'ai_analyzed'
        LIMIT 5
    """)
    
    recent_docs = cursor.fetchall()
    print(f"🔍 Recent AI-analyzed documents:")
    for doc in recent_docs:
        print(f"  📄 {doc[0]} | Type: {doc[1]} | Size: {doc[3]} chars")
    
    conn.close()

def main():
    """Main test function"""
    print("🤖 Hungarian FX Mortgage Legal Research - Document Processing Test")
    print("=" * 70)
    
    # Test 1: API connectivity
    if not test_mistral_api():
        print("❌ Cannot proceed without API connectivity")
        return
    
    # Test 2: Process documents
    processed = process_ocr_documents()
    
    if processed > 0:
        # Test 3: Database verification
        test_database_storage()
        
        print("\n🎉 Document processing test completed successfully!")
        print(f"✅ Processed {processed} documents with AI analysis")
        print("✅ Data stored in SQLite database")
        print("✅ Ready for full system integration")
    else:
        print("❌ No documents were processed successfully")

if __name__ == "__main__":
    main()
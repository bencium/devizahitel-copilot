#!/bin/bash

# Test script for Legal Research System
echo "Testing Legal Research System..."

# Check if server is running
echo "1. Testing health endpoint..."
curl -s http://localhost:8080/health | jq .

echo -e "\n2. Testing precedent cases endpoint..."
curl -s http://localhost:8080/api/cases | jq '.cases | length'

echo -e "\n3. Testing case search..."
curl -s -X POST http://localhost:8080/api/cases/search \
  -H "Content-Type: application/json" \
  -d '{"query": "foreign currency", "limit": 3}' | jq '.results | length'

echo -e "\n4. Testing document upload with sample text..."
# Create a sample FX mortgage contract text
SAMPLE_TEXT="A hitelfelvevő tudomásul veszi, hogy a devizaalapú hitel esetében árfolyamkockázatot vállal. A bank nem nyújt garanciát az árfolyam alakulására. CHF alapú hitel esetében a svájci frank árfolyamának változása befolyásolja a törlesztőrészletek összegét."

# Convert to base64
ENCODED_TEXT=$(echo "$SAMPLE_TEXT" | base64)

# Upload document
UPLOAD_RESULT=$(curl -s -X POST http://localhost:8080/api/documents \
  -H "Content-Type: application/json" \
  -d "{
    \"filename\": \"test_contract.txt\",
    \"content_type\": \"text/plain\",
    \"file_data\": \"$ENCODED_TEXT\",
    \"document_type\": \"mortgage_contract\",
    \"language\": \"hu\"
  }")

echo "Document upload result:"
echo $UPLOAD_RESULT | jq .

# Extract document ID for further testing
DOCUMENT_ID=$(echo $UPLOAD_RESULT | jq -r '.document_id // empty')

if [ ! -z "$DOCUMENT_ID" ]; then
    echo -e "\n5. Testing precedent matching for document $DOCUMENT_ID..."
    curl -s -X POST http://localhost:8080/api/research/match-precedents \
      -H "Content-Type: application/json" \
      -d "{\"document_id\": \"$DOCUMENT_ID\"}" | jq '.overall_case_matches | length'
    
    echo -e "\n6. Testing draft pleading generation..."
    curl -s -X POST http://localhost:8080/api/research/generate-draft \
      -H "Content-Type: application/json" \
      -d "{
        \"document_id\": \"$DOCUMENT_ID\",
        \"pleading_type\": \"complaint\",
        \"court\": \"District Court\"
      }" | jq '.title'
      
    echo -e "\n7. Getting research sheet..."
    curl -s http://localhost:8080/api/research/sheet/$DOCUMENT_ID | jq '.analysis_summary'
fi

echo -e "\nSystem test completed!"
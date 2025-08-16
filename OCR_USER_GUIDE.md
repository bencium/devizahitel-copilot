# OCR Document Processing Guide / OCR Dokumentum Feldolgozási Útmutató

## Quick Start / Gyors kezdés

### English Instructions

#### Step 1: Prepare Your Documents
1. **Create a folder** called `documents_to_process` in your project directory
2. **Add all your files** to this folder:
   - PDF files (loan contracts, bank correspondence)
   - Image files (PNG, JPG, HEIC) of documents
   - Scanned documents in any common format

#### Step 2: Run OCR Processing
```bash
cd mistral_ocr_processor
python cli.py --input ../documents_to_process --output ../ocr_output
```

#### Step 3: Review Results
- Check the `ocr_output` folder for processed text files
- Each document will have a corresponding `.md` file with extracted text
- Review the processing log for any errors or warnings

### Magyar Útmutató

#### 1. Lépés: Dokumentumok Előkészítése
1. **Hozzon létre egy mappát** `feldolgozando_dokumentumok` néven a projekt könyvtárában
2. **Tegye bele az összes fájlt** ebbe a mappába:
   - PDF fájlok (hitelszerződések, banki levelezések)
   - Kép fájlok (PNG, JPG, HEIC) dokumentumokról
   - Szkennelt dokumentumok bármilyen formátumban

#### 2. Lépés: OCR Feldolgozás Futtatása
```bash
cd mistral_ocr_processor
python cli.py --input ../feldolgozando_dokumentumok --output ../ocr_kimenet
```

#### 3. Lépés: Eredmények Ellenőrzése
- Ellenőrizze az `ocr_kimenet` mappát a feldolgozott szöveges fájlokért
- Minden dokumentumhoz tartozik egy `.md` fájl a kinyert szöveggel
- Tekintse át a feldolgozási naplót esetleges hibákért vagy figyelmeztetésekért

---

## Advanced Options / Haladó Beállítások

### Batch Processing Large Collections
```bash
# Process specific file types only
python cli.py --input ../documents --output ../ocr_output --file-types pdf,png,jpg

# Process with verbose logging
python cli.py --input ../documents --output ../ocr_output --verbose

# Resume interrupted processing
python cli.py --input ../documents --output ../ocr_output --resume
```

### Quality Control Tips / Minőségellenőrzési Tippek

**For Best Results / A legjobb eredményekért:**
- Scan documents at 300 DPI minimum
- Ensure good lighting and contrast
- Keep documents flat and unfolded
- Remove any handwritten notes that might confuse OCR

**Common Issues / Gyakori problémák:**
- Blurry images: Rescan with better focus
- Skewed text: Use document scanner apps to auto-straighten
- Poor contrast: Adjust brightness/contrast before scanning

---

## File Organization / Fájl Szervezés

### Recommended Folder Structure / Ajánlott Mappa Struktúra
```
_devizahitel/
├── documents_to_process/
│   ├── erste_contracts/
│   ├── aegon_contracts/
│   ├── bank_correspondence/
│   └── property_documents/
├── ocr_output/
│   ├── erste_loan_2006.md
│   ├── aegon_loan_2010.md
│   └── processing_log.txt
└── mistral_ocr_processor/
    └── cli.py
```

### Naming Conventions / Elnevezési Konvenciók
- Use descriptive filenames: `erste_chf_loan_2006.pdf`
- Include dates when relevant: `bank_letter_2023_03_15.jpg`
- Avoid special characters and spaces in filenames
- Use consistent naming across similar documents

---

## Troubleshooting / Hibaelhárítás

### Common Problems / Gyakori Problémák

**OCR fails to start / OCR nem indul el:**
```bash
# Check Python environment
python --version
pip list | grep mistral

# Reinstall dependencies
pip install -r requirements.txt
```

**Poor text extraction / Gyenge szövegkinyerés:**
- Try preprocessing images with image editing software
- Increase scan resolution to 600 DPI for complex documents
- Split multi-page documents into individual files

**Processing takes too long / A feldolgozás túl sokáig tart:**
- Process smaller batches of files
- Close other applications to free up system resources
- Consider using a more powerful machine for large document sets

### Getting Help / Segítség Kérése

If you encounter issues:
1. Check the processing log file for error messages
2. Verify your document quality and format
3. Try processing a single test file first
4. Contact support with specific error messages and document types

---

*This OCR system uses Mistral AI for accurate document text extraction. The processed text will be used by the legal research assistant for contract analysis and precedent matching.*
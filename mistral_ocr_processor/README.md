# Mistral OCR PDF Processor

A minimal dependency Python application that automatically extracts structured text from PDF files using Mistral's OCR API and saves the results as markdown files.

## Features

- **Batch Processing**: Processes all PDF files in a directory recursively
- **Graceful Filename Handling**: Converts accented Hungarian filenames to safe English-only markdown filenames
- **Dual PDF Support**: Handles both text-based and image-based PDFs
- **Resume Functionality**: Can resume processing from where it left off
- **Comprehensive Logging**: Detailed processing logs and error handling
- **Progress Tracking**: Real-time progress display with file counts and statistics
- **Minimal Dependencies**: Only requires `requests`, `pathlib`, and `unidecode`

## Installation

1. **Clone or download the project**:
   ```bash
   cd mistral_ocr_processor
   ```

2. **Create and activate a Python virtual environment**:
   ```bash
   python3 -m venv venv
   source venv/bin/activate  # On Windows: venv\Scripts\activate
   ```

3. **Install dependencies**:
   ```bash
   pip install -r requirements.txt
   ```

4. **Set up your API key**:
   ```bash
   cp .env.example .env
   # Edit .env and add your Mistral API key:
   # MISTRAL_API_KEY=your_actual_api_key_here
   ```

## Usage

### Basic Usage

Process all PDFs in the current directory:
```bash
python main.py
```

### Advanced Usage

```bash
# Process specific directory
python main.py --input-dir "/path/to/pdfs" --output-dir "/path/to/output"

# Include image data in API requests (slower but more comprehensive)
python main.py --include-images

# Resume processing (skip already processed files)
python main.py --resume

# Enable verbose logging
python main.py --verbose

# Combine multiple options
python main.py --input-dir "./documents" --output-dir "./ocr_results" --resume --verbose
```

### Command Line Arguments

- `--input-dir`: Input directory containing PDFs (default: current directory)
- `--output-dir`: Output directory for markdown files (default: `./ocr_output`)
- `--include-images`: Include base64 image data in API requests
- `--resume`: Skip files that have already been processed
- `--verbose`: Enable detailed logging output

## Configuration

### Environment Variables

Create a `.env` file with your configuration:

```bash
# Required: Your Mistral API key
MISTRAL_API_KEY=your_api_key_here
```

### API Configuration

The application uses the following Mistral API settings:
- **Endpoint**: `https://api.mistral.ai/v1/ocr`
- **Model**: `mistral-ocr-latest`
- **Timeout**: 120 seconds per request
- **Retries**: Up to 3 attempts for failed requests

## Output

### Markdown Files

Each processed PDF is saved as a markdown file with:
- **Metadata header** with original filename, processing date, and file information
- **Structured content** preserving document hierarchy and formatting
- **English-only filenames** for cross-platform compatibility

Example output structure:
```
ocr_output/
├── aegon_aszf.md
├── erste_elidegen_torl2010.md
├── processing_summary.md
├── filename_mappings.txt
└── ocr_processing.log
```

### Processing Reports

- **`processing_summary.md`**: Complete processing statistics and results
- **`filename_mappings.txt`**: Mapping from original to sanitized filenames
- **`ocr_processing.log`**: Detailed processing logs for troubleshooting

## Filename Sanitization

The application automatically converts accented characters and special symbols:

| Original | Sanitized |
|----------|-----------|
| `aegon-ászf.pdf` | `aegon_aszf.md` |
| `Törökbálint vevők.pdf` | `Torokbalint_vevok.md` |
| `fogyasztóváltás.pdf` | `fogyasztovaltas.md` |

## Error Handling

- **File Size Validation**: Warns about large files (>10MB) and skips tiny files
- **API Rate Limiting**: Automatically handles rate limits with exponential backoff
- **Network Errors**: Retries failed requests up to 3 times
- **Resume Capability**: Can restart processing without losing progress
- **Detailed Logging**: All errors logged with context for troubleshooting

## Example Processing Session

```bash
$ python main.py --input-dir ./documents --verbose

2024-08-16 10:30:15 - INFO - Starting OCR processing
2024-08-16 10:30:15 - INFO - Input directory: /path/to/documents
2024-08-16 10:30:15 - INFO - Output directory: /path/to/ocr_output
2024-08-16 10:30:16 - INFO - Testing Mistral API connection...
2024-08-16 10:30:17 - INFO - API connection test successful
2024-08-16 10:30:17 - INFO - Found 15 PDF files to process
2024-08-16 10:30:18 - INFO - Processing file 1/15: aegon-ászf.pdf
2024-08-16 10:30:22 - INFO - Successfully processed: aegon-ászf.pdf -> aegon_aszf.md
...
2024-08-16 10:35:45 - INFO - Processing complete!
2024-08-16 10:35:45 - INFO - Successfully processed: 14 files
2024-08-16 10:35:45 - INFO - Errors encountered: 1 files
```

## Requirements

- **Python**: 3.7 or higher
- **API Key**: Valid Mistral API key with OCR access
- **Dependencies**: 
  - `requests >= 2.25.0`
  - `pathlib >= 1.0.1` 
  - `unidecode >= 1.3.0`

## Troubleshooting

### Common Issues

1. **API Key Error**: Ensure your `.env` file contains a valid `MISTRAL_API_KEY`
2. **Network Errors**: Check internet connection and firewall settings
3. **Large Files**: Files over 10MB may fail or take longer to process
4. **Empty Output**: Check the processing log for API errors or file format issues

### Logging

Enable verbose logging to debug issues:
```bash
python main.py --verbose
```

Check the generated log file:
```bash
tail -f ocr_processing.log
```

## License

This project is released under the MIT License.

## Support

For issues or questions:
1. Check the `ocr_processing.log` file for detailed error information
2. Verify your API key is valid and has OCR access
3. Ensure input files are valid PDF documents
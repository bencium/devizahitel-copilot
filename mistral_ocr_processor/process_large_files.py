#!/usr/bin/env python3
"""
Script to process the large failed files with increased size limit.
"""

import os
import shutil
import tempfile
from pathlib import Path
import subprocess
import sys

def main():
    # The specific large files we found
    large_files = {
        "aegon-vegtorl-scan.pdf": "/Users/bencium/_devizahitel/_pdf/törökbálint árnyas/aegon-vegtorl-scan.pdf",
        "megvett-ingatlan.pdf": "/Users/bencium/_devizahitel/_not needed skip/lakáshitel scan/1 lakásvétel 2006 márc-ápr/megvett-ingatlan.pdf"
    }
    
    print("Processing large files with increased size limit...")
    
    # Create temporary directory with just these files
    with tempfile.TemporaryDirectory() as temp_dir:
        temp_path = Path(temp_dir)
        print(f"Copying files to temporary directory: {temp_path}")
        
        for name, source_path in large_files.items():
            if os.path.exists(source_path):
                dest_path = temp_path / name
                shutil.copy2(source_path, dest_path)
                size_mb = os.path.getsize(source_path) / (1024 * 1024)
                print(f"  Copied: {name} ({size_mb:.1f} MB)")
            else:
                print(f"  ✗ File not found: {source_path}")
        
        # Create a custom config file that allows larger files
        config_content = '''
import os
from pathlib import Path
from typing import Optional
import logging

class Config:
    """Configuration class for OCR processing with increased file size limit."""
    
    def __init__(
        self,
        input_dir: Path,
        output_dir: Path,
        include_images: bool = False,
        resume_mode: bool = False,
        file_type_filter: Optional[str] = None
    ):
        self.input_dir = Path(input_dir).resolve()
        self.output_dir = Path(output_dir).resolve()
        self.include_images = include_images
        self.resume_mode = resume_mode
        self.file_type_filter = file_type_filter
        
        # Load environment variables
        self._load_env_file()
        self.mistral_api_key = self._get_api_key()
        
        # API configuration
        self.mistral_api_url = "https://api.mistral.ai/v1/ocr"
        self.mistral_model = "mistral-ocr-latest"
        
        # Processing configuration - INCREASED SIZE LIMIT
        self.max_file_size_mb = 25  # Increased from 10MB to 25MB
        self.request_timeout = 300  # Increased timeout for large files
        self.max_retries = 3
        self.retry_delay = 2
        
        # Define file type groups
        self.file_type_groups = {
            'pdf': {'.pdf'},
            'images': {'.jpg', '.jpeg', '.png', '.avif'},
            'documents': {'.pptx', '.docx'},
            'all': {'.pdf', '.jpg', '.jpeg', '.png', '.avif', '.pptx', '.docx'}
        }
        
        # Set supported extensions based on filter
        if file_type_filter:
            if file_type_filter in self.file_type_groups:
                self.supported_extensions = self.file_type_groups[file_type_filter]
            else:
                raise ValueError(f"Unknown file type filter: {file_type_filter}")
        else:
            self.supported_extensions = self.file_type_groups['all']
        
        # Logging setup
        self.setup_logging()
    
    def _load_env_file(self):
        """Load environment variables from .env file."""
        env_file = Path('.env')
        if env_file.exists():
            with open(env_file, 'r') as f:
                for line in f:
                    line = line.strip()
                    if line and not line.startswith('#') and '=' in line:
                        key, value = line.split('=', 1)
                        os.environ[key.strip()] = value.strip()
    
    def _get_api_key(self) -> str:
        """Get Mistral API key from environment."""
        api_key = os.getenv('MISTRAL_API_KEY')
        if not api_key:
            raise ValueError("MISTRAL_API_KEY not found in environment variables.")
        return api_key
    
    def get_processed_files(self) -> set:
        """Get set of already processed files for resume mode."""
        processed_files = set()
        if not self.output_dir.exists():
            return processed_files
        
        for md_file in self.output_dir.glob('*.md'):
            if md_file.name != 'processing_summary.md':
                processed_files.add(md_file.stem)
        
        return processed_files
    
    def setup_logging(self):
        """Setup logging configuration."""
        log_file = Path('ocr_processing.log')
        
        # Configure logging
        logging.basicConfig(
            level=logging.INFO,
            format='%(asctime)s - %(levelname)s - %(message)s',
            handlers=[
                logging.FileHandler(log_file, mode='a'),
                logging.StreamHandler()
            ]
        )
'''
        
        # Write the custom config
        config_path = temp_path / "config_large.py"
        with open(config_path, 'w') as f:
            f.write(config_content)
        
        # Run the processor with increased limits using fallback as needed
        print(f"\\nProcessing large files with 25MB limit...")
        
        # Use python -c to run inline script with custom config
        inline_script = f'''
import sys
sys.path.insert(0, "{temp_path}")
import config_large

# Import other modules normally
import sys
import os
sys.path.insert(0, "{os.getcwd()}")

from pathlib import Path
from filename_utils import FilenameSanitizer
from mistral_client import MistralOCRClient
from fallback_ocr import FallbackOCRClient
from output_manager import OutputManager
import time
import logging

# Setup
input_dir = Path("{temp_path}")
output_dir = Path("{os.getcwd()}/ocr_output_retry_large")
output_dir.mkdir(exist_ok=True)

# Use the custom config
config = config_large.Config(
    input_dir=input_dir,
    output_dir=output_dir,
    include_images=False,
    resume_mode=False,
    file_type_filter=None
)

print(f"Max file size limit: {{config.max_file_size_mb}} MB")
print(f"Request timeout: {{config.request_timeout}} seconds")

# Initialize components  
sanitizer = FilenameSanitizer()
mistral_client = MistralOCRClient(
    api_key=config.mistral_api_key,
    api_url=config.mistral_api_url,
    model=config.mistral_model
)
fallback_client = FallbackOCRClient()
output_manager = OutputManager(config.output_dir)

# Find files
pdf_files = list(input_dir.glob("*.pdf"))
print(f"Found {{len(pdf_files)}} PDF files to process")

for i, pdf_file in enumerate(pdf_files, 1):
    print(f"\\nProcessing file {{i}}/{{len(pdf_files)}}: {{pdf_file.name}}")
    
    # Check file size
    file_size_mb = pdf_file.stat().st_size / (1024 * 1024)
    print(f"File size: {{file_size_mb:.1f}} MB")
    
    if file_size_mb > config.max_file_size_mb:
        print(f"File too large ({{file_size_mb:.1f}} MB > {{config.max_file_size_mb}} MB)")
        continue
    
    # Generate sanitized filename
    sanitized_name = sanitizer.sanitize_filename(pdf_file)
    
    # Try processing with fallback
    response = None
    content = None
    processor_used = "Unknown"
    
    try:
        # Try Mistral first
        print("  Attempting Mistral OCR...")
        response = mistral_client.process_file(
            file_path=pdf_file,
            include_images=False,
            max_retries=2,  # Reduced retries for large files
            retry_delay=3,
            timeout=config.request_timeout
        )
        
        if response:
            content = mistral_client.extract_text_from_response(response)
            processor_used = "Mistral OCR"
            print("  ✅ Mistral OCR successful")
    except Exception as e:
        print(f"  ⚠️ Mistral OCR failed: {{e}}")
    
    # Try fallback if Mistral failed
    if not content:
        print("  Attempting fallback OCR (Tesseract)...")
        try:
            response = fallback_client.process_file(
                file_path=pdf_file,
                include_images=False
            )
            
            if response:
                content = fallback_client.extract_text_from_response(response)
                processor_used = "Tesseract OCR (Fallback)"
                print("  ✅ Fallback OCR successful")
        except Exception as e:
            print(f"  ❌ Fallback OCR also failed: {{e}}")
    
    # Save results
    if content:
        success = output_manager.save_markdown(
            content=content,
            sanitized_filename=sanitized_name,
            original_filename=pdf_file.name,
            metadata={{
                'file_type': pdf_file.suffix.lower(),
                'file_size_mb': round(file_size_mb, 2),
                'include_images': False,
                'processor_used': processor_used
            }}
        )
        
        if success:
            print(f"  📄 Saved: {{sanitized_name}}.md (using {{processor_used}})")
        else:
            print(f"  ❌ Failed to save: {{sanitized_name}}.md")
    else:
        print(f"  ❌ No content extracted from {{pdf_file.name}}")

print("\\n🎉 Large file processing complete!")
'''
        
        # Execute the inline script
        result = subprocess.run([
            sys.executable, "-c", inline_script
        ], cwd=os.getcwd())
        
        if result.returncode == 0:
            print("\n✅ Large file processing completed successfully!")
        else:
            print(f"\n❌ Large file processing failed with return code: {result.returncode}")

if __name__ == "__main__":
    main()
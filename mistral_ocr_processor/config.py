"""
Configuration management for Mistral OCR processor.
"""

import os
from pathlib import Path
from typing import Optional
import logging


class Config:
    """Configuration class for OCR processing."""
    
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
        
        # Processing configuration
        self.max_file_size_mb = 10  # Maximum file size in MB
        self.request_timeout = 120  # Request timeout in seconds
        self.max_retries = 3
        self.retry_delay = 2  # Seconds between retries
        
        # Define file type groups
        self.file_type_groups = {
            'pdf': {'.pdf'},
            'images': {'.jpg', '.jpeg', '.png', '.avif'},
            'documents': {'.pptx', '.docx'},
            'all': {'.pdf', '.jpg', '.jpeg', '.png', '.avif', '.pptx', '.docx'}
        }
        
        # Set supported extensions based on filter
        if self.file_type_filter and self.file_type_filter in self.file_type_groups:
            self.supported_extensions = self.file_type_groups[self.file_type_filter]
        else:
            self.supported_extensions = self.file_type_groups['all']
        
        # Validate configuration
        self._validate()
        
        # Ensure output directory exists
        self.output_dir.mkdir(parents=True, exist_ok=True)
    
    def _load_env_file(self) -> None:
        """Load environment variables from .env file."""
        env_file = Path('.env')
        if env_file.exists():
            with open(env_file, 'r', encoding='utf-8') as f:
                for line in f:
                    line = line.strip()
                    if line and not line.startswith('#'):
                        if '=' in line:
                            key, value = line.split('=', 1)
                            os.environ[key.strip()] = value.strip()
    
    def _get_api_key(self) -> str:
        """Get Mistral API key from environment."""
        api_key = os.getenv('MISTRAL_API_KEY')
        if not api_key:
            raise ValueError(
                "MISTRAL_API_KEY not found in environment variables. "
                "Please add it to your .env file: MISTRAL_API_KEY=your_key_here"
            )
        return api_key
    
    def _validate(self) -> None:
        """Validate configuration settings."""
        logger = logging.getLogger(__name__)
        
        if not self.input_dir.exists():
            raise ValueError(f"Input directory does not exist: {self.input_dir}")
        
        if not self.input_dir.is_dir():
            raise ValueError(f"Input path is not a directory: {self.input_dir}")
        
        # Check for supported files in input directory
        supported_files = []
        for ext in self.supported_extensions:
            # Add case variations
            patterns = [f"*{ext}", f"*{ext.upper()}"]
            for pattern in patterns:
                supported_files.extend(list(self.input_dir.rglob(pattern)))
        
        if not supported_files:
            logger.warning(f"No supported files found in {self.input_dir}")
            logger.info(f"Supported formats: {', '.join(self.supported_extensions)}")
        else:
            # Count by type
            file_counts = {}
            for file_path in supported_files:
                ext = file_path.suffix.lower()
                file_counts[ext] = file_counts.get(ext, 0) + 1
            
            logger.info(f"Found {len(supported_files)} supported files to process:")
            for ext, count in sorted(file_counts.items()):
                logger.info(f"  {ext}: {count} files")
    
    def get_processed_files(self) -> set:
        """Get set of already processed files (for resume functionality)."""
        processed = set()
        if self.resume_mode and self.output_dir.exists():
            for md_file in self.output_dir.rglob("*.md"):
                # Extract original filename from metadata or filename
                processed.add(md_file.stem)
        return processed
    
    def __str__(self) -> str:
        """String representation of configuration."""
        return (
            f"Config(\n"
            f"  input_dir={self.input_dir}\n"
            f"  output_dir={self.output_dir}\n"
            f"  include_images={self.include_images}\n"
            f"  resume_mode={self.resume_mode}\n"
            f"  api_key={'***' + self.mistral_api_key[-4:] if self.mistral_api_key else 'None'}\n"
            f")"
        )
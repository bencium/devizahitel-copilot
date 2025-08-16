"""
PDF processing engine that coordinates OCR processing of PDF files.
"""

import logging
import time
from pathlib import Path
from typing import Tuple, List

from config import Config
from filename_utils import FilenameSanitizer
from mistral_client import MistralOCRClient
from fallback_ocr import FallbackOCRClient
from output_manager import OutputManager


class PDFProcessor:
    """Main processor for handling PDF OCR operations."""
    
    def __init__(self, config: Config):
        self.config = config
        self.logger = logging.getLogger(__name__)
        
        # Initialize components
        self.sanitizer = FilenameSanitizer()
        self.mistral_client = MistralOCRClient(
            api_key=config.mistral_api_key,
            api_url=config.mistral_api_url,
            model=config.mistral_model
        )
        self.fallback_client = FallbackOCRClient()
        self.output_manager = OutputManager(config.output_dir)
        
        # Processing counters
        self.successful_count = 0
        self.error_count = 0
        self.skipped_count = 0
        self.error_files = []
    
    def process_directory(self) -> Tuple[int, int]:
        """
        Process all PDF files in the configured directory.
        
        Returns:
            Tuple of (successful_count, error_count)
        """
        start_time = time.time()
        
        # Test API connection first
        self.logger.info("Testing Mistral API connection...")
        mistral_available = self.mistral_client.test_connection()
        
        if not mistral_available:
            self.logger.warning("Mistral API connection failed. Checking fallback OCR...")
            if self.fallback_client.test_connection():
                self.logger.info("Fallback OCR (Tesseract) available - continuing with local processing")
            else:
                self.logger.error("Both Mistral API and fallback OCR are unavailable. Cannot proceed.")
                return 0, 0
        
        # Find all supported files
        supported_files = self._find_supported_files()
        
        if not supported_files:
            self.logger.warning("No supported files found to process")
            return 0, 0
        
        # Log file type breakdown
        file_counts = {}
        for file_path in supported_files:
            ext = file_path.suffix.lower()
            file_counts[ext] = file_counts.get(ext, 0) + 1
        
        self.logger.info(f"Found {len(supported_files)} supported files to process:")
        for ext, count in sorted(file_counts.items()):
            self.logger.info(f"  {ext}: {count} files")
        
        # Get already processed files if in resume mode
        processed_files = set()
        if self.config.resume_mode:
            processed_files = self.config.get_processed_files()
            self.logger.info(f"Resume mode: Found {len(processed_files)} already processed files")
        
        # Process each file
        for i, file_path in enumerate(supported_files, 1):
            self.logger.info(f"Processing file {i}/{len(supported_files)}: {file_path.name}")
            
            try:
                # Check file size
                if not self._validate_file_size(file_path):
                    self.error_count += 1
                    self.error_files.append(file_path.name)
                    continue
                
                # Generate sanitized filename
                sanitized_name = self.sanitizer.sanitize_filename(file_path)
                
                # Check if already processed
                if self.config.resume_mode and sanitized_name in processed_files:
                    self.logger.info(f"Skipping already processed file: {file_path.name}")
                    self.skipped_count += 1
                    continue
                
                # Check if output file already exists
                if self.output_manager.file_exists(sanitized_name):
                    if self.config.resume_mode:
                        self.logger.info(f"Output file exists, skipping: {file_path.name}")
                        self.skipped_count += 1
                        continue
                    else:
                        self.logger.warning(f"Output file exists, will overwrite: {sanitized_name}.md")
                
                # Process the file (PDF or image)
                success = self._process_single_file(file_path, sanitized_name)
                
                if success:
                    self.successful_count += 1
                else:
                    self.error_count += 1
                    self.error_files.append(file_path.name)
                
                # Brief pause between requests to be respectful to the API
                time.sleep(0.5)
                
            except Exception as e:
                self.logger.error(f"Unexpected error processing {file_path.name}: {e}")
                self.error_count += 1
                self.error_files.append(file_path.name)
        
        # Calculate processing time
        processing_time = time.time() - start_time
        
        # Save filename mappings
        self.sanitizer.save_mapping_file(self.config.output_dir)
        
        # Create processing summary
        self.output_manager.create_processing_summary(
            total_files=len(supported_files),
            successful=self.successful_count,
            failed=self.error_count,
            skipped=self.skipped_count,
            processing_time=processing_time,
            error_files=self.error_files
        )
        
        # Log final statistics
        self._log_final_stats(len(supported_files), processing_time)
        
        return self.successful_count, self.error_count
    
    def _find_supported_files(self) -> List[Path]:
        """Find all supported files in the input directory."""
        supported_files = []
        
        try:
            # Search recursively for all supported file types
            for ext in self.config.supported_extensions:
                # Add case variations
                patterns = [f"*{ext}", f"*{ext.upper()}"]
                for pattern in patterns:
                    for file_path in self.config.input_dir.rglob(pattern):
                        if file_path.is_file():
                            supported_files.append(file_path)
            
            # Remove duplicates (in case of case-insensitive filesystems)
            supported_files = list(set(supported_files))
            
            # Sort files for consistent processing order
            supported_files.sort(key=lambda x: x.name.lower())
            
        except Exception as e:
            self.logger.error(f"Error finding supported files: {e}")
        
        return supported_files
    
    def _validate_file_size(self, file_path: Path) -> bool:
        """Validate that the file size is reasonable."""
        try:
            file_size_mb = file_path.stat().st_size / (1024 * 1024)
            
            if file_size_mb > self.config.max_file_size_mb:
                self.logger.warning(
                    f"File too large ({file_size_mb:.1f} MB): {file_path.name} "
                    f"(max: {self.config.max_file_size_mb} MB)"
                )
                return False
            
            if file_size_mb < 0.001:  # Less than 1KB
                self.logger.warning(f"File too small: {file_path.name}")
                return False
            
            return True
            
        except Exception as e:
            self.logger.error(f"Error validating file size for {file_path.name}: {e}")
            return False
    
    def _process_single_file(self, file_path: Path, sanitized_name: str) -> bool:
        """
        Process a single file (PDF, image, or document) with fallback support.
        
        Args:
            file_path: Path to the file
            sanitized_name: Sanitized filename for output
            
        Returns:
            True if successful, False otherwise
        """
        response = None
        content = None
        processor_used = "Unknown"
        
        try:
            # First try Mistral OCR API
            self.logger.debug(f"Attempting Mistral OCR for {file_path.name}")
            response = self.mistral_client.process_file(
                file_path=file_path,
                include_images=self.config.include_images,
                max_retries=self.config.max_retries,
                retry_delay=self.config.retry_delay,
                timeout=self.config.request_timeout
            )
            
            if response:
                content = self.mistral_client.extract_text_from_response(response)
                processor_used = "Mistral OCR"
            
        except Exception as e:
            self.logger.warning(f"Mistral OCR failed for {file_path.name}: {e}")
        
        # If Mistral failed, try fallback OCR
        if not content:
            self.logger.info(f"Trying fallback OCR for {file_path.name}")
            try:
                response = self.fallback_client.process_file(
                    file_path=file_path,
                    include_images=self.config.include_images,
                    max_retries=1,
                    retry_delay=0,
                    timeout=self.config.request_timeout
                )
                
                if response:
                    content = self.fallback_client.extract_text_from_response(response)
                    processor_used = "Tesseract OCR (Fallback)"
                    
            except Exception as e:
                self.logger.error(f"Fallback OCR also failed for {file_path.name}: {e}")
        
        # Check if we got any content
        if not content:
            self.logger.error(f"Both Mistral and fallback OCR failed for {file_path.name}")
            return False
        
        # Save as markdown
        try:
            success = self.output_manager.save_markdown(
                content=content,
                sanitized_filename=sanitized_name,
                original_filename=file_path.name,
                metadata={
                    'file_type': file_path.suffix.lower(),
                    'file_size_mb': round(file_path.stat().st_size / (1024 * 1024), 2),
                    'include_images': self.config.include_images,
                    'processor_used': processor_used
                }
            )
            
            if success:
                self.logger.info(f"Successfully processed: {file_path.name} -> {sanitized_name}.md (using {processor_used})")
                return True
            else:
                self.logger.error(f"Failed to save output for {file_path.name}")
                return False
                
        except Exception as e:
            self.logger.error(f"Error saving output for {file_path.name}: {e}")
            return False
    
    def _log_final_stats(self, total_files: int, processing_time: float) -> None:
        """Log final processing statistics."""
        self.logger.info("=" * 50)
        self.logger.info("PROCESSING COMPLETE")
        self.logger.info("=" * 50)
        self.logger.info(f"Total files found: {total_files}")
        self.logger.info(f"Successfully processed: {self.successful_count}")
        self.logger.info(f"Failed: {self.error_count}")
        self.logger.info(f"Skipped: {self.skipped_count}")
        self.logger.info(f"Processing time: {processing_time:.1f} seconds")
        
        if total_files > 0:
            success_rate = (self.successful_count / total_files) * 100
            self.logger.info(f"Success rate: {success_rate:.1f}%")
        
        # Output directory stats
        stats = self.output_manager.get_output_stats()
        self.logger.info(f"Output files: {stats['total_files']}")
        self.logger.info(f"Output size: {stats['total_size_mb']:.1f} MB")
        
        self.logger.info("=" * 50)
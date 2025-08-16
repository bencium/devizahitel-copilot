"""
Output manager for saving OCR results as markdown files.
"""

import logging
from datetime import datetime
from pathlib import Path
from typing import Optional, Dict, Any


class OutputManager:
    """Manages saving OCR results to markdown files."""
    
    def __init__(self, output_dir: Path):
        self.output_dir = Path(output_dir)
        self.output_dir.mkdir(parents=True, exist_ok=True)
        self.logger = logging.getLogger(__name__)
    
    def save_markdown(
        self, 
        content: str, 
        sanitized_filename: str,
        original_filename: str,
        metadata: Optional[Dict[str, Any]] = None
    ) -> bool:
        """
        Save OCR content as a markdown file.
        
        Args:
            content: Extracted text content
            sanitized_filename: Clean filename without extension
            original_filename: Original PDF filename
            metadata: Optional metadata to include in header
            
        Returns:
            True if saved successfully, False otherwise
        """
        try:
            output_path = self.output_dir / f"{sanitized_filename}.md"
            
            # Prepare markdown content with metadata header
            markdown_content = self._create_markdown_content(
                content, original_filename, metadata
            )
            
            # Write to file
            with open(output_path, 'w', encoding='utf-8') as f:
                f.write(markdown_content)
            
            self.logger.info(f"Saved: {output_path.name}")
            return True
            
        except Exception as e:
            self.logger.error(f"Failed to save {sanitized_filename}.md: {e}")
            return False
    
    def _create_markdown_content(
        self, 
        content: str, 
        original_filename: str,
        metadata: Optional[Dict[str, Any]] = None
    ) -> str:
        """
        Create markdown content with metadata header.
        
        Args:
            content: Extracted text content
            original_filename: Original PDF filename
            metadata: Optional metadata dictionary
            
        Returns:
            Complete markdown content with header
        """
        # Create metadata header
        header = "---\n"
        header += f"title: \"{original_filename}\"\n"
        header += f"source_file: \"{original_filename}\"\n"
        header += f"processed_date: \"{datetime.now().isoformat()}\"\n"
        header += f"processor: \"Mistral OCR\"\n"
        
        # Add additional metadata if provided
        if metadata:
            for key, value in metadata.items():
                if key not in ['title', 'source_file', 'processed_date', 'processor']:
                    header += f"{key}: \"{value}\"\n"
        
        header += "---\n\n"
        
        # Add original filename as main heading
        header += f"# {original_filename}\n\n"
        
        # Add processing note
        header += f"*Document processed with Mistral OCR on {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}*\n\n"
        header += "---\n\n"
        
        # Combine header with content
        return header + content
    
    def file_exists(self, sanitized_filename: str) -> bool:
        """
        Check if output file already exists.
        
        Args:
            sanitized_filename: Clean filename without extension
            
        Returns:
            True if file exists, False otherwise
        """
        output_path = self.output_dir / f"{sanitized_filename}.md"
        return output_path.exists()
    
    def create_processing_summary(
        self, 
        total_files: int,
        successful: int,
        failed: int,
        skipped: int,
        processing_time: float,
        error_files: list = None
    ) -> None:
        """
        Create a summary report of the processing session.
        
        Args:
            total_files: Total number of files found
            successful: Number of successfully processed files
            failed: Number of failed files
            skipped: Number of skipped files
            processing_time: Total processing time in seconds
            error_files: List of files that failed processing
        """
        try:
            summary_path = self.output_dir / "processing_summary.md"
            
            content = f"""# OCR Processing Summary

**Date:** {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}

## Results
- **Total files found:** {total_files}
- **Successfully processed:** {successful}
- **Failed:** {failed}
- **Skipped (already exists):** {skipped}
- **Processing time:** {processing_time:.1f} seconds

## Success Rate
{(successful / max(total_files, 1)) * 100:.1f}% of files processed successfully

"""
            
            if error_files:
                content += "## Failed Files\n"
                for error_file in error_files:
                    content += f"- {error_file}\n"
                content += "\n"
            
            content += """## Notes
- Check `ocr_processing.log` for detailed processing information
- Check `filename_mappings.txt` for original to sanitized filename mappings
- Files are processed using Mistral OCR API

---
*Generated by Mistral OCR Processor*
"""
            
            with open(summary_path, 'w', encoding='utf-8') as f:
                f.write(content)
            
            self.logger.info(f"Processing summary saved to {summary_path}")
            
        except Exception as e:
            self.logger.error(f"Failed to create processing summary: {e}")
    
    def get_output_stats(self) -> Dict[str, int]:
        """
        Get statistics about output files.
        
        Returns:
            Dictionary with file counts and sizes
        """
        try:
            md_files = list(self.output_dir.glob("*.md"))
            
            total_files = len(md_files)
            total_size = sum(f.stat().st_size for f in md_files if f.is_file())
            
            return {
                'total_files': total_files,
                'total_size_bytes': total_size,
                'total_size_mb': total_size / (1024 * 1024)
            }
            
        except Exception as e:
            self.logger.error(f"Failed to get output stats: {e}")
            return {'total_files': 0, 'total_size_bytes': 0, 'total_size_mb': 0}
    
    def cleanup_empty_files(self) -> int:
        """
        Remove empty or very small markdown files that might indicate processing errors.
        
        Returns:
            Number of files removed
        """
        removed_count = 0
        min_size_bytes = 100  # Minimum file size to keep
        
        try:
            for md_file in self.output_dir.glob("*.md"):
                if md_file.stat().st_size < min_size_bytes:
                    md_file.unlink()
                    removed_count += 1
                    self.logger.warning(f"Removed empty file: {md_file.name}")
            
            if removed_count > 0:
                self.logger.info(f"Cleaned up {removed_count} empty files")
            
        except Exception as e:
            self.logger.error(f"Error during cleanup: {e}")
        
        return removed_count
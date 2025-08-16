#!/usr/bin/env python3
"""
Mistral OCR PDF Processor
Processes all PDFs in a directory using Mistral's OCR API and saves as markdown files.
"""

import argparse
import logging
import sys
from pathlib import Path
from typing import Optional

from config import Config
from pdf_processor import PDFProcessor


def setup_logging(verbose: bool = False) -> None:
    """Set up logging configuration."""
    level = logging.DEBUG if verbose else logging.INFO
    logging.basicConfig(
        level=level,
        format='%(asctime)s - %(levelname)s - %(message)s',
        handlers=[
            logging.StreamHandler(sys.stdout),
            logging.FileHandler('ocr_processing.log')
        ]
    )


def main():
    """Main entry point for the OCR processor."""
    parser = argparse.ArgumentParser(
        description='Process PDFs in a directory using Mistral OCR API'
    )
    parser.add_argument(
        '--input-dir', 
        type=str, 
        default='.',
        help='Input directory containing PDFs (default: current directory)'
    )
    parser.add_argument(
        '--output-dir', 
        type=str, 
        default='./ocr_output',
        help='Output directory for markdown files (default: ./ocr_output)'
    )
    parser.add_argument(
        '--verbose', 
        action='store_true',
        help='Enable verbose logging'
    )
    parser.add_argument(
        '--include-images',
        action='store_true',
        help='Include base64 image data in API requests'
    )
    parser.add_argument(
        '--resume',
        action='store_true',
        help='Resume processing, skip already processed files'
    )
    parser.add_argument(
        '--file-types',
        type=str,
        default='all',
        help='File types to process: all, pdf, images, documents (default: all)'
    )
    parser.add_argument(
        '--images-only',
        action='store_true',
        help='Process only image files (jpg, jpeg, png, avif)'
    )
    parser.add_argument(
        '--pdf-only',
        action='store_true',
        help='Process only PDF files'
    )

    args = parser.parse_args()
    
    setup_logging(args.verbose)
    logger = logging.getLogger(__name__)
    
    try:
        # Determine file type filter
        file_type_filter = None
        if args.images_only:
            file_type_filter = 'images'
        elif args.pdf_only:
            file_type_filter = 'pdf'
        elif args.file_types != 'all':
            file_type_filter = args.file_types
        
        # Initialize configuration
        config = Config(
            input_dir=Path(args.input_dir),
            output_dir=Path(args.output_dir),
            include_images=args.include_images,
            resume_mode=args.resume,
            file_type_filter=file_type_filter
        )
        
        logger.info(f"Starting OCR processing")
        logger.info(f"Input directory: {config.input_dir}")
        logger.info(f"Output directory: {config.output_dir}")
        
        # Initialize processor
        processor = PDFProcessor(config)
        
        # Process all PDFs
        success_count, error_count = processor.process_directory()
        
        logger.info(f"Processing complete!")
        logger.info(f"Successfully processed: {success_count} files")
        logger.info(f"Errors encountered: {error_count} files")
        
        if error_count > 0:
            logger.warning("Check ocr_processing.log for detailed error information")
            
    except KeyboardInterrupt:
        logger.info("Processing interrupted by user")
        sys.exit(1)
    except Exception as e:
        logger.error(f"Fatal error: {e}")
        sys.exit(1)


if __name__ == "__main__":
    main()
"""
Fallback OCR implementation using Tesseract for when Mistral API fails.
"""

import logging
import base64
from pathlib import Path
from typing import Optional, Dict, Any
from PIL import Image
import pytesseract
import PyPDF2
import io


class FallbackOCRClient:
    """Fallback OCR client using Tesseract OCR."""
    
    def __init__(self):
        self.logger = logging.getLogger(__name__)
        self.logger.info("Initializing fallback OCR client (Tesseract)")
        
        # Configure Tesseract for Hungarian language support
        self.tesseract_config = r'--oem 3 --psm 6 -l hun+eng'
    
    def process_file(
        self, 
        file_path: Path, 
        include_images: bool = False,
        max_retries: int = 1,
        retry_delay: int = 0,
        timeout: int = 60
    ) -> Optional[Dict[str, Any]]:
        """
        Process a file using Tesseract OCR as fallback.
        
        Args:
            file_path: Path to file
            include_images: Not used in fallback (for API compatibility)
            max_retries: Number of retry attempts
            retry_delay: Delay between retries
            timeout: Not used in fallback
            
        Returns:
            Mock API response dict or None if failed
        """
        try:
            self.logger.info(f"Processing {file_path.name} with fallback OCR (Tesseract)")
            
            ext = file_path.suffix.lower()
            if ext in ['.jpg', '.jpeg', '.png', '.avif']:
                text_content = self._process_image(file_path)
            elif ext == '.pdf':
                text_content = self._process_pdf(file_path)
            else:
                self.logger.warning(f"Unsupported file type for fallback: {ext}")
                return None
            
            if not text_content:
                self.logger.warning(f"No text extracted from {file_path.name}")
                return None
            
            # Create a mock response structure similar to Mistral API
            mock_response = type('MockOCRResponse', (), {
                'pages': [type('MockPage', (), {
                    'markdown': text_content,
                    'text': text_content,
                    'content': text_content
                })()]
            })()
            
            self.logger.info(f"Successfully processed {file_path.name} with fallback OCR")
            return mock_response
            
        except Exception as e:
            self.logger.error(f"Fallback OCR failed for {file_path.name}: {e}")
            return None
    
    def _process_image(self, image_path: Path) -> Optional[str]:
        """
        Process an image file using Tesseract OCR.
        
        Args:
            image_path: Path to image file
            
        Returns:
            Extracted text or None if failed
        """
        try:
            # Open and process image
            with Image.open(image_path) as img:
                # Convert to RGB if necessary
                if img.mode != 'RGB':
                    img = img.convert('RGB')
                
                # Extract text using Tesseract
                text = pytesseract.image_to_string(img, config=self.tesseract_config)
                
                # Clean up the text
                text = text.strip()
                if not text:
                    self.logger.warning(f"No text found in image: {image_path.name}")
                    return None
                
                # Convert to markdown-like format
                lines = text.split('\n')
                markdown_text = '\n'.join(line.strip() for line in lines if line.strip())
                
                return markdown_text
                
        except Exception as e:
            self.logger.error(f"Failed to process image {image_path.name}: {e}")
            return None
    
    def _process_pdf(self, pdf_path: Path) -> Optional[str]:
        """
        Process a PDF file using PyPDF2 for text extraction.
        
        Args:
            pdf_path: Path to PDF file
            
        Returns:
            Extracted text or None if failed
        """
        try:
            text_content = ""
            
            with open(pdf_path, 'rb') as file:
                pdf_reader = PyPDF2.PdfReader(file)
                
                for page_num, page in enumerate(pdf_reader.pages):
                    try:
                        page_text = page.extract_text()
                        if page_text.strip():
                            text_content += f"\n\n## Page {page_num + 1}\n\n{page_text.strip()}"
                    except Exception as e:
                        self.logger.warning(f"Failed to extract text from page {page_num + 1}: {e}")
                        continue
            
            if not text_content.strip():
                self.logger.warning(f"No text extracted from PDF: {pdf_path.name}")
                return None
            
            return text_content.strip()
            
        except Exception as e:
            self.logger.error(f"Failed to process PDF {pdf_path.name}: {e}")
            return None
    
    def extract_text_from_response(self, response: Any) -> Optional[str]:
        """
        Extract text from fallback OCR response.
        
        Args:
            response: Mock OCR response object
            
        Returns:
            Extracted text content or None if failed
        """
        try:
            if hasattr(response, 'pages') and response.pages:
                page = response.pages[0]
                if hasattr(page, 'markdown'):
                    return page.markdown
                elif hasattr(page, 'text'):
                    return page.text
                elif hasattr(page, 'content'):
                    return page.content
            
            return str(response) if response else None
            
        except Exception as e:
            self.logger.error(f"Failed to extract text from fallback response: {e}")
            return None
    
    def test_connection(self) -> bool:
        """
        Test the fallback OCR functionality.
        
        Returns:
            True if Tesseract is available, False otherwise
        """
        try:
            # Test Tesseract installation
            version = pytesseract.get_tesseract_version()
            self.logger.info(f"Tesseract version: {version}")
            
            # Test with a simple image
            test_img = Image.new('RGB', (100, 50), color='white')
            pytesseract.image_to_string(test_img)
            
            self.logger.info("Fallback OCR (Tesseract) connection test successful")
            return True
            
        except Exception as e:
            self.logger.error(f"Fallback OCR test failed: {e}")
            self.logger.error("Make sure Tesseract is installed: https://github.com/tesseract-ocr/tesseract")
            return False
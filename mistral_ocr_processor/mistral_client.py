"""
Mistral OCR API client for processing PDF documents.
"""

import base64
import logging
import time
from pathlib import Path
from typing import Dict, Any, Optional
from mistralai import Mistral


class MistralOCRClient:
    """Client for interacting with Mistral's OCR API."""
    
    def __init__(self, api_key: str, api_url: str, model: str):
        self.api_key = api_key
        self.api_url = api_url
        self.model = model
        self.client = Mistral(api_key=api_key)
        self.logger = logging.getLogger(__name__)
    
    def process_file(
        self, 
        file_path: Path, 
        include_images: bool = False,
        max_retries: int = 3,
        retry_delay: int = 2,
        timeout: int = 120
    ) -> Optional[Dict[str, Any]]:
        """
        Process a file (PDF, image, or document) using Mistral OCR API.
        
        Args:
            file_path: Path to file
            include_images: Whether to include base64 image data
            max_retries: Maximum number of retry attempts
            retry_delay: Delay between retries in seconds
            timeout: Request timeout in seconds
            
        Returns:
            API response dict or None if failed
        """
        try:
            # Encode file to base64
            file_base64 = self._encode_file_to_base64(file_path)
            if not file_base64:
                return None
            
            # Get MIME type for the file
            mime_type = self._get_mime_type(file_path)
            
            # Make API request with retries
            for attempt in range(max_retries):
                try:
                    self.logger.debug(f"Processing {file_path.name} (attempt {attempt + 1}/{max_retries})")
                    
                    # Use the Mistral SDK client with appropriate request structure
                    ext = file_path.suffix.lower()
                    if ext in ['.jpg', '.jpeg', '.png', '.avif']:
                        # Images use image_url structure with proper MIME types
                        actual_mime = self._get_actual_mime_type(file_path)
                        response = self.client.ocr.process(
                            model=self.model,
                            document={
                                "type": "image_url",
                                "image_url": f"data:{actual_mime};base64,{file_base64}"
                            },
                            include_image_base64=include_images
                        )
                    else:
                        # Documents (PDF, DOCX, PPTX) use document_url structure
                        response = self.client.ocr.process(
                            model=self.model,
                            document={
                                "type": "document_url",
                                "document_url": f"data:{mime_type};base64,{file_base64}"
                            },
                            include_image_base64=include_images
                        )
                    
                    self.logger.info(f"Successfully processed {file_path.name}")
                    return response
                    
                except Exception as e:
                    error_str = str(e).lower()
                    
                    if "rate limit" in error_str or "429" in error_str:
                        wait_time = retry_delay * (attempt + 1)
                        self.logger.warning(f"Rate limit hit for {file_path.name}. Waiting {wait_time}s...")
                        time.sleep(wait_time)
                        continue
                    
                    elif "400" in error_str or "bad request" in error_str:
                        self.logger.error(f"Bad request for {file_path.name}: {e}")
                        return None  # Don't retry bad requests
                    
                    else:
                        self.logger.warning(
                            f"API error for {file_path.name} (attempt {attempt + 1}): {e}"
                        )
                
                # Wait before retry (except on last attempt)
                if attempt < max_retries - 1:
                    time.sleep(retry_delay)
            
            self.logger.error(f"Failed to process {file_path.name} after {max_retries} attempts")
            return None
            
        except Exception as e:
            self.logger.error(f"Unexpected error processing {file_path.name}: {e}")
            return None
    
    def _encode_file_to_base64(self, file_path: Path) -> Optional[str]:
        """
        Encode file to base64 string.
        
        Args:
            file_path: Path to file
            
        Returns:
            Base64 encoded string or None if failed
        """
        try:
            with open(file_path, 'rb') as file:
                file_content = file.read()
                
            # Check file size (warn if large)
            file_size_mb = len(file_content) / (1024 * 1024)
            if file_size_mb > 10:
                self.logger.warning(f"Large file: {file_path.name} ({file_size_mb:.1f} MB)")
            
            # Encode to base64
            base64_content = base64.b64encode(file_content).decode('utf-8')
            return base64_content
            
        except Exception as e:
            self.logger.error(f"Failed to encode {file_path.name}: {e}")
            return None
    
    def _get_mime_type(self, file_path: Path) -> str:
        """
        Get MIME type for file based on extension.
        For documents (PDF, DOCX, PPTX) - used with document_url.
        
        Args:
            file_path: Path to file
            
        Returns:
            MIME type string for document_url requests
        """
        ext = file_path.suffix.lower()
        if ext == '.pdf':
            return 'application/pdf'
        elif ext == '.pptx':
            return 'application/vnd.openxmlformats-officedocument.presentationml.presentation'
        elif ext == '.docx':
            return 'application/vnd.openxmlformats-officedocument.wordprocessingml.document'
        else:
            return 'application/octet-stream'
    
    def _get_actual_mime_type(self, file_path: Path) -> str:
        """
        Get actual MIME type for image files.
        For images (JPG, PNG, AVIF) - used with image_url.
        
        Args:
            file_path: Path to file
            
        Returns:
            Actual MIME type string for image_url requests
        """
        ext = file_path.suffix.lower()
        if ext in ['.jpg', '.jpeg']:
            return 'image/jpeg'
        elif ext == '.png':
            return 'image/png'
        elif ext == '.avif':
            return 'image/avif'
        else:
            return 'image/jpeg'  # fallback
    
    def extract_text_from_response(self, response: Any) -> Optional[str]:
        """
        Extract markdown text from Mistral OCR API response.
        
        Args:
            response: API response object from Mistral SDK
            
        Returns:
            Extracted text content or None if failed
        """
        try:
            # The Mistral SDK returns an OCRResponse object with pages
            if hasattr(response, 'pages') and response.pages:
                # Combine all pages' text content
                combined_text = ""
                for page in response.pages:
                    if hasattr(page, 'markdown'):
                        combined_text += page.markdown + "\n\n"
                    elif hasattr(page, 'text'):
                        combined_text += page.text + "\n\n"
                    elif hasattr(page, 'content'):
                        combined_text += page.content + "\n\n"
                return combined_text.strip() if combined_text else None
            
            # Check for document_annotation attribute
            if hasattr(response, 'document_annotation'):
                return response.document_annotation
            
            # Fallback to other common attributes
            if hasattr(response, 'content'):
                return response.content
            
            if hasattr(response, 'text'):
                return response.text
            
            # Try to convert to dict and extract
            if hasattr(response, 'model_dump'):
                try:
                    response_dict = response.model_dump()
                    if 'pages' in response_dict and response_dict['pages']:
                        combined_text = ""
                        for page in response_dict['pages']:
                            if 'markdown' in page:
                                combined_text += page['markdown'] + "\n\n"
                            elif 'text' in page:
                                combined_text += page['text'] + "\n\n"
                            elif 'content' in page:
                                combined_text += page['content'] + "\n\n"
                        return combined_text.strip() if combined_text else None
                    
                    if 'document_annotation' in response_dict:
                        return response_dict['document_annotation']
                except Exception:
                    pass
            
            # Log for debugging but try string conversion
            self.logger.debug(f"OCR Response type: {type(response)} with attributes: {[attr for attr in dir(response) if not attr.startswith('_')]}")
            return str(response) if response else None
            
        except Exception as e:
            self.logger.error(f"Failed to extract text from response: {e}")
            return None
    
    def test_connection(self) -> bool:
        """
        Test the API connection and authentication.
        
        Returns:
            True if connection successful, False otherwise
        """
        try:
            # Test with a minimal valid PDF (empty document)
            minimal_pdf = "JVBERi0xLjQKJcOkw7zDssOMCjEgMCBvYmoKPDwKL1R5cGUgL0NhdGFsb2cKL1BhZ2VzIDIgMCBSCj4+CmVuZG9iago="
            
            response = self.client.ocr.process(
                model=self.model,
                document={
                    "type": "document_url",
                    "document_url": f"data:application/pdf;base64,{minimal_pdf}"
                },
                include_image_base64=False
            )
            
            self.logger.info("API connection test successful")
            return True
                
        except Exception as e:
            error_str = str(e).lower()
            if "401" in error_str or "authentication" in error_str:
                self.logger.error("API authentication failed - check your API key")
            else:
                self.logger.error(f"API connection test failed: {e}")
            return False
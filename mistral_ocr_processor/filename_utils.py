"""
Filename sanitization utilities for handling accented characters and special symbols.
"""

import re
from pathlib import Path
from typing import Dict, Set
import unicodedata


class FilenameSanitizer:
    """Utility class for sanitizing filenames with accented characters."""
    
    # Hungarian character mappings for better readability
    HUNGARIAN_MAPPINGS = {
        'á': 'a', 'à': 'a', 'â': 'a', 'ä': 'a', 'ã': 'a', 'å': 'a',
        'é': 'e', 'è': 'e', 'ê': 'e', 'ë': 'e',
        'í': 'i', 'ì': 'i', 'î': 'i', 'ï': 'i',
        'ó': 'o', 'ò': 'o', 'ô': 'o', 'ö': 'o', 'õ': 'o', 'ő': 'o',
        'ú': 'u', 'ù': 'u', 'û': 'u', 'ü': 'u', 'ű': 'u',
        'ý': 'y', 'ÿ': 'y',
        'ñ': 'n', 'ç': 'c',
        'Á': 'A', 'À': 'A', 'Â': 'A', 'Ä': 'A', 'Ã': 'A', 'Å': 'A',
        'É': 'E', 'È': 'E', 'Ê': 'E', 'Ë': 'E',
        'Í': 'I', 'Ì': 'I', 'Î': 'I', 'Ï': 'I',
        'Ó': 'O', 'Ò': 'O', 'Ô': 'O', 'Ö': 'O', 'Õ': 'O', 'Ő': 'O',
        'Ú': 'U', 'Ù': 'U', 'Û': 'U', 'Ü': 'U', 'Ű': 'U',
        'Ý': 'Y', 'Ÿ': 'Y',
        'Ñ': 'N', 'Ç': 'C'
    }
    
    def __init__(self):
        self.processed_names: Dict[str, str] = {}
        self.used_names: Set[str] = set()
    
    def sanitize_filename(self, original_path: Path) -> str:
        """
        Convert accented filename to safe English-only filename.
        
        Args:
            original_path: Original file path
            
        Returns:
            Sanitized filename without extension
        """
        if str(original_path) in self.processed_names:
            return self.processed_names[str(original_path)]
        
        # Get filename without extension
        filename = original_path.stem
        
        # Apply custom Hungarian mappings first
        sanitized = self._apply_hungarian_mappings(filename)
        
        # Apply unicode normalization as fallback
        sanitized = self._apply_unicode_normalization(sanitized)
        
        # Remove or replace special characters
        sanitized = self._clean_special_characters(sanitized)
        
        # Ensure it's not empty
        if not sanitized:
            sanitized = "document"
        
        # Ensure uniqueness
        sanitized = self._ensure_unique(sanitized)
        
        # Store mapping for future reference
        self.processed_names[str(original_path)] = sanitized
        self.used_names.add(sanitized)
        
        return sanitized
    
    def _apply_hungarian_mappings(self, text: str) -> str:
        """Apply custom Hungarian character mappings."""
        result = text
        for accented, ascii_char in self.HUNGARIAN_MAPPINGS.items():
            result = result.replace(accented, ascii_char)
        return result
    
    def _apply_unicode_normalization(self, text: str) -> str:
        """Apply unicode normalization for remaining accented characters."""
        # Normalize to NFD (decomposed form) and remove diacritics
        normalized = unicodedata.normalize('NFD', text)
        ascii_text = ''.join(
            char for char in normalized 
            if unicodedata.category(char) != 'Mn'  # Remove diacritical marks
        )
        return ascii_text
    
    def _clean_special_characters(self, text: str) -> str:
        """Remove or replace special characters and spaces."""
        # Replace spaces and common separators with underscores
        text = re.sub(r'[\s\-\.]+', '_', text)
        
        # Remove non-alphanumeric characters except underscores
        text = re.sub(r'[^a-zA-Z0-9_]', '', text)
        
        # Remove multiple consecutive underscores
        text = re.sub(r'_+', '_', text)
        
        # Remove leading/trailing underscores
        text = text.strip('_')
        
        return text
    
    def _ensure_unique(self, name: str) -> str:
        """Ensure filename is unique by adding numbers if necessary."""
        if name not in self.used_names:
            return name
        
        counter = 1
        while f"{name}_{counter}" in self.used_names:
            counter += 1
        
        return f"{name}_{counter}"
    
    def get_mapping_report(self) -> str:
        """Generate a report of all filename mappings."""
        if not self.processed_names:
            return "No files processed yet."
        
        report = "Filename Mappings:\n"
        report += "=" * 50 + "\n"
        
        for original, sanitized in self.processed_names.items():
            original_name = Path(original).name
            report += f"{original_name} -> {sanitized}.md\n"
        
        return report
    
    def save_mapping_file(self, output_dir: Path) -> None:
        """Save filename mappings to a reference file."""
        mapping_file = output_dir / "filename_mappings.txt"
        
        with open(mapping_file, 'w', encoding='utf-8') as f:
            f.write(self.get_mapping_report())


def test_sanitizer():
    """Test function for the filename sanitizer."""
    sanitizer = FilenameSanitizer()
    
    test_files = [
        "aegon-ászf.pdf",
        "erste-elidegen-törl2010.pdf", 
        "csernak_bence-hasznalati_megall-.pdf",
        "etvfogyasztóváltás_dokumentumlista_magánszemély_20140620.pdf",
        "Törökbálint vevők által aláírt előszerződés16.10.18..pdf"
    ]
    
    print("Testing filename sanitization:")
    print("-" * 50)
    
    for filename in test_files:
        path = Path(filename)
        sanitized = sanitizer.sanitize_filename(path)
        print(f"{filename} -> {sanitized}.md")
    
    print("\nMapping report:")
    print(sanitizer.get_mapping_report())


if __name__ == "__main__":
    test_sanitizer()
#!/usr/bin/env python3
"""
Script to process the 5 specific failed files.
"""

import os
import shutil
import tempfile
from pathlib import Path
import subprocess
import sys

# The 5 failed files to find and process
FAILED_FILES = [
    "aegon-közjegyz.pdf",
    "aegon-vegtorl-scan.pdf", 
    "Csernák Bence_Dudás Petra Zsófia_ADÁSVÉTELI SZERZŐDÉS_tervezet_161025.docx",
    "erste-közj.pdf",
    "megvett-ingatlan.pdf"
]

def find_files(base_dir, filenames):
    """Find the exact paths for the failed files."""
    found_files = {}
    
    print(f"Searching for failed files in {base_dir}...")
    
    for filename in filenames:
        print(f"Looking for: {filename}")
        # Use find command to search for files
        result = subprocess.run(
            ["find", base_dir, "-name", filename, "-type", "f"],
            capture_output=True,
            text=True
        )
        
        if result.returncode == 0 and result.stdout.strip():
            paths = result.stdout.strip().split('\n')
            for path in paths:
                if path and os.path.exists(path):
                    found_files[filename] = path
                    print(f"  ✓ Found: {path}")
                    break
        else:
            print(f"  ✗ Not found: {filename}")
    
    return found_files

def main():
    base_search_dir = "/Users/bencium/_devizahitel"
    
    # Find the files
    found_files = find_files(base_search_dir, FAILED_FILES)
    
    if not found_files:
        print("No failed files found to process.")
        return
    
    print(f"\nFound {len(found_files)} out of {len(FAILED_FILES)} failed files:")
    for name, path in found_files.items():
        print(f"  {name} -> {path}")
    
    # Create temporary directory with just these files
    with tempfile.TemporaryDirectory() as temp_dir:
        temp_path = Path(temp_dir)
        print(f"\nCopying files to temporary directory: {temp_path}")
        
        for name, source_path in found_files.items():
            dest_path = temp_path / name
            shutil.copy2(source_path, dest_path)
            print(f"  Copied: {name}")
        
        # Run the main processor on just these files
        print(f"\nProcessing {len(found_files)} files...")
        
        cmd = [
            sys.executable, "main.py",
            "--input-dir", str(temp_path),
            "--output-dir", "./ocr_output_retry",
            "--verbose"
        ]
        
        print(f"Running: {' '.join(cmd)}")
        
        # Run in the current directory where main.py is located
        result = subprocess.run(cmd, cwd=os.getcwd())
        
        if result.returncode == 0:
            print("\n✅ Processing completed successfully!")
        else:
            print(f"\n❌ Processing failed with return code: {result.returncode}")

if __name__ == "__main__":
    main()
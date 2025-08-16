use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::collections::HashMap;
use tokio::time::interval;
use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileChangeEvent {
    pub file_path: String,
    pub change_type: FileChangeType,
    pub timestamp: u64,
    pub file_size: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FileChangeType {
    Added,
    Modified,
    Deleted,
}

#[derive(Debug)]
pub struct FileWatcher {
    watch_directory: PathBuf,
    file_states: HashMap<PathBuf, FileState>,
    check_interval: Duration,
    enabled: bool,
}

#[derive(Debug, Clone)]
struct FileState {
    modified_time: SystemTime,
    size: u64,
}

impl FileWatcher {
    pub fn new(watch_directory: impl AsRef<Path>) -> Result<Self> {
        let watch_directory = watch_directory.as_ref().to_path_buf();
        
        if !watch_directory.exists() {
            return Err(anyhow!("Watch directory does not exist: {:?}", watch_directory));
        }

        let check_interval_seconds = std::env::var("FILE_WATCH_INTERVAL_SECONDS")
            .unwrap_or_else(|_| "5".to_string())
            .parse::<u64>()
            .unwrap_or(5);

        let enabled = std::env::var("ENABLE_FILE_WATCHER")
            .unwrap_or_else(|_| "true".to_string())
            .parse::<bool>()
            .unwrap_or(true);

        let mut watcher = Self {
            watch_directory,
            file_states: HashMap::new(),
            check_interval: Duration::from_secs(check_interval_seconds),
            enabled,
        };

        // Initialize with current state
        watcher.scan_initial_state()?;

        Ok(watcher)
    }

    fn scan_initial_state(&mut self) -> Result<()> {
        println!("📁 Scanning initial file state in {:?}", self.watch_directory);
        
        self.scan_directory(&self.watch_directory.clone())?;
        
        println!("📊 Found {} files to monitor", self.file_states.len());
        Ok(())
    }

    fn scan_directory(&mut self, directory: &Path) -> Result<()> {
        if !directory.is_dir() {
            return Ok(());
        }

        for entry in std::fs::read_dir(directory)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                // Only monitor text-based files that might contain legal documents
                if let Some(extension) = path.extension() {
                    if matches!(extension.to_str(), Some("md") | Some("txt") | Some("rtf") | Some("pdf") | Some("docx")) {
                        if let Ok(metadata) = entry.metadata() {
                            let file_state = FileState {
                                modified_time: metadata.modified().unwrap_or(UNIX_EPOCH),
                                size: metadata.len(),
                            };
                            self.file_states.insert(path, file_state);
                        }
                    }
                }
            } else if path.is_dir() {
                // Recursively scan subdirectories (but avoid hidden directories)
                if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
                    if !dir_name.starts_with('.') {
                        self.scan_directory(&path)?;
                    }
                }
            }
        }

        Ok(())
    }

    pub async fn start_watching<F>(&mut self, mut callback: F) -> Result<()>
    where
        F: FnMut(Vec<FileChangeEvent>) + Send + 'static,
    {
        if !self.enabled {
            println!("📁 File watcher is disabled");
            return Ok(());
        }

        println!("👀 Starting file watcher for {:?}", self.watch_directory);
        println!("🔄 Checking for changes every {:?}", self.check_interval);

        let mut interval_timer = interval(self.check_interval);

        loop {
            interval_timer.tick().await;
            
            match self.check_for_changes() {
                Ok(changes) => {
                    if !changes.is_empty() {
                        println!("📝 Detected {} file changes", changes.len());
                        for change in &changes {
                            println!("   {:?}: {}", change.change_type, change.file_path);
                        }
                        callback(changes);
                    }
                },
                Err(e) => {
                    eprintln!("❌ Error checking for file changes: {}", e);
                }
            }
        }
    }

    fn check_for_changes(&mut self) -> Result<Vec<FileChangeEvent>> {
        let mut changes = Vec::new();
        let mut current_files = HashMap::new();

        // Scan current state
        self.scan_directory_for_changes(&self.watch_directory.clone(), &mut current_files)?;

        // Check for added or modified files
        for (path, current_state) in &current_files {
            if let Some(previous_state) = self.file_states.get(path) {
                // File existed before - check if modified
                if current_state.modified_time > previous_state.modified_time ||
                   current_state.size != previous_state.size {
                    changes.push(FileChangeEvent {
                        file_path: path.to_string_lossy().to_string(),
                        change_type: FileChangeType::Modified,
                        timestamp: current_state.modified_time
                            .duration_since(UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_secs(),
                        file_size: current_state.size,
                    });
                }
            } else {
                // New file
                changes.push(FileChangeEvent {
                    file_path: path.to_string_lossy().to_string(),
                    change_type: FileChangeType::Added,
                    timestamp: current_state.modified_time
                        .duration_since(UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs(),
                    file_size: current_state.size,
                });
            }
        }

        // Check for deleted files
        for path in self.file_states.keys() {
            if !current_files.contains_key(path) {
                changes.push(FileChangeEvent {
                    file_path: path.to_string_lossy().to_string(),
                    change_type: FileChangeType::Deleted,
                    timestamp: SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs(),
                    file_size: 0,
                });
            }
        }

        // Update our state
        self.file_states = current_files;

        Ok(changes)
    }

    fn scan_directory_for_changes(&self, directory: &Path, file_states: &mut HashMap<PathBuf, FileState>) -> Result<()> {
        if !directory.is_dir() {
            return Ok(());
        }

        for entry in std::fs::read_dir(directory)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if matches!(extension.to_str(), Some("md") | Some("txt") | Some("rtf") | Some("pdf") | Some("docx")) {
                        if let Ok(metadata) = entry.metadata() {
                            let file_state = FileState {
                                modified_time: metadata.modified().unwrap_or(UNIX_EPOCH),
                                size: metadata.len(),
                            };
                            file_states.insert(path, file_state);
                        }
                    }
                }
            } else if path.is_dir() {
                if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
                    if !dir_name.starts_with('.') {
                        self.scan_directory_for_changes(&path, file_states)?;
                    }
                }
            }
        }

        Ok(())
    }

    pub fn get_monitored_files(&self) -> Vec<String> {
        self.file_states.keys()
            .map(|path| path.to_string_lossy().to_string())
            .collect()
    }

    pub fn get_file_count(&self) -> usize {
        self.file_states.len()
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

// Utility function to create file watcher with default OCR directory
pub fn create_ocr_file_watcher() -> Result<FileWatcher> {
    let ocr_dir = std::env::var("OCR_OUTPUT_DIR")
        .unwrap_or_else(|_| "./ocr_output".to_string());
    
    FileWatcher::new(ocr_dir)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs::write;

    #[tokio::test]
    async fn test_file_watcher_detects_new_file() {
        let temp_dir = TempDir::new().unwrap();
        let mut watcher = FileWatcher::new(temp_dir.path()).unwrap();

        // Create a new file
        let test_file = temp_dir.path().join("test.md");
        write(&test_file, "test content").unwrap();

        // Check for changes
        let changes = watcher.check_for_changes().unwrap();
        assert_eq!(changes.len(), 1);
        assert!(matches!(changes[0].change_type, FileChangeType::Added));
    }

    #[tokio::test]
    async fn test_file_watcher_detects_modification() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.md");
        write(&test_file, "initial content").unwrap();

        let mut watcher = FileWatcher::new(temp_dir.path()).unwrap();

        // Give some time for different modification timestamp
        tokio::time::sleep(Duration::from_millis(10)).await;

        // Modify the file
        write(&test_file, "modified content").unwrap();

        let changes = watcher.check_for_changes().unwrap();
        assert_eq!(changes.len(), 1);
        assert!(matches!(changes[0].change_type, FileChangeType::Modified));
    }
}
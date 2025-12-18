use clap::Parser;
use colored::Colorize;
use futures::future::BoxFuture;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::{Path, PathBuf};
use tokio::fs;

fn format_size_with_color(size: u64) -> colored::ColoredString {
    let units = ["B", "KB", "MB", "GB", "TB"];
    let mut size_value = size as f64;
    let mut unit_index = 0;
    
    while size_value >= 1024.0 && unit_index < units.len() - 1 {
        size_value /= 1024.0;
        unit_index += 1;
    }
    
    let formatted = format!("{:.1} {}", size_value, units[unit_index]);
    
    match units[unit_index] {
        "TB" => formatted.red(),
        "GB" => formatted.yellow(), // Use yellow instead of orange since orange() doesn't exist
        "MB" => formatted.green(),
        "KB" => formatted.blue(),
        "B" => formatted.purple(),
        _ => formatted.cyan(), // default color as fallback
    }
}

fn create_progress_bar() -> ProgressBar {
    let pb = ProgressBar::new(0);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} files scanned")
        .unwrap()
        .progress_chars("=> "));
    pb
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser)]
enum Commands {
    /// Detect cache files
    Detect {
        /// Path to scan (default: current directory)
        path: Option<PathBuf>,
    },
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
enum CacheCategory {
    Browser,
    System,
    Application,
    Log,
    Temporary,
    Backup,
    Other,
}

impl CacheCategory {
    fn as_str(&self) -> &'static str {
        match self {
            CacheCategory::Browser => "Browser",
            CacheCategory::System => "System",
            CacheCategory::Application => "Application",
            CacheCategory::Log => "Log",
            CacheCategory::Temporary => "Temporary",
            CacheCategory::Backup => "Backup",
            CacheCategory::Other => "Other",
        }
    }
}

#[derive(Debug)]
struct CacheFile {
    path: PathBuf,
    size: u64,
    category: CacheCategory,
}

impl CacheFile {
    async fn new(path: PathBuf) -> Option<Self> {
        match fs::metadata(&path).await {
            Ok(metadata) if metadata.is_file() => {
                // Classify the cache file
                let category = classify_cache_file(&path).unwrap_or(CacheCategory::Other);
                
                Some(CacheFile {
                    path,
                    size: metadata.len(),
                    category,
                })
            }
            _ => None,
        }
    }
}

fn classify_cache_file(path: &Path) -> Option<CacheCategory> {
    let file_name = match path.file_name() {
        Some(os_str) => match os_str.to_str() {
            Some(name) => name.to_lowercase(),
            None => return None,
        },
        None => return None,
    };
    
    let path_str = path.to_string_lossy().to_lowercase();
    
    // Check for browser cache patterns
    let browser_patterns = ["chrome", "firefox", "edge", "safari", "browser", "mozilla"];
    if browser_patterns.iter().any(|&pattern| path_str.contains(pattern)) {
        return Some(CacheCategory::Browser);
    }
    
    // Check for log files
    if file_name.ends_with(".log") || path_str.contains("log") {
        return Some(CacheCategory::Log);
    }
    
    // Check for temporary files
    let temp_patterns = [".tmp", ".temp", ".swp", ".swo", ".crdownload", ".part", "tmp", "temp"];
    if temp_patterns.iter().any(|&pattern| file_name.contains(pattern) || path_str.contains(pattern)) {
        return Some(CacheCategory::Temporary);
    }
    
    // Check for backup files
    let backup_patterns = [".bak", ".backup", ".old", "backup"];
    if backup_patterns.iter().any(|&pattern| file_name.contains(pattern) || path_str.contains(pattern)) {
        return Some(CacheCategory::Backup);
    }
    
    // Check for system cache patterns
    let system_patterns = ["system", ".cache", "cache"];
    if system_patterns.iter().any(|&pattern| path_str.contains(pattern)) {
        return Some(CacheCategory::System);
    }
    
    // Check for application cache patterns
    let app_patterns = ["app", "application", ".app"];
    if app_patterns.iter().any(|&pattern| path_str.contains(pattern)) {
        return Some(CacheCategory::Application);
    }
    
    // Default to Other
    Some(CacheCategory::Other)
}

fn is_cache_file(path: &Path) -> bool {
    let cache_extensions = [
        ".cache", ".tmp", ".temp", ".swp", ".swo", ".bak", 
        ".log", ".old", ".backup", ".crdownload", ".part",
    ];
    
    let cache_directories = [
        "cache", "caches", ".cache", "temp", ".temp", "tmp", ".tmp",
        "logs", ".logs", "backup", ".backup", "old", ".old",
    ];
    
    let file_name = match path.file_name() {
        Some(os_str) => match os_str.to_str() {
            Some(name) => name,
            None => return false,
        },
        None => return false,
    };
    
    let parent_name = match path.parent() {
        Some(parent) => match parent.file_name() {
            Some(os_str) => match os_str.to_str() {
                Some(name) => name,
                None => return false,
            },
            None => return false,
        },
        None => return false,
    };
    
    // Check by extension
    for ext in cache_extensions.iter() {
        if file_name.to_lowercase().ends_with(ext) {
            return true;
        }
    }
    
    // Check by directory name
    for dir in cache_directories.iter() {
        if parent_name.to_lowercase() == *dir {
            return true;
        }
    }
    
    // Check for common cache file patterns
    let cache_patterns = [
        "cache", "temp", "tmp", "log", "backup", "old", ".swp", ".swo",
        "crdownload", "part", "~$", ".ds_store",
    ];
    
    for pattern in cache_patterns.iter() {
        if file_name.to_lowercase().contains(pattern) {
            return true;
        }
    }
    
    false
}

// Define a boxed future type for recursive async function
type WalkDirFuture<'a> = BoxFuture<'a, Vec<PathBuf>>;

async fn async_walk_dir(path: &Path) -> Vec<PathBuf> {
    async_walk_dir_inner(path).await
}

// Helper function with boxed future to handle recursion
fn async_walk_dir_inner(path: &Path) -> WalkDirFuture<'_> {
    Box::pin(async move {
        let mut files = Vec::new();
        
        if let Ok(mut dir_entries) = fs::read_dir(path).await {
            // Use async iteration with proper Result<Option<DirEntry>> handling
            while let Ok(Some(entry)) = dir_entries.next_entry().await {
                let entry_path = entry.path();
                
                if let Ok(metadata) = fs::metadata(&entry_path).await {
                    if metadata.is_dir() {
                        // Recursively walk subdirectories with boxed future
                        let mut sub_files = async_walk_dir_inner(&entry_path).await;
                        files.append(&mut sub_files);
                    } else if metadata.is_file() {
                        files.push(entry_path);
                    }
                }
            }
        }
        
        files
    })
}

async fn scan_cache_files(path: &Path) -> Vec<CacheFile> {
    let mut cache_files = Vec::new();
    
    println!("{} Traversing directory structure...", "[Running!]".yellow());
    
    // Asynchronously get all files
    let all_files = async_walk_dir(path).await;
    let total_files = all_files.len() as u64;
    
    // Create progress bar
    let pb = create_progress_bar();
    pb.set_length(total_files);
    
    // Process files asynchronously with progress updates
    for (i, file_path) in all_files.into_iter().enumerate() {
        pb.set_position((i + 1) as u64);
        
        if is_cache_file(&file_path) {
            if let Some(cache_file) = CacheFile::new(file_path).await {
                cache_files.push(cache_file);
            }
        }
    }
    
    pb.finish_with_message("Scan completed");
    
    cache_files
}

async fn detect_cache_files(path: &Path) {
    println!("{} Scanning for cache files in {}", "[Scan:]".yellow(), path.display());
    
    let cache_files = scan_cache_files(path).await;
    let total_size: u64 = cache_files.iter().map(|f| f.size).sum();
    
    println!("\n{} Found {} cache files totaling {}", 
        "[OK!]".green(), 
        cache_files.len().to_string().cyan(), 
        format_size_with_color(total_size)
    );
    
    if !cache_files.is_empty() {
        // Group files by category
        let mut categories = std::collections::HashMap::new();
        for file in &cache_files {
            categories.entry(file.category).or_insert_with(|| {
                (0, 0u64) // (count, size)
            }).0 += 1;
            categories.entry(file.category).and_modify(|(_count, size)| {
                *size += file.size;
            });
        }
        
        // Print category summary
        println!("\n{}", "Category Summary: ".blue().bold());
        for (category, (count, size)) in categories {
            println!("  {}: {} files ({})", 
                category.as_str().cyan(), 
                count.to_string().cyan(), 
                format_size_with_color(size)
            );
        }
        
        // Prompt to show full file list
        println!("\n{}", "Do you want to see the full list of cache files? (y/N)".yellow());
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");
        
        if input.trim().eq_ignore_ascii_case("y") {
            println!("\n{}", "Cache files: ".blue().bold());
            for file in &cache_files {
                println!("  {} ({}) [{}]\n    {}", 
                    file.path.file_name().unwrap().to_str().unwrap().yellow(),
                    format_size_with_color(file.size),
                    file.category.as_str().magenta(),
                    file.path.display()
                );
            }
        }
        
        // Prompt to delete cache files
        println!("\n{}", "Do you want to delete these cache files? (y/N)".red().bold());
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");
        
        if input.trim().eq_ignore_ascii_case("y") {
            let mut deleted_count = 0;
            let mut deleted_size = 0;
            
            println!("\n{} Deleting cache files...", "🗑️".red());
            
            for file in cache_files {
                match fs::remove_file(&file.path).await {
                    Ok(_) => {
                        println!("  {} Deleted {}", "[OK!]".green(), file.path.display());
                        deleted_count += 1;
                        deleted_size += file.size;
                    }
                    Err(e) => {
                        println!("  {} Failed to delete {}: {}", 
                            "[Failed!]".red(), 
                            file.path.display(), 
                            e.to_string().red()
                        );
                    }
                }
            }
            
            println!("\n{} Deleted {} files totaling {}", 
                "[OK!]".green(), 
                deleted_count.to_string().cyan(), 
                format_size_with_color(deleted_size)
            );
        } else {
            println!("\n{} Deletion canceled", "[OK!]".green());
        }
    }
}



#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Detect { path } => {
            let scan_path = path.unwrap_or_else(|| PathBuf::from("."));
            detect_cache_files(&scan_path).await;
        }
    }
}

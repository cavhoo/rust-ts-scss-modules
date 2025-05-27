use walkdir::{DirEntry, WalkDir};

fn matches_file_type(entry: &DirEntry, file_type: &str) -> bool {
    entry.file_name().to_str().map(|s| {
        s.ends_with(&format!(".{}", file_type))
    }).unwrap_or(false)
}

fn is_node_modules(entry: &DirEntry) -> bool {
    entry.path().to_str().map(|e| e.contains("node_modules")).unwrap_or(false)
}

fn is_yalc(entry: &DirEntry) -> bool {
    entry.path().to_str().map(|e| e.contains(".yalc")).unwrap_or(false)
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.path().to_str().map(|s| s.starts_with(".")).unwrap_or(false)
}

fn is_dist(entry: &DirEntry) -> bool {
    entry.path().to_str().map(|s| s.contains("dist")).unwrap_or(false)
}

pub fn get_scss_files(path: &str) -> impl Iterator<Item = walkdir::DirEntry> {
    println!("Searching for files in: {}", path);
    WalkDir::new(String::from(path))
        .follow_links(false)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !is_node_modules(e))
        .filter(|e| !is_hidden(e))
        .filter(|e| !is_yalc(e))
        .filter(|e| !is_dist(e))
        .filter(|e| matches_file_type(e, "scss"))
}

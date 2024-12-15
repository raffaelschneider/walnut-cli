use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use chrono::Local;

/// Creates a new journal entry using the template from `Templates/Journal/_record.md`
pub fn create_journal_entry(repo_path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    let template_path = repo_path.join("Templates/Journal/_record.md");
    if !template_path.exists() {
        return Err("Journal template not found".into());
    }

    let mut template = String::new();
    File::open(&template_path)?.read_to_string(&mut template)?;

    // Format a date-based filename
    let date_str = Local::now().format("%Y-%m-%d-record.md").to_string();
    let target_path = repo_path.join("Journal").join(&date_str);
    let mut f = File::create(&target_path)?;
    // You might want to do template variable substitution here
    f.write_all(template.as_bytes())?;

    Ok(())
}

/// Creates a new note entry using the template from `Templates/Notes/_note.md`
pub fn create_note_entry(repo_path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    let template_path = repo_path.join("Templates/Notes/_note.md");
    if !template_path.exists() {
        return Err("Note template not found".into());
    }

    let mut template = String::new();
    File::open(&template_path)?.read_to_string(&mut template)?;

    let date_str = Local::now().format("%Y-%m-%d-note.md").to_string();
    let target_path = repo_path.join("Notes/Personal").join(&date_str);
    let mut f = File::create(&target_path)?;
    f.write_all(template.as_bytes())?;

    Ok(())
}


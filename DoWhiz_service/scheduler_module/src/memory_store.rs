use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::RunTaskTask;

const DEFAULT_MEMO_CONTENT: &str = "# Memo\n\n## Profile\n\n## Preferences\n\n## Projects\n\n## Contacts\n\n## Decisions\n\n## Processes\n";

pub(crate) fn ensure_default_user_memo(memory_dir: &Path) -> Result<(), io::Error> {
    fs::create_dir_all(memory_dir)?;
    if has_markdown_files(memory_dir)? {
        return Ok(());
    }
    let memo_path = memory_dir.join("memo.md");
    fs::write(memo_path, DEFAULT_MEMO_CONTENT)?;
    Ok(())
}

pub(crate) fn resolve_user_memory_dir(task: &RunTaskTask) -> Option<PathBuf> {
    if let Some(archive_root) = task.archive_root.as_ref() {
        if let Some(user_root) = archive_root.parent() {
            return Some(user_root.join("memory"));
        }
    }
    let user_root = task.workspace_dir.parent()?.parent()?;
    Some(user_root.join("memory"))
}

pub(crate) fn sync_user_memory_to_workspace(
    user_memory_dir: &Path,
    workspace_memory_dir: &Path,
) -> Result<(), io::Error> {
    if paths_refer_to_same_dir(user_memory_dir, workspace_memory_dir) {
        return Ok(());
    }
    ensure_default_user_memo(user_memory_dir)?;
    fs::create_dir_all(workspace_memory_dir)?;
    clear_markdown_files(workspace_memory_dir)?;
    copy_markdown_files(user_memory_dir, workspace_memory_dir)?;
    Ok(())
}

pub(crate) fn sync_workspace_memory_to_user(
    workspace_memory_dir: &Path,
    user_memory_dir: &Path,
) -> Result<(), io::Error> {
    if paths_refer_to_same_dir(user_memory_dir, workspace_memory_dir) {
        return Ok(());
    }
    fs::create_dir_all(user_memory_dir)?;
    clear_markdown_files(user_memory_dir)?;
    copy_markdown_files(workspace_memory_dir, user_memory_dir)?;
    ensure_default_user_memo(user_memory_dir)?;
    Ok(())
}

fn has_markdown_files(dir: &Path) -> Result<bool, io::Error> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        if entry.file_type()?.is_file() && is_markdown_file(&entry.path()) {
            return Ok(true);
        }
    }
    Ok(false)
}

fn clear_markdown_files(dir: &Path) -> Result<(), io::Error> {
    if !dir.exists() {
        return Ok(());
    }
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        if entry.file_type()?.is_file() && is_markdown_file(&entry.path()) {
            fs::remove_file(entry.path())?;
        }
    }
    Ok(())
}

fn copy_markdown_files(src: &Path, dest: &Path) -> Result<(), io::Error> {
    if !src.exists() {
        return Ok(());
    }
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        if entry.file_type()?.is_file() && is_markdown_file(&entry.path()) {
            let dest_path = dest.join(entry.file_name());
            fs::copy(entry.path(), dest_path)?;
        }
    }
    Ok(())
}

fn is_markdown_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| matches!(ext.to_ascii_lowercase().as_str(), "md" | "markdown"))
        .unwrap_or(false)
}

fn paths_refer_to_same_dir(left: &Path, right: &Path) -> bool {
    if left == right {
        return true;
    }
    let left_canon = fs::canonicalize(left);
    let right_canon = fs::canonicalize(right);
    matches!((left_canon, right_canon), (Ok(left), Ok(right)) if left == right)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn ensure_default_user_memo_creates_memo() {
        let temp = TempDir::new().expect("tempdir");
        let memory_dir = temp.path().join("memory");
        ensure_default_user_memo(&memory_dir).expect("memo created");
        let memo_path = memory_dir.join("memo.md");
        let contents = fs::read_to_string(memo_path).expect("memo contents");
        assert!(contents.contains("# Memo"));
    }

    #[test]
    fn ensure_default_user_memo_skips_when_markdown_exists() {
        let temp = TempDir::new().expect("tempdir");
        let memory_dir = temp.path().join("memory");
        fs::create_dir_all(&memory_dir).expect("memory dir");
        fs::write(memory_dir.join("custom.md"), "custom").expect("custom file");
        ensure_default_user_memo(&memory_dir).expect("memo skip");
        assert!(!memory_dir.join("memo.md").exists());
    }

    #[test]
    fn sync_user_memory_to_workspace_copies_markdown() {
        let temp = TempDir::new().expect("tempdir");
        let user_memory = temp.path().join("user_memory");
        let workspace_memory = temp.path().join("workspace_memory");
        fs::create_dir_all(&user_memory).expect("user memory");
        fs::create_dir_all(&workspace_memory).expect("workspace memory");
        fs::write(user_memory.join("memo.md"), "origin").expect("memo");
        fs::write(workspace_memory.join("old.md"), "old").expect("old memo");

        sync_user_memory_to_workspace(&user_memory, &workspace_memory)
            .expect("sync user to workspace");

        assert!(!workspace_memory.join("old.md").exists());
        let copied = fs::read_to_string(workspace_memory.join("memo.md")).expect("copied memo");
        assert_eq!(copied, "origin");
    }

    #[test]
    fn sync_workspace_memory_to_user_overwrites_markdown() {
        let temp = TempDir::new().expect("tempdir");
        let user_memory = temp.path().join("user_memory");
        let workspace_memory = temp.path().join("workspace_memory");
        fs::create_dir_all(&user_memory).expect("user memory");
        fs::create_dir_all(&workspace_memory).expect("workspace memory");
        fs::write(user_memory.join("memo.md"), "before").expect("memo");
        fs::write(workspace_memory.join("memo.md"), "after").expect("memo");

        sync_workspace_memory_to_user(&workspace_memory, &user_memory)
            .expect("sync workspace to user");

        let updated = fs::read_to_string(user_memory.join("memo.md")).expect("updated memo");
        assert_eq!(updated, "after");
    }
}

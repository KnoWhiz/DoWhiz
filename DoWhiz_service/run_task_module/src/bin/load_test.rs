use run_task_module::{run_task, RunTaskParams};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut count = 10usize;
    let mut concurrency = 4usize;
    let mut reply_required = false;
    let mut workspace_root = PathBuf::from("tmp/load_test/workspaces");
    let mut employee_id = env::var("EMPLOYEE_ID").unwrap_or_else(|_| "little_bear".to_string());
    let mut runner = "codex".to_string();

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--count" => {
                if let Some(value) = args.next() {
                    count = value.parse()?;
                }
            }
            "--concurrency" => {
                if let Some(value) = args.next() {
                    concurrency = value.parse()?;
                }
            }
            "--workspace-root" => {
                if let Some(value) = args.next() {
                    workspace_root = PathBuf::from(value);
                }
            }
            "--reply-required" => {
                reply_required = true;
            }
            "--employee" => {
                if let Some(value) = args.next() {
                    employee_id = value;
                }
            }
            "--runner" => {
                if let Some(value) = args.next() {
                    runner = value;
                }
            }
            "--help" | "-h" => {
                print_help();
                return Ok(());
            }
            _ => {
                eprintln!("Unknown argument: {arg}");
                print_help();
                return Ok(());
            }
        }
    }

    if concurrency == 0 || count == 0 {
        eprintln!("count and concurrency must be > 0");
        return Ok(());
    }

    fs::create_dir_all(&workspace_root)?;

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let skills_src = manifest_dir.join("..").join("skills");
    let employee_dir = manifest_dir.join("..").join("employees").join(&employee_id);

    let mut params_list = Vec::with_capacity(count);
    for idx in 0..count {
        let workspace_dir = workspace_root.join(format!("task_{idx:04}"));
        prepare_workspace(&workspace_dir)?;
        write_sample_email(&workspace_dir, idx)?;
        write_sample_memory(&workspace_dir, idx)?;
        copy_skills(&skills_src, &workspace_dir.join(".agents").join("skills"))?;
        copy_employee_guidance(&employee_dir, &workspace_dir)?;

        let reply_to = if reply_required {
            vec!["loadtest@example.com".to_string()]
        } else {
            Vec::new()
        };

        params_list.push(RunTaskParams {
            workspace_dir,
            input_email_dir: PathBuf::from("incoming_email"),
            input_attachments_dir: PathBuf::from("incoming_attachments"),
            memory_dir: PathBuf::from("memory"),
            reference_dir: PathBuf::from("references"),
            reply_to,
            model_name: "gpt-5.2-codex".to_string(),
            runner: runner.clone(),
            codex_disabled: false,
            channel: "email".to_string(),
            google_access_token: None,
            has_unified_account: false,
        });
    }

    let receiver = Arc::new(Mutex::new(params_list.into_iter()));
    let successes = Arc::new(Mutex::new(0usize));
    let failures = Arc::new(Mutex::new(Vec::new()));
    let start = Instant::now();

    let mut handles = Vec::with_capacity(concurrency);
    for _ in 0..concurrency {
        let receiver = receiver.clone();
        let successes = successes.clone();
        let failures = failures.clone();
        handles.push(thread::spawn(move || loop {
            let params = {
                let mut iter = receiver.lock().expect("load test iterator poisoned");
                iter.next()
            };
            let Some(params) = params else { break };

            match run_task(&params) {
                Ok(_) => {
                    let mut ok = successes.lock().expect("success counter poisoned");
                    *ok += 1;
                }
                Err(err) => {
                    let mut errs = failures.lock().expect("failure list poisoned");
                    errs.push(err.to_string());
                }
            }
        }));
    }

    for handle in handles {
        let _ = handle.join();
    }

    let elapsed = start.elapsed();
    let ok = *successes
        .lock()
        .unwrap_or_else(|poison| poison.into_inner());
    let errs = failures.lock().unwrap_or_else(|poison| poison.into_inner());
    println!(
        "load_test finished: total={} ok={} failed={} elapsed={:?}",
        count,
        ok,
        errs.len(),
        elapsed
    );

    if !errs.is_empty() {
        for (idx, err) in errs.iter().take(5).enumerate() {
            eprintln!("failure[{}]: {}", idx + 1, err);
        }
        std::process::exit(1);
    }

    Ok(())
}

fn print_help() {
    println!(
        "Usage: cargo run -p run_task_module --bin load_test -- [options]\n\
Options:\n\
  --count <n>            Number of tasks (default: 10)\n\
  --concurrency <n>      Parallel workers (default: 4)\n\
  --workspace-root <p>   Root dir for workspaces (default: tmp/load_test/workspaces)\n\
  --reply-required       Require reply files (reply_email_draft.html)\n\
  --employee <id>        Employee id for guidance files (default: little_bear)\n\
  --runner <name>        Runner name (default: codex)\n\
  --help, -h             Show this help\n"
    );
}

fn prepare_workspace(workspace: &Path) -> std::io::Result<()> {
    fs::create_dir_all(workspace.join("incoming_email"))?;
    fs::create_dir_all(workspace.join("incoming_attachments"))?;
    fs::create_dir_all(workspace.join("memory"))?;
    fs::create_dir_all(workspace.join("references"))?;
    Ok(())
}

fn write_sample_email(workspace: &Path, idx: usize) -> std::io::Result<()> {
    let email_path = workspace.join("incoming_email").join("email.txt");
    let body = format!(
        "Subject: Load test task {idx}\n\nPlease summarize the task id ({idx}) in one short sentence and confirm completion."
    );
    fs::write(email_path, body)
}

fn write_sample_memory(workspace: &Path, idx: usize) -> std::io::Result<()> {
    let memo_path = workspace.join("memory").join("memo.md");
    let body = format!("# Memory\n\n- User: load_test_user_{idx}\n- Preference: concise replies\n");
    fs::write(memo_path, body)
}

fn copy_skills(src: &Path, dest: &Path) -> std::io::Result<()> {
    if !src.exists() {
        return Ok(());
    }
    copy_dir_recursive(src, dest)
}

fn copy_employee_guidance(employee_dir: &Path, workspace: &Path) -> std::io::Result<()> {
    for filename in ["AGENTS.md", "CLAUDE.md", "SOUL.md"] {
        let src = employee_dir.join(filename);
        if src.exists() {
            fs::copy(src, workspace.join(filename))?;
        }
    }
    Ok(())
}

fn copy_dir_recursive(src: &Path, dest: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dest)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dest_path = dest.join(entry.file_name());
        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dest_path)?;
        } else {
            fs::copy(&src_path, &dest_path)?;
        }
    }
    Ok(())
}

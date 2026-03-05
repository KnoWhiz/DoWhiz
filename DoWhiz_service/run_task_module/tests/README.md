# run_task_module tests

## Run

```bash
cd DoWhiz_service
cargo test -p run_task_module
```

Primary coverage includes:
- workspace input/output path validation
- channel-aware reply file generation
- prompt construction and scheduler block extraction
- timeout/error behavior
- GitHub/x402 env injection behavior
- backend selection (`local` vs `azure_aci` policy)

## Optional live Codex path

`run_task_tests` can execute real Codex when enabled.

```bash
cd DoWhiz_service
RUN_CODEX_E2E=1 \
AZURE_OPENAI_API_KEY_BACKUP=... \
cargo test -p run_task_module --test run_task_tests -- --nocapture
```

## Notes

- Live execution requires local CLI/runtime prerequisites (Codex CLI and related deps).
- Keep live tests opt-in to avoid accidental cost.

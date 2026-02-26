# Contributing to DoWhiz

Thanks for contributing! We keep setup, run, and testing instructions in `README.md`. Please start there for prerequisites, dev commands, and links to component-specific docs (Rust service and website).

Deployment and config policy:
- Production deploy branch is `main` (CI/CD baseline).
- Staging deploy branch is `dev` (CI/CD rollout target).
- Keep one `DoWhiz_service/.env`; use base keys for production and `STAGING_`-prefixed keys for staging.
- For exact staging/prod runbooks and rollback, follow `DoWhiz_service/docs/staging_production_deploy.md`.

When opening a PR, include a short summary, tests run, and screenshots for UI changes.

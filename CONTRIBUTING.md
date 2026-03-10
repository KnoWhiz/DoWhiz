# Contributing to DoWhiz

## Development Flow

1. Branch from latest `dev` for non-trivial work.
2. Keep commits scoped and easy to review.
3. Update related docs when behavior/config changes.

## Documentation and Accuracy

- Code behavior is source of truth.
- If docs conflict with code, update docs in the same PR.

## Testing

- For `DoWhiz_service` changes, use:
  `reference_documentation/test_plans/DoWhiz_service_tests.md`
- Run relevant AUTO tests.
- For LIVE/MANUAL/PLANNED entries, mark SKIP with reason unless explicitly executed.

## Deployment Policy

- Production deploy branch: `main`
- Staging deploy branch: `dev`
- Runtime `.env` on VM must use unprefixed keys only.
- VM `.env` should be merged from secret sets (`ENV_COMMON + ENV_STAGING/ENV_PROD`) per CI workflow logic.

## PR Checklist

- Short summary of changes
- Test commands and results
- Any env/config migration notes
- Screenshots for UI changes (if applicable)

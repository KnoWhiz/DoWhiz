# DoWhiz - Email-first digital employees

<p align="center">
  <img src="assets/readme-banner.svg" alt="DoWhiz - Email-first digital employees" width="1200" />
</p>

<p align="center">
  <a href="LICENSE">
    <img alt="License: Apache 2.0" src="https://img.shields.io/badge/License-Apache%202.0-0f172a?style=for-the-badge" />
  </a>
  <a href="DoWhiz_service/README.md">
    <img alt="Rust service" src="https://img.shields.io/badge/Rust-Service-0ea5e9?style=for-the-badge&logo=rust&logoColor=white" />
  </a>
  <a href="website/README.md">
    <img alt="React website" src="https://img.shields.io/badge/React-Website-3b82f6?style=for-the-badge&logo=react&logoColor=white" />
  </a>
  <img alt="Email first" src="https://img.shields.io/badge/Email-First-6366f1?style=for-the-badge" />
</p>

Send tasks by email and get structured work and results back in the same
channel.

## Overview
DoWhiz lets users send tasks to role-based agents over email. The platform
routes, schedules, executes, and replies with results, follow-ups, and
scheduled work.

## Core capabilities
- Email-first task intake and replies.
- Role-based agents with isolated, user-specific memory and data.
- Scheduling and orchestration for long-running or recurring work.
- Tool-backed execution for reliable outputs.

## High-level architecture
```
Inbound email -> Scheduler -> Task runner -> Tools -> Outbound email
```

## Vision
The long-term product direction lives in `vision.md`: a multi-agent,
multi-tenant platform where each user has isolated memory/data, agents act
like teammates, and the system escalates when human input is needed.

## Repository layout
- `DoWhiz_service/`: Rust service for inbound email, scheduling, task execution,
  and outbound replies.
- `website/`: Product website.
- `landing/`: Landing page experiments/assets.
- `api_reference_documentation/`: Postmark/Gmail API references (reference-only).
- `example_files/`: Sample inputs/outputs used for testing and demos.
- `external/`: Vendored references (including OpenClaw).
- `scripts/`: Helper scripts.

## Getting started
Rust service:
```
cargo run -p scheduler_module --bin rust_service -- --host 0.0.0.0 --port 9001
```

Website:
```
cd website
npm run dev
```

More detail:
- `DoWhiz_service/README.md`
- `website/README.md`

## Testing
Rust unit tests:
```
cargo test -p scheduler_module
cargo test -p run_task_module
```

Website lint:
```
cd website
npm run lint
```

## License
See `LICENSE`.

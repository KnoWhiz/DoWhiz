# DoWhiz_service Test Checklist

Scope: DoWhiz_service only. Excludes website/ and function_app/.

Status legend:
AUTO = implemented and can run locally without external creds.
LIVE = implemented but requires external creds/services or manual enable flags.
MANUAL = no automated test; must be verified by hand.
PLANNED = not implemented; should be added.

Reporting rule:
After any code change, consult this checklist, run all relevant AUTO tests, and explicitly mark LIVE/MANUAL/PLANNED with reason. Use the Test Report Template at the end of this document.

## Unit Tests: run_task_module
| ID | Test | Target (file::function/module) | Test File | Verifies | Does Not Verify | Status | Run/Env |
|---|---|---|---|---|---|---|---|
| UT-RUN-01 | load_memory_context_sorts_and_includes_markdown | run_task_module/run_task.rs::load_memory_context | DoWhiz_service/run_task_module/run_task.rs | Markdown read + sorted ordering | Large files, encoding errors, IO failures | AUTO | cargo test -p run_task_module |
| UT-RUN-02 | build_prompt_includes_memory_policy_and_section | run_task_module/run_task.rs::build_prompt | DoWhiz_service/run_task_module/run_task.rs | Prompt includes memory policy and section | Prompt quality/LLM correctness | AUTO | cargo test -p run_task_module |
| UT-RUN-03 | build_prompt_skips_reply_instruction_for_non_replyable | run_task_module/run_task.rs::build_prompt | DoWhiz_service/run_task_module/run_task.rs | Non-replyable branch | Channel-specific variants | AUTO | cargo test -p run_task_module |
| UT-RUN-04 | extract_scheduler_actions_returns_empty_when_missing | run_task_module/run_task.rs::extract_scheduler_actions | DoWhiz_service/run_task_module/run_task.rs | Safe empty return | Multiple blocks handling | AUTO | cargo test -p run_task_module |
| UT-RUN-05 | extract_scheduler_actions_parses_list | run_task_module/run_task.rs::extract_scheduler_actions | DoWhiz_service/run_task_module/run_task.rs | Parse cancel action list | All action types | AUTO | cargo test -p run_task_module |
| UT-RUN-06 | extract_scheduler_actions_reports_invalid_json | run_task_module/run_task.rs::extract_scheduler_actions | DoWhiz_service/run_task_module/run_task.rs | Error path on invalid JSON | Error message detail | AUTO | cargo test -p run_task_module |

## Unit Tests: scheduler_module (core/data/stores)
| ID | Test | Target (file::function/module) | Test File | Verifies | Does Not Verify | Status | Run/Env |
|---|---|---|---|---|---|---|---|
| UT-SCH-01 | build_scheduler_snapshot_limits_to_window | scheduler_module/src/scheduler/snapshot | DoWhiz_service/scheduler_module/src/scheduler/tests.rs | Snapshot window trimming | Performance with large task sets | AUTO | cargo test -p scheduler_module |
| UT-SCH-02 | apply_scheduler_actions_cancels_and_reschedules | scheduler_module/src/scheduler/actions::apply_scheduler_actions | DoWhiz_service/scheduler_module/src/scheduler/tests.rs | Cancel + reschedule semantics | Real run_task output compatibility | AUTO | cargo test -p scheduler_module |
| UT-SCH-03 | apply_scheduler_actions_creates_run_task | scheduler_module/src/scheduler/actions::apply_scheduler_actions | DoWhiz_service/scheduler_module/src/scheduler/tests.rs | create_run_task task wiring | Field inheritance edge cases | AUTO | cargo test -p scheduler_module |
| UT-SCH-04 | enqueue_and_claim_roundtrip | scheduler_module/src/ingestion_queue::IngestionQueue | DoWhiz_service/scheduler_module/src/ingestion_queue.rs | enqueue/claim/mark_done | Multi-worker race | AUTO | cargo test -p scheduler_module |
| UT-SCH-05 | enqueue_dedupe_prevents_duplicates | scheduler_module/src/ingestion_queue::IngestionQueue | DoWhiz_service/scheduler_module/src/ingestion_queue.rs | Dedupe key uniqueness | Concurrent insertions | AUTO | cargo test -p scheduler_module |
| UT-SCH-06 | ensure_default_user_memo_creates_memo | scheduler_module/src/memory_store::ensure_default_user_memo | DoWhiz_service/scheduler_module/src/memory_store.rs | Default memo creation | Permissions, IO failures | AUTO | cargo test -p scheduler_module |
| UT-SCH-07 | ensure_default_user_memo_skips_when_markdown_exists | scheduler_module/src/memory_store::ensure_default_user_memo | DoWhiz_service/scheduler_module/src/memory_store.rs | Skip when md exists | Mixed extensions | AUTO | cargo test -p scheduler_module |
| UT-SCH-08 | sync_user_memory_to_workspace_copies_markdown | scheduler_module/src/memory_store::sync_user_memory_to_workspace | DoWhiz_service/scheduler_module/src/memory_store.rs | user -> workspace sync | Merge strategy | AUTO | cargo test -p scheduler_module |
| UT-SCH-09 | sync_workspace_memory_to_user_overwrites_markdown | scheduler_module/src/memory_store::sync_workspace_memory_to_user | DoWhiz_service/scheduler_module/src/memory_store.rs | workspace -> user overwrite | Conflict resolution | AUTO | cargo test -p scheduler_module |
| UT-SCH-10 | resolve_user_secrets_path_prefers_archive_root | scheduler_module/src/secrets_store::resolve_user_secrets_path | DoWhiz_service/scheduler_module/src/secrets_store.rs | secrets path resolution | Nonstandard layouts | AUTO | cargo test -p scheduler_module |
| UT-SCH-11 | sync_user_secrets_to_workspace_copies_env | scheduler_module/src/secrets_store::sync_user_secrets_to_workspace | DoWhiz_service/scheduler_module/src/secrets_store.rs | Copy secrets to workspace | Encryption, redaction | AUTO | cargo test -p scheduler_module |
| UT-SCH-12 | sync_user_secrets_to_workspace_creates_empty_env_when_missing | scheduler_module/src/secrets_store::sync_user_secrets_to_workspace | DoWhiz_service/scheduler_module/src/secrets_store.rs | Creates empty env | Read-only FS | AUTO | cargo test -p scheduler_module |
| UT-SCH-13 | upsert_and_get_installation | scheduler_module/src/slack_store::SlackStore | DoWhiz_service/scheduler_module/src/slack_store.rs | Upsert + get | Concurrency | AUTO | cargo test -p scheduler_module |
| UT-SCH-14 | upsert_updates_existing | scheduler_module/src/slack_store::SlackStore | DoWhiz_service/scheduler_module/src/slack_store.rs | Update behavior | Partial updates | AUTO | cargo test -p scheduler_module |
| UT-SCH-15 | get_not_found | scheduler_module/src/slack_store::SlackStore | DoWhiz_service/scheduler_module/src/slack_store.rs | NotFound error | Env fallback path | AUTO | cargo test -p scheduler_module |
| UT-SCH-16 | delete_installation | scheduler_module/src/slack_store::SlackStore | DoWhiz_service/scheduler_module/src/slack_store.rs | Delete result | Transaction rollback | AUTO | cargo test -p scheduler_module |
| UT-SCH-17 | list_installations | scheduler_module/src/slack_store::SlackStore | DoWhiz_service/scheduler_module/src/slack_store.rs | List count | Ordering stability | AUTO | cargo test -p scheduler_module |
| UT-SCH-18 | sync_user_tasks_and_query_due_users | scheduler_module/src/index_store::IndexStore | DoWhiz_service/scheduler_module/src/index_store/tests/mod.rs | due users query | Timezone/DST | AUTO | cargo test -p scheduler_module |
| UT-SCH-19 | normalize_email_handles_tags_and_case | scheduler_module/src/user_store::normalize_email | DoWhiz_service/scheduler_module/src/user_store/tests/mod.rs | Email normalization | Internationalized addresses | AUTO | cargo test -p scheduler_module |
| UT-SCH-20 | normalize_phone_handles_various_formats | scheduler_module/src/user_store::normalize_phone | DoWhiz_service/scheduler_module/src/user_store/tests/mod.rs | Phone normalization | Country formats | AUTO | cargo test -p scheduler_module |
| UT-SCH-21 | normalize_slack_id_uppercases | scheduler_module/src/user_store::normalize_slack_id | DoWhiz_service/scheduler_module/src/user_store/tests/mod.rs | Slack ID normalization | Invalid chars | AUTO | cargo test -p scheduler_module |
| UT-SCH-22 | extract_emails_finds_all_candidates | scheduler_module/src/user_store::extract_emails | DoWhiz_service/scheduler_module/src/user_store/tests/mod.rs | Extract multiple emails | Internationalized addresses | AUTO | cargo test -p scheduler_module |
| UT-SCH-23 | user_store_get_or_create_is_stable_for_email | scheduler_module/src/user_store::UserStore | DoWhiz_service/scheduler_module/src/user_store/tests/mod.rs | Stable user id for email | Concurrent creation | AUTO | cargo test -p scheduler_module |
| UT-SCH-24 | user_store_get_or_create_is_stable_for_phone | scheduler_module/src/user_store::UserStore | DoWhiz_service/scheduler_module/src/user_store/tests/mod.rs | Stable user id for phone | Invalid phone handling | AUTO | cargo test -p scheduler_module |
| UT-SCH-25 | user_store_get_or_create_is_stable_for_slack | scheduler_module/src/user_store::UserStore | DoWhiz_service/scheduler_module/src/user_store/tests/mod.rs | Stable user id for slack | Workspace separation | AUTO | cargo test -p scheduler_module |
| UT-SCH-26 | user_store_separates_by_identifier_type | scheduler_module/src/user_store::UserStore | DoWhiz_service/scheduler_module/src/user_store/tests/mod.rs | Type isolation | Multi-factor IDs | AUTO | cargo test -p scheduler_module |
| UT-SCH-27 | list_user_ids_returns_all_users | scheduler_module/src/user_store::UserStore | DoWhiz_service/scheduler_module/src/user_store/tests/mod.rs | List size | Large datasets | AUTO | cargo test -p scheduler_module |
| UT-SCH-28 | selects_service_mailbox_with_display_name | scheduler_module/src/mailbox::select_inbound_service_mailbox | DoWhiz_service/scheduler_module/src/mailbox.rs | Display name parsing | Encoded headers | AUTO | cargo test -p scheduler_module |
| UT-SCH-29 | selects_service_mailbox_without_display_name | scheduler_module/src/mailbox::select_inbound_service_mailbox | DoWhiz_service/scheduler_module/src/mailbox.rs | No display name case | Multiple addresses | AUTO | cargo test -p scheduler_module |
| UT-SCH-30 | rejects_non_service_address | scheduler_module/src/mailbox::select_inbound_service_mailbox | DoWhiz_service/scheduler_module/src/mailbox.rs | Non-service address rejection | Aliases | AUTO | cargo test -p scheduler_module |
| UT-SCH-31 | archive_outbound_writes_payload_and_attachments | scheduler_module/src/past_emails::archive_outbound | DoWhiz_service/scheduler_module/src/past_emails.rs | Outbound archive structure | Large attachments | AUTO | cargo test -p scheduler_module |
| UT-SCH-32 | router_config_defaults | scheduler_module/src/message_router::RouterConfig | DoWhiz_service/scheduler_module/src/message_router.rs | Router defaults | Actual HTTP behavior | AUTO | cargo test -p scheduler_module |
| UT-SCH-33 | forward_marker_detected | scheduler_module/src/message_router::FORWARD_MARKER | DoWhiz_service/scheduler_module/src/message_router.rs | Marker detection | Real LLM output | AUTO | cargo test -p scheduler_module |
| UT-SCH-34 | test_config_validation | scheduler_module/src/google_auth::GoogleAuthConfig | DoWhiz_service/scheduler_module/src/google_auth.rs | Config validation | OAuth refresh | AUTO | cargo test -p scheduler_module |

## Unit Tests: scheduler_module (adapters)
| ID | Test | Target (file::function/module) | Test File | Verifies | Does Not Verify | Status | Run/Env |
|---|---|---|---|---|---|---|---|
| UT-AD-SL-01 | parse_url_verification_challenge | adapters/slack.rs::is_url_verification | DoWhiz_service/scheduler_module/src/adapters/slack.rs | URL verification parsing | Signature verification | AUTO | cargo test -p scheduler_module |
| UT-AD-SL-02 | parse_message_event | adapters/slack.rs::SlackInboundAdapter::parse | DoWhiz_service/scheduler_module/src/adapters/slack.rs | Basic message parsing | Real webhook headers | AUTO | cargo test -p scheduler_module |
| UT-AD-SL-03 | parse_threaded_message | adapters/slack.rs::SlackInboundAdapter::parse | DoWhiz_service/scheduler_module/src/adapters/slack.rs | thread_ts parsing | Real thread semantics | AUTO | cargo test -p scheduler_module |
| UT-AD-SL-04 | ignore_bot_messages | adapters/slack.rs::SlackInboundAdapter::is_bot_message | DoWhiz_service/scheduler_module/src/adapters/slack.rs | Bot message filtering | Bot list refresh | AUTO | cargo test -p scheduler_module |
| UT-AD-SL-05 | ignore_message_subtypes | adapters/slack.rs::SlackInboundAdapter::parse | DoWhiz_service/scheduler_module/src/adapters/slack.rs | Subtype filtering | Other subtypes | AUTO | cargo test -p scheduler_module |
| UT-AD-SL-06 | parse_message_with_files | adapters/slack.rs::SlackInboundAdapter::parse | DoWhiz_service/scheduler_module/src/adapters/slack.rs | File metadata parsing | File download | AUTO | cargo test -p scheduler_module |
| UT-AD-SL-07 | missing_user_field_errors | adapters/slack.rs::SlackInboundAdapter::parse | DoWhiz_service/scheduler_module/src/adapters/slack.rs | Missing field error | Recovery strategy | AUTO | cargo test -p scheduler_module |
| UT-AD-SL-08 | missing_channel_field_errors | adapters/slack.rs::SlackInboundAdapter::parse | DoWhiz_service/scheduler_module/src/adapters/slack.rs | Missing field error | Recovery strategy | AUTO | cargo test -p scheduler_module |
| UT-AD-SL-09 | own_bot_user_id_filtered | adapters/slack.rs::SlackInboundAdapter::is_bot_message | DoWhiz_service/scheduler_module/src/adapters/slack.rs | Own bot id filtering | Multi-bot env | AUTO | cargo test -p scheduler_module |
| UT-AD-SL-10 | non_bot_user_not_filtered | adapters/slack.rs::SlackInboundAdapter::is_bot_message | DoWhiz_service/scheduler_module/src/adapters/slack.rs | Non-bot passes | Workspace separation | AUTO | cargo test -p scheduler_module |
| UT-AD-SL-11 | thread_id_uses_ts_when_no_thread_ts | adapters/slack.rs::SlackInboundAdapter::parse | DoWhiz_service/scheduler_module/src/adapters/slack.rs | thread_id fallback | Real thread creation | AUTO | cargo test -p scheduler_module |
| UT-AD-SL-12 | unsupported_event_type_errors | adapters/slack.rs::SlackInboundAdapter::parse | DoWhiz_service/scheduler_module/src/adapters/slack.rs | Unsupported event error | Other event types | AUTO | cargo test -p scheduler_module |
| UT-AD-SL-13 | empty_text_message_parses | adapters/slack.rs::SlackInboundAdapter::parse | DoWhiz_service/scheduler_module/src/adapters/slack.rs | Empty text handling | Empty file payload | AUTO | cargo test -p scheduler_module |
| UT-AD-DC-01 | bot_user_id_filtered | adapters/discord.rs::DiscordInboundAdapter::is_bot_message | DoWhiz_service/scheduler_module/src/adapters/discord.rs | Bot id filtering | Real webhook | AUTO | cargo test -p scheduler_module |
| UT-AD-DC-02 | bot_flag_filtered | adapters/discord.rs::DiscordInboundAdapter::is_bot_message | DoWhiz_service/scheduler_module/src/adapters/discord.rs | Bot flag filtering | Role-based bots | AUTO | cargo test -p scheduler_module |
| UT-AD-DC-03 | create_message_request_serializes | adapters/discord.rs::DiscordCreateMessageRequest | DoWhiz_service/scheduler_module/src/adapters/discord.rs | JSON serialization | API acceptance | AUTO | cargo test -p scheduler_module |
| UT-AD-DC-04 | create_message_request_with_reference | adapters/discord.rs::DiscordCreateMessageRequest | DoWhiz_service/scheduler_module/src/adapters/discord.rs | Reply reference | Threading in API | AUTO | cargo test -p scheduler_module |
| UT-AD-DC-05 | message_response_deserializes | adapters/discord.rs::DiscordMessageResponse | DoWhiz_service/scheduler_module/src/adapters/discord.rs | Response parsing | Schema drift | AUTO | cargo test -p scheduler_module |
| UT-AD-TG-01 | parse_text_message | adapters/telegram.rs::TelegramInboundAdapter::parse | DoWhiz_service/scheduler_module/src/adapters/telegram.rs | Text parse | Real webhook | AUTO | cargo test -p scheduler_module |
| UT-AD-TG-02 | parse_group_message | adapters/telegram.rs::TelegramInboundAdapter::parse | DoWhiz_service/scheduler_module/src/adapters/telegram.rs | Group thread_id | Permissions | AUTO | cargo test -p scheduler_module |
| UT-AD-TG-03 | ignore_bot_messages | adapters/telegram.rs::TelegramInboundAdapter::parse | DoWhiz_service/scheduler_module/src/adapters/telegram.rs | Bot filtering | Bot list updates | AUTO | cargo test -p scheduler_module |
| UT-AD-TG-04 | parse_message_with_photo | adapters/telegram.rs::TelegramInboundAdapter::parse | DoWhiz_service/scheduler_module/src/adapters/telegram.rs | Photo attachment parsing | Actual file download | AUTO | cargo test -p scheduler_module |
| UT-AD-TG-05 | parse_edited_message | adapters/telegram.rs::TelegramInboundAdapter::parse | DoWhiz_service/scheduler_module/src/adapters/telegram.rs | Edited message parsing | Edit history | AUTO | cargo test -p scheduler_module |
| UT-AD-BB-01 | parse_new_message_webhook | adapters/bluebubbles.rs::BlueBubblesInboundAdapter::parse | DoWhiz_service/scheduler_module/src/adapters/bluebubbles.rs | New message parsing | Real webhook | AUTO | cargo test -p scheduler_module |
| UT-AD-BB-02 | ignore_outgoing_messages | adapters/bluebubbles.rs::BlueBubblesInboundAdapter::parse | DoWhiz_service/scheduler_module/src/adapters/bluebubbles.rs | Outgoing filter | Multi-device | AUTO | cargo test -p scheduler_module |
| UT-AD-BB-03 | ignore_non_message_events | adapters/bluebubbles.rs::BlueBubblesInboundAdapter::parse | DoWhiz_service/scheduler_module/src/adapters/bluebubbles.rs | Non-message filter | Other event types | AUTO | cargo test -p scheduler_module |
| UT-AD-BB-04 | parse_message_with_attachments | adapters/bluebubbles.rs::BlueBubblesInboundAdapter::parse | DoWhiz_service/scheduler_module/src/adapters/bluebubbles.rs | Attachment parsing | Actual file download | AUTO | cargo test -p scheduler_module |
| UT-AD-BB-05 | parse_group_chat_message | adapters/bluebubbles.rs::BlueBubblesInboundAdapter::parse | DoWhiz_service/scheduler_module/src/adapters/bluebubbles.rs | Group chat parsing | Participant mapping | AUTO | cargo test -p scheduler_module |
| UT-AD-PM-01 | parse_simple_postmark_payload | adapters/postmark.rs::PostmarkInboundAdapter::parse | DoWhiz_service/scheduler_module/src/adapters/postmark.rs | Basic email parse | Real webhook headers | AUTO | cargo test -p scheduler_module |
| UT-AD-PM-02 | extract_thread_key_from_references | adapters/postmark.rs::PostmarkInboundAdapter::extract_thread_key | DoWhiz_service/scheduler_module/src/adapters/postmark.rs | References extraction | Multiple references parsing | AUTO | cargo test -p scheduler_module |
| UT-AD-PM-03 | replyable_recipients_filters_noreply | adapters/postmark.rs::replyable_recipients | DoWhiz_service/scheduler_module/src/adapters/postmark.rs | no-reply filtering | Complex headers | AUTO | cargo test -p scheduler_module |
| UT-AD-GD-01 | test_employee_mention_detection | adapters/google_docs.rs::contains_employee_mention | DoWhiz_service/scheduler_module/src/adapters/google_docs.rs | Mention detection | NLP edge cases | AUTO | cargo test -p scheduler_module |
| UT-AD-GD-02 | test_extract_employee_name | adapters/google_docs.rs::extract_employee_name | DoWhiz_service/scheduler_module/src/adapters/google_docs.rs | Name extraction | Multi-language | AUTO | cargo test -p scheduler_module |
| UT-AD-GD-03 | test_format_edit_proposal | adapters/google_docs.rs::format_edit_proposal | DoWhiz_service/scheduler_module/src/adapters/google_docs.rs | Proposal formatting | Real Docs API | AUTO | cargo test -p scheduler_module |

## Unit Tests: scheduler_module (service/config)
| ID | Test | Target (file::function/module) | Test File | Verifies | Does Not Verify | Status | Run/Env |
|---|---|---|---|---|---|---|---|
| UT-SVC-01 | resolve_telegram_bot_token_prefers_employee_specific_env | service/config.rs::resolve_telegram_bot_token | DoWhiz_service/scheduler_module/src/service/config.rs | Env priority | Env missing in prod | AUTO | cargo test -p scheduler_module |
| UT-SVC-02 | resolve_telegram_bot_token_falls_back_to_address_then_global | service/config.rs::resolve_telegram_bot_token | DoWhiz_service/scheduler_module/src/service/config.rs | Fallback order | Multi-employee conflicts | AUTO | cargo test -p scheduler_module |
| UT-SVC-03 | resolve_telegram_bot_token_uses_global_when_employee_missing | service/config.rs::resolve_telegram_bot_token | DoWhiz_service/scheduler_module/src/service/config.rs | Global fallback | Token rotation | AUTO | cargo test -p scheduler_module |
| UT-SVC-04 | replyable_recipients_filters_no_reply_addresses | service/recipients.rs::replyable_recipients | DoWhiz_service/scheduler_module/src/service/recipients.rs | no-reply filtering | Complex headers | AUTO | cargo test -p scheduler_module |
| UT-SVC-05 | replyable_recipients_returns_empty_when_only_no_reply | service/recipients.rs::replyable_recipients | DoWhiz_service/scheduler_module/src/service/recipients.rs | no-reply only | List parsing | AUTO | cargo test -p scheduler_module |
| UT-SVC-06 | replyable_recipients_keeps_quoted_display_name_commas | service/recipients.rs::replyable_recipients | DoWhiz_service/scheduler_module/src/service/recipients.rs | Quoted commas | Nested quotes | AUTO | cargo test -p scheduler_module |
| UT-SVC-07 | no_reply_detection_matches_common_variants | service/recipients.rs::is_no_reply_address | DoWhiz_service/scheduler_module/src/service/recipients.rs | Common variants | Domain-based rules | AUTO | cargo test -p scheduler_module |
| UT-SVC-08 | no_reply_detection_requires_exact_local_part | service/recipients.rs::is_no_reply_address | DoWhiz_service/scheduler_module/src/service/recipients.rs | Exact match | Alias handling | AUTO | cargo test -p scheduler_module |
| UT-SVC-09 | no_reply_detection_ignores_domain_markers | service/recipients.rs::is_no_reply_address | DoWhiz_service/scheduler_module/src/service/recipients.rs | Domain markers ignored | Provider-specific rules | AUTO | cargo test -p scheduler_module |
| UT-SVC-10 | process_sms_message_creates_run_task | service/inbound/sms.rs::process_sms_message | DoWhiz_service/scheduler_module/src/service/inbound/sms.rs | SMS inbound pipeline | Real Twilio webhook | AUTO | cargo test -p scheduler_module |
| UT-SVC-11 | create_workspace_hydrates_past_emails | service/email.rs::ensure_thread_workspace | DoWhiz_service/scheduler_module/src/service/email.rs | past_emails hydration | Large archives | AUTO | cargo test -p scheduler_module |
| UT-SVC-12 | stop_and_join_returns_quickly_with_short_watchdog_interval | service/scheduler.rs::start_scheduler_threads | DoWhiz_service/scheduler_module/src/service/scheduler.rs | stop/join behavior | Watchdog recovery | AUTO | cargo test -p scheduler_module |

## Integration/E2E: run_task_module
| ID | Test | Target (file::function/module) | Test File | Verifies | Does Not Verify | Status | Run/Env |
|---|---|---|---|---|---|---|---|
| IT-RUN-01 | run_task_success_with_fake_codex | run_task_module::run_task | DoWhiz_service/run_task_module/tests/run_task_tests.rs | Fake codex output, reply HTML, attachments dir, config write | Real codex behavior | AUTO | cargo test -p run_task_module |
| IT-RUN-02 | run_task_reports_missing_output | run_task_module::run_task | DoWhiz_service/run_task_module/tests/run_task_tests.rs | Missing output error | Real codex edge cases | AUTO | cargo test -p run_task_module |
| IT-RUN-03 | run_task_reports_codex_failure | run_task_module::run_task | DoWhiz_service/run_task_module/tests/run_task_tests.rs | Error mapping on failure | stderr parsing quality | AUTO | cargo test -p run_task_module |
| IT-RUN-04 | run_task_reports_missing_codex_cli | run_task_module::run_task | DoWhiz_service/run_task_module/tests/run_task_tests.rs | CLI not found error | Other runners | AUTO | cargo test -p run_task_module |
| IT-RUN-05 | run_task_maps_github_env_from_dotenv | run_task_module::run_task | DoWhiz_service/run_task_module/tests/run_task_tests.rs | GitHub env injection | Real gh auth | AUTO | cargo test -p run_task_module |
| IT-RUN-06 | run_task_maps_employee_github_env_from_dotenv | run_task_module::run_task | DoWhiz_service/run_task_module/tests/run_task_tests.rs | Employee GitHub env | Multi-employee conflicts | AUTO | cargo test -p run_task_module |
| IT-RUN-07 | run_task_reports_missing_env | run_task_module::run_task | DoWhiz_service/run_task_module/tests/run_task_tests.rs | Required env validation | Other keys | AUTO | cargo test -p run_task_module |
| IT-RUN-08 | run_task_rejects_absolute_input_dir | run_task_module::run_task | DoWhiz_service/run_task_module/tests/run_task_tests.rs | Path validation | Symlink bypass | AUTO | cargo test -p run_task_module |
| IT-RUN-09 | run_task_codex_disabled_writes_placeholder | run_task_module::run_task | DoWhiz_service/run_task_module/tests/run_task_tests.rs | Placeholder output | Actual HTML template | AUTO | cargo test -p run_task_module |
| IT-RUN-10 | run_task_codex_disabled_skips_placeholder_without_reply_to | run_task_module::run_task | DoWhiz_service/run_task_module/tests/run_task_tests.rs | No reply_to handling | Channel variations | AUTO | cargo test -p run_task_module |
| IT-RUN-11 | run_task_real_codex_e2e_when_enabled | run_task_module::run_task | DoWhiz_service/run_task_module/tests/run_task_tests.rs | Real codex CLI flow | Content quality and tools | LIVE | RUN_CODEX_E2E=1 + AZURE creds |
| IT-RUN-12 | run_task_updates_existing_config_block | run_task_module::run_task | DoWhiz_service/run_task_module/tests/run_task_basic.rs | Update config block | Multi-provider config | AUTO | cargo test -p run_task_module |

## Integration/E2E: scheduler_module
| ID | Test | Target (file::function/module) | Test File | Verifies | Does Not Verify | Status | Run/Env |
|---|---|---|---|---|---|---|---|
| IT-SCH-01 | cron_requires_six_fields | scheduler_module::Scheduler | DoWhiz_service/scheduler_module/tests/scheduler_basic.rs | Cron 6-field validation | Timezone/DST | AUTO | cargo test -p scheduler_module --test scheduler_basic |
| IT-SCH-02 | one_shot_persists_across_restarts | scheduler_module::Scheduler | DoWhiz_service/scheduler_module/tests/scheduler_basic.rs | sqlite persistence | Concurrency | AUTO | cargo test -p scheduler_module --test scheduler_basic |
| IT-SCH-03 | tick_disables_one_shot_tasks | scheduler_module::Scheduler | DoWhiz_service/scheduler_module/tests/scheduler_basic.rs | One-shot disable | Long task timeouts | AUTO | cargo test -p scheduler_module --test scheduler_basic |
| IT-SCH-04 | tick_sets_last_run_for_one_shot | scheduler_module::Scheduler | DoWhiz_service/scheduler_module/tests/scheduler_basic.rs | last_run set | Retry metadata | AUTO | cargo test -p scheduler_module --test scheduler_basic |
| IT-SCH-05 | run_loop_stops_when_flag_set | scheduler_module::Scheduler | DoWhiz_service/scheduler_module/tests/scheduler_basic.rs | stop flag | Thread leak | AUTO | cargo test -p scheduler_module --test scheduler_basic |
| IT-SCH-06 | scheduler_actions_end_to_end | run_task + scheduler actions | DoWhiz_service/scheduler_module/tests/scheduler_agent_e2e.rs | cancel/reschedule/create + follow-up | Real codex output | AUTO | cargo test -p scheduler_module --test scheduler_agent_e2e |
| IT-SCH-07 | inbound_email_html_is_sanitized | service/email.rs::process_inbound_payload | DoWhiz_service/scheduler_module/tests/email_html_e2e.rs | HTML sanitization | Complex HTML | AUTO | cargo test -p scheduler_module --test email_html_e2e |
| IT-SCH-08 | inbound_email_html_is_sanitized (dup) | service/email.rs::process_inbound_payload | DoWhiz_service/scheduler_module/tests/email_html_e2e_2.rs | Duplicate of IT-SCH-07 | No additional coverage | AUTO | cargo test -p scheduler_module --test email_html_e2e_2 |
| IT-SCH-09 | email_flow_injects_github_env | process_inbound_payload + run_task | DoWhiz_service/scheduler_module/tests/github_env_e2e.rs | GH env injection | Real gh auth | AUTO | cargo test -p scheduler_module --test github_env_e2e |
| IT-SCH-10 | email_flow_injects_employee_github_env | process_inbound_payload + run_task | DoWhiz_service/scheduler_module/tests/github_env_e2e.rs | Employee GH env | Multi-employee conflicts | AUTO | cargo test -p scheduler_module --test github_env_e2e |
| IT-SCH-11 | memory_sync_roundtrip_via_run_task | ModuleExecutor::execute | DoWhiz_service/scheduler_module/tests/memory_e2e.rs | Memory sync roundtrip | Large file sets | AUTO | cargo test -p scheduler_module --test memory_e2e |
| IT-SCH-12 | secrets_sync_roundtrip_via_run_task | ModuleExecutor::execute | DoWhiz_service/scheduler_module/tests/secrets_e2e.rs | Secrets sync roundtrip | Encryption/redaction | AUTO | cargo test -p scheduler_module --test secrets_e2e |
| IT-SCH-13 | secrets_persist_across_workspaces_and_load | process_inbound_payload + scheduler | DoWhiz_service/scheduler_module/tests/secrets_e2e.rs | Secrets persistence across workspaces | Large scale | AUTO | cargo test -p scheduler_module --test secrets_e2e |
| IT-SCH-14 | run_task_followups_persist_to_sqlite | Scheduler follow-ups | DoWhiz_service/scheduler_module/tests/scheduler_followups.rs | Follow-up persisted | Multiple follow-ups | AUTO | cargo test -p scheduler_module --test scheduler_followups |
| IT-SCH-15 | scheduler_parallelism_reduces_wall_clock_time | run_server + scheduler concurrency | DoWhiz_service/scheduler_module/tests/scheduler_concurrency.rs | Concurrency speedup | Real workload | AUTO | cargo test -p scheduler_module --test scheduler_concurrency |
| IT-SCH-16 | thread_latest_epoch_end_to_end | cancel_pending_thread_tasks | DoWhiz_service/scheduler_module/tests/thread_latest_epoch_e2e.rs | Cancel stale replies, latest wins | Non-email channels | AUTO | cargo test -p scheduler_module --test thread_latest_epoch_e2e |
| IT-SCH-17 | send_reply_slack_uses_mock | SendReplyTask outbound | DoWhiz_service/scheduler_module/tests/send_reply_outbound_e2e.rs | Slack outbound request | Real Slack API | AUTO | cargo test -p scheduler_module --test send_reply_outbound_e2e |
| IT-SCH-18 | send_reply_discord_uses_mock | SendReplyTask outbound | DoWhiz_service/scheduler_module/tests/send_reply_outbound_e2e.rs | Discord outbound request | Real Discord API | AUTO | cargo test -p scheduler_module --test send_reply_outbound_e2e |
| IT-SCH-19 | send_reply_sms_uses_mock | SendReplyTask outbound | DoWhiz_service/scheduler_module/tests/send_reply_outbound_e2e.rs | SMS outbound request | Real Twilio API | AUTO | cargo test -p scheduler_module --test send_reply_outbound_e2e |
| IT-SCH-20 | run_task_failure_retries_and_notifies | Retry + notifications | DoWhiz_service/scheduler_module/tests/scheduler_retry_notifications_e2e.rs | Retry + user/admin notice | Real mail delivery | AUTO | cargo test -p scheduler_module --test scheduler_retry_notifications_e2e |
| IT-SCH-21 | slack_failure_retries_and_notifies | Retry + Slack notice | DoWhiz_service/scheduler_module/tests/scheduler_retry_notifications_slack_e2e.rs | Retry + Slack notice | Real Slack API | AUTO | cargo test -p scheduler_module --test scheduler_retry_notifications_slack_e2e |
| LIVE-SCH-01 | google_docs_cli_e2e_list_documents | google-docs CLI | DoWhiz_service/scheduler_module/tests/google_docs_cli_e2e.rs | Real docs list | Rate limits | LIVE | GOOGLE_DOCS_CLI_E2E=1 + creds |
| LIVE-SCH-02 | google_docs_cli_e2e_read_document | google-docs CLI | DoWhiz_service/scheduler_module/tests/google_docs_cli_e2e.rs | Real doc read | Permission issues | LIVE | GOOGLE_DOCS_CLI_E2E=1 + doc id |
| LIVE-SCH-03 | google_docs_cli_e2e_list_comments | google-docs CLI | DoWhiz_service/scheduler_module/tests/google_docs_cli_e2e.rs | Real comments list | Pagination | LIVE | GOOGLE_DOCS_CLI_E2E=1 + doc id |
| LIVE-SCH-04 | google_docs_cli_e2e_mark_deletion | google-docs CLI | DoWhiz_service/scheduler_module/tests/google_docs_cli_e2e.rs | Mark deletion | Text anchor errors | LIVE | GOOGLE_DOCS_CLI_E2E=1 + doc id |
| LIVE-SCH-05 | google_docs_cli_e2e_insert_suggestion | google-docs CLI | DoWhiz_service/scheduler_module/tests/google_docs_cli_e2e.rs | Insert suggestion | Conflicts | LIVE | GOOGLE_DOCS_CLI_E2E=1 + doc id |
| LIVE-SCH-06 | google_docs_cli_e2e_suggest_replace | google-docs CLI | DoWhiz_service/scheduler_module/tests/google_docs_cli_e2e.rs | Suggest replace | Text mismatch | LIVE | GOOGLE_DOCS_CLI_E2E=1 + doc id |
| LIVE-SCH-07 | google_docs_cli_e2e_apply_suggestions | google-docs CLI | DoWhiz_service/scheduler_module/tests/google_docs_cli_e2e.rs | Apply suggestions | Conflicts | LIVE | GOOGLE_DOCS_CLI_E2E=1 + doc id |
| LIVE-SCH-08 | google_docs_cli_e2e_discard_suggestions | google-docs CLI | DoWhiz_service/scheduler_module/tests/google_docs_cli_e2e.rs | Discard suggestions | Conflicts | LIVE | GOOGLE_DOCS_CLI_E2E=1 + doc id |
| LIVE-SCH-09 | google_docs_cli_e2e_full_suggestion_workflow | google-docs CLI | DoWhiz_service/scheduler_module/tests/google_docs_cli_e2e.rs | Full suggestion workflow | Network failures | LIVE | GOOGLE_DOCS_CLI_E2E=1 + doc id |
| LIVE-SCH-10 | google_docs_cli_e2e_apply_edit | google-docs CLI | DoWhiz_service/scheduler_module/tests/google_docs_cli_e2e.rs | Apply edit | Text mismatch | LIVE | GOOGLE_DOCS_CLI_E2E=1 + doc id |
| LIVE-SCH-11 | rust_service_real_email_end_to_end | run_server + gateway + Postmark | DoWhiz_service/scheduler_module/tests/service_real_email.rs | Real inbound/outbound email flow | Cost and external variance | LIVE | RUST_SERVICE_LIVE_TEST=1 + Postmark/ngrok |

## Integration/E2E: send_emails_module
| ID | Test | Target (file::function/module) | Test File | Verifies | Does Not Verify | Status | Run/Env |
|---|---|---|---|---|---|---|---|
| IT-EMAIL-01 | send_payload_includes_recipients_and_attachments | send_emails_module::send_email | DoWhiz_service/send_emails_module/tests/send_emails_integration.rs | Payload and attachments encoding | Real Postmark behavior | AUTO | cargo test -p send_emails_module |
| IT-EMAIL-02 | live_postmark_delivery_with_attachments | send_emails_module::send_email | DoWhiz_service/send_emails_module/tests/send_emails_integration.rs | Real delivery with attachments | External latency/cost | LIVE | POSTMARK_LIVE_TEST=1 + creds |
| IT-EMAIL-03 | send_email_requires_recipient | send_emails_module::send_email | DoWhiz_service/send_emails_module/tests/send_emails_integration.rs | Missing recipient error | Multi-recipient edge cases | AUTO | cargo test -p send_emails_module |
| IT-EMAIL-04 | send_email_requires_token | send_emails_module::send_email | DoWhiz_service/send_emails_module/tests/send_emails_integration.rs | Missing token error | Token expiry | AUTO | cargo test -p send_emails_module |
| LIVE-EMAIL-01 | send_email_with_attachments_and_delivery | send_emails_module::send_email | DoWhiz_service/send_emails_module/tests/live_postmark.rs | Real delivery and attachments | External latency/cost | LIVE | POSTMARK_LIVE_TEST=1 + creds |
| LIVE-EMAIL-02 | send_multiple_emails_batch | send_emails_module::send_email | DoWhiz_service/send_emails_module/tests/live_postmark.rs | Batch send | External latency/cost | LIVE | POSTMARK_LIVE_TEST=1 + creds |

## Gaps and Planned Tests (Manual or To Be Implemented)
| ID | Priority | Gap | Target (file::function/module) | What Is Missing | Status | Manual Validation |
|---|---|---|---|---|---|---|
| GAP-01 | P0 | Docker runner E2E | run_task_module::run_task (docker path) | RUN_TASK_USE_DOCKER path not covered | PLANNED | Run dockerized run_task and verify outputs |
| GAP-02 | P0 | Watchdog recovery/stale tasks | service/scheduler.rs::watchdog logic | No stale task recovery test | PLANNED | Force stuck task, observe recovery |
| GAP-03 | P0 | Outbound failure retry for Slack/Discord/SMS | outbound adapters + retry logic | Only success mocks covered | PLANNED | Return 5xx from mock, check retries |
| GAP-04 | P0 | Non-email channels thread cancel/latest-epoch | cancel_pending_thread_tasks for non-email | Only email thread scenario covered | PLANNED | Manual multi-message per channel |
| GAP-05 | P0 | Ingestion queue concurrency claim | ingestion_queue::claim_next | No multi-worker race test | PLANNED | Parallel claims with threads |
| GAP-06 | P1 | HTML sanitizer complex cases | service/email.rs::render_email_html | Only simple HTML cases | MANUAL | Feed complex HTML samples |
| GAP-07 | P1 | Large attachment and size cap behavior | past_emails::hydrate_past_emails | No max size behavior test | MANUAL | Create large attachment samples |
| GAP-08 | P1 | Router HTTP failure handling | message_router::classify | No network failure test | MANUAL | Simulate OpenAI/Ollama down |
| GAP-09 | P1 | SlackStore env fallback | slack_store::get_installation_or_env | Not tested | PLANNED | Set env + call fallback path |
| GAP-10 | P2 | Cron timezone/DST edge behavior | scheduler cron parsing | No DST tests | MANUAL | Run around DST boundary |
| GAP-11 | P2 | Postmark inbound payload edge cases | service/email.rs::process_inbound_payload | Limited payload variations | MANUAL | Create malformed/partial payloads |

## Test Report Template
| Test ID | Status (PASS/FAIL/SKIP) | Evidence (log/summary) | Notes/Reason |
|---|---|---|---|
| UT-... |  |  |  |

Rules:
- All relevant AUTO tests must be run after changes.
- LIVE tests must be marked SKIP with reason unless explicitly run.
- MANUAL/PLANNED must be marked SKIP with reason and follow-up if needed.

## Live E2E Defaults (Ngrok)
- If a real end-to-end test needs a public ngrok URL, use: https://shayne-laminar-lillian.ngrok-free.dev

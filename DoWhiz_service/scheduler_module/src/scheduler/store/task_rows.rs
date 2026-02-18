use rusqlite::{params, Connection, OptionalExtension, Transaction};
use std::path::PathBuf;

use crate::channel::Channel;

use super::super::types::{RunTaskTask, SchedulerError, SendReplyTask};
use super::super::utils::{
    insert_recipients, normalize_header_value, normalize_optional_path, split_recipients,
};
use super::SqliteSchedulerStore;

impl SqliteSchedulerStore {
    pub(super) fn insert_send_email_task(
        &self,
        tx: &Transaction,
        task_id: &str,
        send: &SendReplyTask,
    ) -> Result<(), SchedulerError> {
        tx.execute(
            "INSERT INTO send_email_tasks (task_id, subject, html_path, attachments_dir, from_address, in_reply_to, references_header, archive_root, thread_epoch, thread_state_path)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                task_id,
                send.subject.as_str(),
                send.html_path.to_string_lossy().into_owned(),
                send.attachments_dir.to_string_lossy().into_owned(),
                send.from.as_deref(),
                send.in_reply_to.as_deref(),
                send.references.as_deref(),
                send.archive_root
                    .as_ref()
                    .map(|value| value.to_string_lossy().into_owned()),
                send.thread_epoch.map(|value| value as i64),
                send.thread_state_path
                    .as_ref()
                    .map(|value| value.to_string_lossy().into_owned()),
            ],
        )?;
        insert_recipients(tx, task_id, "to", &send.to)?;
        insert_recipients(tx, task_id, "cc", &send.cc)?;
        insert_recipients(tx, task_id, "bcc", &send.bcc)?;
        Ok(())
    }

    pub(super) fn insert_send_slack_task(
        &self,
        tx: &Transaction,
        task_id: &str,
        send: &SendReplyTask,
    ) -> Result<(), SchedulerError> {
        // For Slack, we use to[0] as channel_id and html_path as text_path
        let slack_channel_id = send.to.first().cloned().unwrap_or_default();
        let thread_ts = send.in_reply_to.clone();
        let workspace_dir = send
            .archive_root
            .as_ref()
            .map(|p| p.to_string_lossy().into_owned());
        tx.execute(
            "INSERT INTO send_slack_tasks (task_id, slack_channel_id, thread_ts, text_path, workspace_dir)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                task_id,
                slack_channel_id,
                thread_ts,
                send.html_path.to_string_lossy().into_owned(),
                workspace_dir,
            ],
        )?;
        Ok(())
    }

    pub(super) fn insert_send_discord_task(
        &self,
        tx: &Transaction,
        task_id: &str,
        send: &SendReplyTask,
    ) -> Result<(), SchedulerError> {
        // For Discord, we use to[0] as channel_id and html_path as text_path
        let discord_channel_id = send.to.first().cloned().unwrap_or_default();
        let thread_id = send.in_reply_to.clone();
        let workspace_dir = send
            .archive_root
            .as_ref()
            .map(|p| p.to_string_lossy().into_owned());
        tx.execute(
            "INSERT INTO send_discord_tasks (task_id, discord_channel_id, thread_id, text_path, workspace_dir)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                task_id,
                discord_channel_id,
                thread_id,
                send.html_path.to_string_lossy().into_owned(),
                workspace_dir,
            ],
        )?;
        Ok(())
    }

    pub(super) fn insert_send_sms_task(
        &self,
        tx: &Transaction,
        task_id: &str,
        send: &SendReplyTask,
    ) -> Result<(), SchedulerError> {
        let to_number = send.to.first().cloned().unwrap_or_default();
        tx.execute(
            "INSERT INTO send_sms_tasks (task_id, from_number, to_number, text_path, thread_id, thread_epoch, thread_state_path)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                task_id,
                send.from.as_deref(),
                to_number,
                send.html_path.to_string_lossy().into_owned(),
                send.in_reply_to.as_deref(),
                send.thread_epoch.map(|value| value as i64),
                send.thread_state_path
                    .as_ref()
                    .map(|value| value.to_string_lossy().into_owned()),
            ],
        )?;
        Ok(())
    }

    pub(super) fn insert_send_bluebubbles_task(
        &self,
        tx: &Transaction,
        task_id: &str,
        send: &SendReplyTask,
    ) -> Result<(), SchedulerError> {
        // For BlueBubbles, we use to[0] as chat_guid and html_path as text_path
        let chat_guid = send.to.first().cloned().unwrap_or_default();
        tx.execute(
            "INSERT INTO send_bluebubbles_tasks (task_id, chat_guid, text_path, thread_epoch, thread_state_path)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                task_id,
                chat_guid,
                send.html_path.to_string_lossy().into_owned(),
                send.thread_epoch.map(|value| value as i64),
                send.thread_state_path
                    .as_ref()
                    .map(|value| value.to_string_lossy().into_owned()),
            ],
        )?;
        Ok(())
    }

    pub(super) fn insert_send_telegram_task(
        &self,
        tx: &Transaction,
        task_id: &str,
        send: &SendReplyTask,
    ) -> Result<(), SchedulerError> {
        // For Telegram, we use to[0] as chat_id and html_path as text_path
        let chat_id = send.to.first().cloned().unwrap_or_default();
        tx.execute(
            "INSERT INTO send_telegram_tasks (task_id, chat_id, text_path, thread_epoch, thread_state_path)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                task_id,
                chat_id,
                send.html_path.to_string_lossy().into_owned(),
                send.thread_epoch.map(|value| value as i64),
                send.thread_state_path
                    .as_ref()
                    .map(|value| value.to_string_lossy().into_owned()),
            ],
        )?;
        Ok(())
    }

    pub(super) fn load_send_email_task(
        &self,
        conn: &Connection,
        task_id: &str,
        channel: Channel,
    ) -> Result<SendReplyTask, SchedulerError> {
        let row = conn
            .query_row(
                "SELECT subject, html_path, attachments_dir, from_address, in_reply_to, references_header, archive_root, thread_epoch, thread_state_path
                 FROM send_email_tasks
                 WHERE task_id = ?1",
                params![task_id],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, String>(2)?,
                        row.get::<_, Option<String>>(3)?,
                        row.get::<_, Option<String>>(4)?,
                        row.get::<_, Option<String>>(5)?,
                        row.get::<_, Option<String>>(6)?,
                        row.get::<_, Option<i64>>(7)?,
                        row.get::<_, Option<String>>(8)?,
                    ))
                },
            )
            .optional()?;
        let (
            subject,
            html_path,
            attachments_dir,
            from_raw,
            in_reply_to_raw,
            references_raw,
            archive_root,
            thread_epoch_raw,
            thread_state_path,
        ) = row.ok_or_else(|| {
            SchedulerError::Storage(format!("missing send_email_tasks row for task {}", task_id))
        })?;

        let mut to = Vec::new();
        let mut cc = Vec::new();
        let mut bcc = Vec::new();
        let mut stmt = conn.prepare(
            "SELECT recipient_type, address
             FROM send_email_recipients
             WHERE task_id = ?1
             ORDER BY id",
        )?;
        let rows = stmt.query_map(params![task_id], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?;
        for row in rows {
            let (recipient_type, address) = row?;
            match recipient_type.as_str() {
                "to" => to.push(address),
                "cc" => cc.push(address),
                "bcc" => bcc.push(address),
                _ => {}
            }
        }

        Ok(SendReplyTask {
            channel,
            subject,
            html_path: PathBuf::from(html_path),
            attachments_dir: PathBuf::from(attachments_dir),
            from: normalize_header_value(from_raw),
            to,
            cc,
            bcc,
            in_reply_to: normalize_header_value(in_reply_to_raw),
            references: normalize_header_value(references_raw),
            archive_root: normalize_optional_path(archive_root),
            thread_epoch: thread_epoch_raw.map(|value| value as u64),
            thread_state_path: normalize_optional_path(thread_state_path),
        })
    }

    pub(super) fn load_send_slack_task(
        &self,
        conn: &Connection,
        task_id: &str,
        channel: Channel,
    ) -> Result<SendReplyTask, SchedulerError> {
        let row = conn
            .query_row(
                "SELECT slack_channel_id, thread_ts, text_path, workspace_dir
                 FROM send_slack_tasks
                 WHERE task_id = ?1",
                params![task_id],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, Option<String>>(1)?,
                        row.get::<_, String>(2)?,
                        row.get::<_, Option<String>>(3)?,
                    ))
                },
            )
            .optional()?;
        let (slack_channel_id, thread_ts, text_path, workspace_dir) = row.ok_or_else(|| {
            SchedulerError::Storage(format!("missing send_slack_tasks row for task {}", task_id))
        })?;

        Ok(SendReplyTask {
            channel,
            subject: String::new(), // Slack doesn't use subject
            html_path: PathBuf::from(text_path),
            attachments_dir: PathBuf::new(), // Slack attachments handled differently
            from: None,
            to: vec![slack_channel_id], // channel_id stored in to[0]
            cc: Vec::new(),
            bcc: Vec::new(),
            in_reply_to: thread_ts, // thread_ts stored in in_reply_to
            references: None,
            archive_root: workspace_dir.map(PathBuf::from),
            thread_epoch: None,
            thread_state_path: None,
        })
    }

    pub(super) fn load_send_discord_task(
        &self,
        conn: &Connection,
        task_id: &str,
    ) -> Result<SendReplyTask, SchedulerError> {
        let row = conn
            .query_row(
                "SELECT discord_channel_id, thread_id, text_path, workspace_dir
                 FROM send_discord_tasks
                 WHERE task_id = ?1",
                params![task_id],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, Option<String>>(1)?,
                        row.get::<_, String>(2)?,
                        row.get::<_, Option<String>>(3)?,
                    ))
                },
            )
            .optional()?;
        let (discord_channel_id, thread_id, text_path, workspace_dir) = row.ok_or_else(|| {
            SchedulerError::Storage(format!(
                "missing send_discord_tasks row for task {}",
                task_id
            ))
        })?;

        Ok(SendReplyTask {
            channel: Channel::Discord,
            subject: String::new(), // Discord doesn't use subject
            html_path: PathBuf::from(text_path),
            attachments_dir: PathBuf::new(), // Discord attachments handled differently
            from: None,
            to: vec![discord_channel_id], // channel_id stored in to[0]
            cc: Vec::new(),
            bcc: Vec::new(),
            in_reply_to: thread_id, // thread_id stored in in_reply_to
            references: None,
            archive_root: workspace_dir.map(PathBuf::from),
            thread_epoch: None,
            thread_state_path: None,
        })
    }

    pub(super) fn load_send_sms_task(
        &self,
        conn: &Connection,
        task_id: &str,
    ) -> Result<SendReplyTask, SchedulerError> {
        let row = conn
            .query_row(
                "SELECT from_number, to_number, text_path, thread_id, thread_epoch, thread_state_path
                 FROM send_sms_tasks
                 WHERE task_id = ?1",
                params![task_id],
                |row| {
                    Ok((
                        row.get::<_, Option<String>>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, String>(2)?,
                        row.get::<_, Option<String>>(3)?,
                        row.get::<_, Option<i64>>(4)?,
                        row.get::<_, Option<String>>(5)?,
                    ))
                },
            )
            .optional()?;
        let (from_number, to_number, text_path, thread_id, thread_epoch_raw, thread_state_path) =
            row.ok_or_else(|| {
                SchedulerError::Storage(format!("missing send_sms_tasks row for task {}", task_id))
            })?;

        Ok(SendReplyTask {
            channel: Channel::Sms,
            subject: String::new(),
            html_path: PathBuf::from(text_path),
            attachments_dir: PathBuf::new(),
            from: normalize_header_value(from_number),
            to: vec![to_number],
            cc: Vec::new(),
            bcc: Vec::new(),
            in_reply_to: normalize_header_value(thread_id),
            references: None,
            archive_root: None,
            thread_epoch: thread_epoch_raw.map(|value| value as u64),
            thread_state_path: normalize_optional_path(thread_state_path),
        })
    }

    pub(super) fn load_send_bluebubbles_task(
        &self,
        conn: &Connection,
        task_id: &str,
    ) -> Result<SendReplyTask, SchedulerError> {
        let row = conn
            .query_row(
                "SELECT chat_guid, text_path, thread_epoch, thread_state_path
                 FROM send_bluebubbles_tasks
                 WHERE task_id = ?1",
                params![task_id],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, Option<i64>>(2)?,
                        row.get::<_, Option<String>>(3)?,
                    ))
                },
            )
            .optional()?;
        let (chat_guid, text_path, thread_epoch_raw, thread_state_path) = row.ok_or_else(|| {
            SchedulerError::Storage(format!(
                "missing send_bluebubbles_tasks row for task {}",
                task_id
            ))
        })?;

        Ok(SendReplyTask {
            channel: Channel::BlueBubbles,
            subject: String::new(), // BlueBubbles doesn't use subject
            html_path: PathBuf::from(text_path),
            attachments_dir: PathBuf::new(), // BlueBubbles attachments handled differently
            from: None,
            to: vec![chat_guid], // chat_guid stored in to[0]
            cc: Vec::new(),
            bcc: Vec::new(),
            in_reply_to: None,
            references: None,
            archive_root: None,
            thread_epoch: thread_epoch_raw.map(|value| value as u64),
            thread_state_path: normalize_optional_path(thread_state_path),
        })
    }

    pub(super) fn load_send_telegram_task(
        &self,
        conn: &Connection,
        task_id: &str,
    ) -> Result<SendReplyTask, SchedulerError> {
        let row = conn
            .query_row(
                "SELECT chat_id, text_path, thread_epoch, thread_state_path
                 FROM send_telegram_tasks
                 WHERE task_id = ?1",
                params![task_id],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, Option<i64>>(2)?,
                        row.get::<_, Option<String>>(3)?,
                    ))
                },
            )
            .optional()?;
        let (chat_id, text_path, thread_epoch_raw, thread_state_path) = row.ok_or_else(|| {
            SchedulerError::Storage(format!(
                "missing send_telegram_tasks row for task {}",
                task_id
            ))
        })?;

        Ok(SendReplyTask {
            channel: Channel::Telegram,
            subject: String::new(), // Telegram doesn't use subject
            html_path: PathBuf::from(text_path),
            attachments_dir: PathBuf::new(), // Telegram attachments handled differently
            from: None,
            to: vec![chat_id], // chat_id stored in to[0]
            cc: Vec::new(),
            bcc: Vec::new(),
            in_reply_to: None,
            references: None,
            archive_root: None,
            thread_epoch: thread_epoch_raw.map(|value| value as u64),
            thread_state_path: normalize_optional_path(thread_state_path),
        })
    }

    pub(super) fn insert_send_whatsapp_task(
        &self,
        tx: &Transaction,
        task_id: &str,
        send: &SendReplyTask,
    ) -> Result<(), SchedulerError> {
        // For WhatsApp, we use to[0] as phone_number and html_path as text_path
        let phone_number = send.to.first().cloned().unwrap_or_default();
        tx.execute(
            "INSERT INTO send_whatsapp_tasks (task_id, phone_number, text_path, thread_epoch, thread_state_path)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                task_id,
                phone_number,
                send.html_path.to_string_lossy().into_owned(),
                send.thread_epoch.map(|value| value as i64),
                send.thread_state_path
                    .as_ref()
                    .map(|value| value.to_string_lossy().into_owned()),
            ],
        )?;
        Ok(())
    }

    pub(super) fn load_send_whatsapp_task(
        &self,
        conn: &Connection,
        task_id: &str,
    ) -> Result<SendReplyTask, SchedulerError> {
        let row = conn
            .query_row(
                "SELECT phone_number, text_path, thread_epoch, thread_state_path
                 FROM send_whatsapp_tasks
                 WHERE task_id = ?1",
                params![task_id],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, Option<i64>>(2)?,
                        row.get::<_, Option<String>>(3)?,
                    ))
                },
            )
            .optional()?;
        let (phone_number, text_path, thread_epoch_raw, thread_state_path) =
            row.ok_or_else(|| {
                SchedulerError::Storage(format!(
                    "missing send_whatsapp_tasks row for task {}",
                    task_id
                ))
            })?;

        Ok(SendReplyTask {
            channel: Channel::WhatsApp,
            subject: String::new(), // WhatsApp doesn't use subject
            html_path: PathBuf::from(text_path),
            attachments_dir: PathBuf::new(), // WhatsApp attachments handled differently
            from: None,
            to: vec![phone_number], // phone_number stored in to[0]
            cc: Vec::new(),
            bcc: Vec::new(),
            in_reply_to: None,
            references: None,
            archive_root: None,
            thread_epoch: thread_epoch_raw.map(|value| value as u64),
            thread_state_path: normalize_optional_path(thread_state_path),
        })
    }

    pub(super) fn load_run_task_task(
        &self,
        conn: &Connection,
        task_id: &str,
        channel: Channel,
    ) -> Result<RunTaskTask, SchedulerError> {
        let row = conn
            .query_row(
                "SELECT workspace_dir, input_email_dir, input_attachments_dir, memory_dir, reference_dir, model_name, runner, codex_disabled, reply_to, reply_from, archive_root, thread_id, thread_epoch, thread_state_path
                 FROM run_task_tasks
                 WHERE task_id = ?1",
                params![task_id],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, String>(2)?,
                        row.get::<_, String>(3)?,
                        row.get::<_, String>(4)?,
                        row.get::<_, String>(5)?,
                        row.get::<_, Option<String>>(6)?,
                        row.get::<_, i64>(7)?,
                        row.get::<_, String>(8)?,
                        row.get::<_, Option<String>>(9)?,
                        row.get::<_, Option<String>>(10)?,
                        row.get::<_, Option<String>>(11)?,
                        row.get::<_, Option<i64>>(12)?,
                        row.get::<_, Option<String>>(13)?,
                    ))
                },
            )
            .optional()?;
        let (
            workspace_dir,
            input_email_dir,
            input_attachments_dir,
            memory_dir,
            reference_dir,
            model_name,
            runner,
            codex_disabled,
            reply_to_raw,
            reply_from,
            archive_root,
            thread_id,
            thread_epoch_raw,
            thread_state_path,
        ) = row.ok_or_else(|| {
            SchedulerError::Storage(format!("missing run_task_tasks row for task {}", task_id))
        })?;

        Ok(RunTaskTask {
            workspace_dir: PathBuf::from(workspace_dir),
            input_email_dir: PathBuf::from(input_email_dir),
            input_attachments_dir: PathBuf::from(input_attachments_dir),
            memory_dir: PathBuf::from(memory_dir),
            reference_dir: PathBuf::from(reference_dir),
            model_name,
            runner: runner
                .as_deref()
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(|value| value.to_string())
                .unwrap_or_else(|| "codex".to_string()),
            codex_disabled: codex_disabled != 0,
            reply_to: split_recipients(&reply_to_raw),
            reply_from: normalize_header_value(reply_from),
            archive_root: normalize_optional_path(archive_root),
            thread_id,
            thread_epoch: thread_epoch_raw.map(|value| value as u64),
            thread_state_path: normalize_optional_path(thread_state_path),
            channel,
            slack_team_id: None,
            employee_id: None,
        })
    }
}

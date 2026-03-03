use serde::Deserialize;

use super::constants::{
    SCHEDULED_TASKS_BEGIN, SCHEDULED_TASKS_END, SCHEDULER_ACTIONS_BEGIN, SCHEDULER_ACTIONS_END,
};
use super::types::{ScheduledTaskRequest, SchedulerActionRequest};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ScheduledTasksBlock {
    List(Vec<ScheduledTaskRequest>),
    Wrapper { tasks: Vec<ScheduledTaskRequest> },
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum SchedulerActionsBlock {
    List(Vec<SchedulerActionRequest>),
    Wrapper {
        actions: Vec<SchedulerActionRequest>,
    },
}

pub(super) fn extract_scheduled_tasks(output: &str) -> (Vec<ScheduledTaskRequest>, Option<String>) {
    let segments = extract_json_segments(output, SCHEDULED_TASKS_BEGIN, SCHEDULED_TASKS_END);
    if segments.is_empty() {
        return (Vec::new(), None);
    }

    let mut last_error: Option<String> = None;
    for raw_json in segments.into_iter().rev() {
        if raw_json.is_empty() {
            continue;
        }
        match serde_json::from_str::<ScheduledTasksBlock>(&raw_json) {
            Ok(block) => {
                let tasks = match block {
                    ScheduledTasksBlock::List(tasks) => tasks,
                    ScheduledTasksBlock::Wrapper { tasks } => tasks,
                };
                return (tasks, None);
            }
            Err(err) => {
                last_error = Some(err.to_string());
            }
        }
    }
    (
        Vec::new(),
        last_error.map(|err| format!("failed to parse scheduled tasks JSON: {}", err)),
    )
}

pub(super) fn extract_scheduler_actions(
    output: &str,
) -> (Vec<SchedulerActionRequest>, Option<String>) {
    let segments = extract_json_segments(output, SCHEDULER_ACTIONS_BEGIN, SCHEDULER_ACTIONS_END);
    if segments.is_empty() {
        return (Vec::new(), None);
    }

    let mut last_error: Option<String> = None;
    for raw_json in segments.into_iter().rev() {
        if raw_json.is_empty() {
            continue;
        }
        match serde_json::from_str::<SchedulerActionsBlock>(&raw_json) {
            Ok(block) => {
                let actions = match block {
                    SchedulerActionsBlock::List(actions) => actions,
                    SchedulerActionsBlock::Wrapper { actions } => actions,
                };
                return (actions, None);
            }
            Err(err) => {
                last_error = Some(err.to_string());
            }
        }
    }
    (
        Vec::new(),
        last_error.map(|err| format!("failed to parse scheduler actions JSON: {}", err)),
    )
}

fn extract_json_segments(output: &str, begin: &str, end: &str) -> Vec<String> {
    let mut segments = Vec::new();
    let mut scan_from = 0usize;
    while let Some(begin_rel) = output[scan_from..].find(begin) {
        let start = scan_from + begin_rel + begin.len();
        let Some(end_rel) = output[start..].find(end) else {
            break;
        };
        let stop = start + end_rel;
        segments.push(output[start..stop].trim().to_string());
        scan_from = stop + end.len();
    }
    segments
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_scheduler_actions_returns_empty_when_missing() {
        let output = "no scheduler actions here";
        let (actions, error) = extract_scheduler_actions(output);
        assert!(actions.is_empty());
        assert!(error.is_none());
    }

    #[test]
    fn extract_scheduler_actions_parses_list() {
        let output = format!(
            "before\n{}\n[{{\"action\":\"cancel\",\"task_ids\":[\"a\",\"b\"]}}]\n{}\nafter",
            SCHEDULER_ACTIONS_BEGIN, SCHEDULER_ACTIONS_END
        );
        let (actions, error) = extract_scheduler_actions(&output);
        assert!(error.is_none());
        assert_eq!(actions.len(), 1);
        match &actions[0] {
            SchedulerActionRequest::Cancel { task_ids } => {
                assert_eq!(task_ids, &vec!["a".to_string(), "b".to_string()]);
            }
            other => panic!("unexpected action: {:?}", other),
        }
    }

    #[test]
    fn extract_scheduler_actions_reports_invalid_json() {
        let output = format!(
            "{}\n[{{\"action\":\"cancel\",\"task_ids\"::}}]\n{}",
            SCHEDULER_ACTIONS_BEGIN, SCHEDULER_ACTIONS_END
        );
        let (actions, error) = extract_scheduler_actions(&output);
        assert!(actions.is_empty());
        assert!(error.is_some());
    }

    #[test]
    fn extract_scheduled_tasks_prefers_latest_valid_block() {
        let output = format!(
            "before\n{}\n<JSON array here>\n{}\nmid\n{}\n[{{\"type\":\"send_email\",\"delay_seconds\":60,\"subject\":\"x\",\"html_path\":\"x.html\"}}]\n{}\nafter",
            SCHEDULED_TASKS_BEGIN,
            SCHEDULED_TASKS_END,
            SCHEDULED_TASKS_BEGIN,
            SCHEDULED_TASKS_END
        );
        let (tasks, error) = extract_scheduled_tasks(&output);
        assert!(error.is_none());
        assert_eq!(tasks.len(), 1);
    }

    #[test]
    fn extract_scheduler_actions_prefers_latest_valid_block() {
        let output = format!(
            "before\n{}\n<JSON array here>\n{}\nmid\n{}\n[{{\"action\":\"cancel\",\"task_ids\":[\"abc\"]}}]\n{}\nafter",
            SCHEDULER_ACTIONS_BEGIN,
            SCHEDULER_ACTIONS_END,
            SCHEDULER_ACTIONS_BEGIN,
            SCHEDULER_ACTIONS_END
        );
        let (actions, error) = extract_scheduler_actions(&output);
        assert!(error.is_none());
        assert_eq!(actions.len(), 1);
    }
}

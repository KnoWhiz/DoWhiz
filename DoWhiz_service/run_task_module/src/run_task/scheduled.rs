use serde::Deserialize;

use super::constants::{SCHEDULED_TASKS_BEGIN, SCHEDULED_TASKS_END, SCHEDULER_ACTIONS_BEGIN, SCHEDULER_ACTIONS_END};
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
    Wrapper { actions: Vec<SchedulerActionRequest> },
}

pub(super) fn extract_scheduled_tasks(output: &str) -> (Vec<ScheduledTaskRequest>, Option<String>) {
    if !output.contains(SCHEDULED_TASKS_BEGIN) {
        return (Vec::new(), None);
    }

    let start = output
        .find(SCHEDULED_TASKS_BEGIN)
        .map(|idx| idx + SCHEDULED_TASKS_BEGIN.len())
        .unwrap_or(output.len());
    let end = output
        .rfind(SCHEDULED_TASKS_END)
        .unwrap_or(output.len());

    let raw_json = output[start..end].trim();
    if raw_json.is_empty() {
        return (Vec::new(), None);
    }

    match serde_json::from_str::<ScheduledTasksBlock>(raw_json) {
        Ok(block) => {
            let tasks = match block {
                ScheduledTasksBlock::List(tasks) => tasks,
                ScheduledTasksBlock::Wrapper { tasks } => tasks,
            };
            (tasks, None)
        }
        Err(err) => (
            Vec::new(),
            Some(format!("failed to parse scheduled tasks JSON: {}", err)),
        ),
    }
}

pub(super) fn extract_scheduler_actions(
    output: &str,
) -> (Vec<SchedulerActionRequest>, Option<String>) {
    if !output.contains(SCHEDULER_ACTIONS_BEGIN) {
        return (Vec::new(), None);
    }

    let start = output
        .find(SCHEDULER_ACTIONS_BEGIN)
        .map(|idx| idx + SCHEDULER_ACTIONS_BEGIN.len())
        .unwrap_or(output.len());
    let end = output
        .rfind(SCHEDULER_ACTIONS_END)
        .unwrap_or(output.len());

    let raw_json = output[start..end].trim();
    if raw_json.is_empty() {
        return (Vec::new(), None);
    }

    match serde_json::from_str::<SchedulerActionsBlock>(raw_json) {
        Ok(block) => {
            let actions = match block {
                SchedulerActionsBlock::List(actions) => actions,
                SchedulerActionsBlock::Wrapper { actions } => actions,
            };
            (actions, None)
        }
        Err(err) => (
            Vec::new(),
            Some(format!("failed to parse scheduler actions JSON: {}", err)),
        ),
    }
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
}

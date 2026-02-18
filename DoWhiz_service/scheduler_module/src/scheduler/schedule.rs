use chrono::{DateTime, Utc};
use cron::Schedule as CronSchedule;
use std::str::FromStr;

use super::types::SchedulerError;

pub(crate) fn validate_cron_expression(expression: &str) -> Result<(), SchedulerError> {
    let fields = expression.split_whitespace().count();
    if fields != 6 {
        return Err(SchedulerError::InvalidCron(fields));
    }
    Ok(())
}

pub(crate) fn next_run_after(
    expression: &str,
    after: DateTime<Utc>,
) -> Result<DateTime<Utc>, SchedulerError> {
    validate_cron_expression(expression)?;
    let schedule = CronSchedule::from_str(expression)?;
    for datetime in schedule.upcoming(Utc) {
        if datetime > after {
            return Ok(datetime);
        }
    }
    Err(SchedulerError::NoNextRun)
}

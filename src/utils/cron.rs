use cron::Schedule;
use std::str::FromStr;

pub fn validate_cron(expression: &str) -> bool {
    Schedule::from_str(expression).is_ok()
}

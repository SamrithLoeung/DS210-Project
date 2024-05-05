use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct ParticipantData {
    pub screen_time_hours: f64,
    pub physical_activity_hours: HashMap<String, f64>,
}

impl ParticipantData {
    pub fn new() -> ParticipantData {
        ParticipantData {
            screen_time_hours: 0.0,
            physical_activity_hours: HashMap::new(),
        }
    }

    pub fn add_screen_time(&mut self, hours: f64) {
        self.screen_time_hours += hours;
        debug!("Added screen time: {} hours. Total now: {} hours", hours, self.screen_time_hours);
    }

    pub fn add_physical_activity(&mut self, category: String, hours: f64) {
        *self.physical_activity_hours.entry(category).or_insert(0.0) += hours;
        debug!("Added {} hours to {}. Total now: {} hours", hours, category, self.physical_activity_hours[&category]);
    }
}
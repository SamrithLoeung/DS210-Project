use crate::participant::ParticipantData;
use std::collections::HashMap;
use log::{info, warn};

pub fn compare_activities(data: &HashMap<String, ParticipantData>) {
    info!("Comparing screen time and physical activity for each participant.");
    for (id, pdata) in data.iter() {
        let total_physical_activity = pdata.physical_activity_hours.values().sum::<f64>();
        let screen_time = pdata.screen_time_hours;

        // Compute the difference
        let difference = total_physical_activity - screen_time;

        // Log or print the comparison
        info!("Participant ID: {}, Total Physical Activity: {:.2} hours, Screen Time: {:.2} hours, Difference: {:.2} hours",
              id, total_physical_activity, screen_time, difference);

        // Additional logic based on the comparison
        if difference > 0.0 {
            info!("Participant ID: {} spends more time on physical activity than on screen time.", id);
        } else if difference < 0.0 {
            warn!("Participant ID: {} spends more time on screen time than on physical activity.", id);
        } else {
            info!("Participant ID: {} spends equal time on physical activity and screen time.", id);
        }
    }
}
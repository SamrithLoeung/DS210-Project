use super::participant::ParticipantData;
use calamine::{open_workbook, Reader, Xlsx, RangeDeserializerBuilder};
use std::collections::HashMap;

pub fn read_physical_activity_data(file_path: &str, data: &mut HashMap<String, ParticipantData>) -> calamine::Result<()> {
    let mut workbook: Xlsx<_] = open_workbook(file_path)?;
    if let Some(Ok(range)) = workbook.worksheet_range("Sheet1") {
        let mut iter = RangeDeserializerBuilder::new().from_range(&range)?;
        for row in iter {
            let (participant_id, activity_category, hours): (String, String, f64) = row?;
            let pdata = data.entry(participant_id).or_insert_with(ParticipantData::new);
            pdata.add_physical_activity(activity_category, hours);
        }
    }
    Ok(())
}
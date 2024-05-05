extern crate log;
extern crate env_logger;
extern crate plotters;

mod participant;
mod screen_time;
mod physical_activity;
mod activity_comparison;

use participant::ParticipantData;
use plotters::prelude::*;
use std::collections::HashMap;
use std::time::Instant;

fn main() -> calamine::Result<()> {
    env_logger::init();
    let mut data = HashMap::new();

    // Timing start
    let start = Instant::now();

    info!("Starting to read screen time data...");
    if let Err(e) = screen_time::read_screen_time_data("24 hour recalldata.xlsx", &mut data) {
        error!("Failed to read screen time data: {}", e);
        return Err(e);
    }
    info!("Screen time data successfully read.");

    info!("Starting to read physical activity data...");
    if let Err(e) = physical_activity::read_physical_activity_data("actigraph activity data.xlsx", &mut data) {
        error!("Failed to read physical activity data: {}", e);
        return Err(e);
    }
    info!("Physical activity data successfully read.");

    info!("Data processing completed in {} seconds.", start.elapsed().as_secs());

    activity_comparison::compare_activities(&data);
    
    // Start graph generation
    info!("Generating graph...");
    if let Err(e) = generate_graph(&data) {
        error!("Failed to generate graph: {}", e);
        return Err(e);
    }
    info!("Graph generated successfully and saved as output.png");

    Ok(())
}

fn generate_graph(data: &HashMap<String, ParticipantData>) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("output.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let max_hours = data.values()
        .map(|d| d.screen_time_hours.max(d.physical_activity_hours.values().sum::<f64>()))
        .fold(0.0 / 0.0, f64::max);

    let mut chart = ChartBuilder::on(&root)
        .caption("Participant Activity Data", ("sans-serif", 20).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(40)
        .build_cartesian_2d(0..data.len() * 2, 0.0..max_hours)?;

    chart.configure_mesh().x_labels(data.len()).draw()?;

    let bar_width = 0.8;
    let mut index = 0;
    for (participant_id, pdata) in data.iter() {
        let total_physical_activity = pdata.physical_activity_hours.values().sum::<f64>();
        let screen_time = pdata.screen_time_hours;

        // Draw a bar to depict physical activity
        chart.draw_series(
            std::iter::once(Rectangle::new(
                [(index, 0), (index + bar_width, total_physical_activity)],
                BLUE.mix(0.5).filled(),
            )),
        )?.label("Physical Activity").legend(move |(x, y)| Rectangle::new([(x, y - 5), (x + 20, y + 5)], BLUE.mix(0.5).filled()));

        // Draw a bar to depict screen time
        chart.draw_series(
            std::iter::once(Rectangle::new(
                [(index + bar_width, 0), (index + 2.0 * bar_width, screen_time)],
                RED.mix(0.5).filled(),
            )),
        )?.label("Screen Time").legend(move |(x, y)| Rectangle::new([(x, y - 5), (x + 20, y + 5)], RED.mix(0.5).filled()));

        index += 2; // Increase index to place next pair of bars
    }

    // Draw legend
    chart.configure_series_labels().border_style(&BLACK).draw()?;

    root.present()?;
    info!("Graph drawn and saved as output.png");

    Ok(())
}

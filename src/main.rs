extern crate csv;
use std::fs;

use live_tracking_com_api::api;


fn main() {

    println!("Fetching livetracking data.");

    const RESOLUTION: u32 = 10;
    const RACE_NAME: &str = "2022-garmisch";
    let start_pilot_number = 1265;
    let end_pilot_number = 1266;

    fs::create_dir_all("export");

    for pilot in start_pilot_number..(end_pilot_number + 1) {
        println!("Fetching Pilot: {}", pilot);
        let result = api::get_pilot_data(&RACE_NAME, pilot, RESOLUTION);

        // write to csv
        let filename = format!("export/{}_pilot{}.csv", RACE_NAME, pilot);

        let mut writer = csv::Writer::from_path(filename).unwrap();

        writer.write_record(&["Timestamp", "Latitude", "Longitude", "Altitude", "Speed", "Unknown1", "Unknown2"]).expect("Failed to write data");

        for d in result.unwrap().data.data {
            writer.write_record(&[d.timestamp.to_string(), d.lat.to_string(), d.lon.to_string(), d.altitude.to_string(), d.speed.to_string(), d.unknown1.to_string(), d.unknown2.to_string()]);
        }
    }
}

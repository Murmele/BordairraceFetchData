extern crate csv;
use std::fs;

use live_tracking_com_api::api;


fn main() {

    println!("Fetching livetracking data.");

    const RESOLUTION: u32 = 10;
    const RACE_NAME: &str = "2022-garmisch";

    let pilots = api::get_ranking(RACE_NAME).unwrap();
    fs::create_dir_all("export");

    let url = "http://bordairrace.live-tracking.com//data/2022-garmisch-vendor.json";
    let pilot_data = api::get_pilot_data(url).unwrap();

    for pilot in pilot_data {
        let pilot_id = pilot.id.to_string();
        let last_name = pilot.athlete.last_name;
        let first_name = pilot.athlete.first_name;
        println!("Fetching Pilot: {}", pilot_id);
        let result = api::get_tracking(&RACE_NAME, &pilot_id, RESOLUTION);

        match result {
            Ok(res) => {
                // write to csv
                let filename = format!("export/{}_pilot_{}_{}_{}.csv", RACE_NAME, last_name, first_name, pilot_id);

                let mut writer = csv::Writer::from_path(filename).unwrap();

                writer.write_record(&["Timestamp", "Latitude", "Longitude", "Altitude", "Speed", "Unknown1", "Unknown2"]).expect("Failed to write data");

                for d in res.data.data {
                    writer.write_record(&[d.timestamp.to_string(), d.lat.to_string(), d.lon.to_string(), d.altitude.to_string(), d.speed.to_string(), d.unknown1.to_string(), d.unknown2.to_string()]);
                }
            },
            Err(_) => println!("Unable to fetch track for: {} {} {}", last_name, first_name, pilot_id)

        }
    }
}

#[macro_use]
extern crate text_io;

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

#[derive(Clone)]
struct Car {
    model: String,
    skin: String,
    spectator_mode: String,
    drivername: String,
    team: String,
    guid: String,
    ballast: String,
    restrictor: String,
    ai: String,
}

static PATH: &str = r"<your assetto corsa path>\assettocorsa\server\cfg";

// I know serde exists, but last time I tried it, it didn't work or rust was being rust
fn deserialize() -> Result<Vec<Car>, ()> {
    let config_path: String = PATH.to_owned() + "\\entry_list.ini"; // Path to entry_list.ini
    if File::open(config_path.clone()).is_err() {
        println!("Unable to open file at <{}>", config_path);
        return Err(());
    }
    let file = File::open(&config_path).unwrap();
    let reader = BufReader::new(file);
    let mut cars: Vec<Car> = Vec::new();
    let mut curr_car: Car;
    let mut curr_model = String::new();
    let mut curr_skin = String::new();
    let mut curr_spectator_mode = String::new();
    let mut curr_drivername = String::new();
    let mut curr_team = String::new();
    let mut curr_guid = String::new();
    let mut curr_ballast = String::new();
    let mut curr_restrictor = String::new();
    let mut curr_ai = String::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() { // Maybe also add in regex option or auto all
            println!(
                "Should <model: {}, skin: {}> be AI? [(a)uto/(f)ixed/(n)one], is currently <{}>",
                curr_model, curr_skin, curr_ai
            );
            let ans: String = read!("{}\n");
            curr_ai = if ans.starts_with('a') {
                String::from("auto")
            } else if ans.starts_with('f') || ans.starts_with('y') {
                String::from("fixed")
            } else {
                String::from("none")
            };
            curr_car = Car {
                model: curr_model.clone(),
                skin: curr_skin.clone(),
                spectator_mode: curr_spectator_mode.clone(),
                drivername: curr_drivername.clone(),
                team: curr_team.clone(),
                guid: curr_guid.clone(),
                ballast: curr_ballast.clone(),
                restrictor: curr_restrictor.clone(),
                ai: curr_ai.clone(),
            };
            
            cars.push(curr_car.clone());

            curr_model = String::new();
            curr_skin = String::new();
            curr_spectator_mode = String::new();
            curr_drivername = String::new();
            curr_team = String::new();
            curr_guid = String::new();
            curr_ballast = String::new();
            curr_restrictor = String::new();
            curr_ai = String::new();
            continue;
        }

        match &line[0..2] {
            "MO" => {
                curr_model = line.split('=').nth(1).unwrap().to_string();
            }
            "SK" => {
                curr_skin = line.split('=').nth(1).unwrap().to_string();
            }
            "SP" => {
                curr_spectator_mode = line.split('=').nth(1).unwrap().to_string();
            }
            "DR" => {
                curr_drivername = line.split('=').nth(1).unwrap().to_string();
            }
            "TE" => {
                curr_team = line.split('=').nth(1).unwrap().to_string();
            }
            "GU" => {
                curr_guid = line.split('=').nth(1).unwrap().to_string();
            }
            "BA" => {
                curr_ballast = line.split('=').nth(1).unwrap().to_string();
            }
            "RE" => {
                curr_restrictor = line.split('=').nth(1).unwrap().to_string();
            }
            "AI" => {
                curr_ai = line.split('=').nth(1).unwrap().to_string();
            }
            _ => {}
        }
    }
    Ok(cars)
}

fn serialize(arr: Vec<Car>) -> String {
    let mut result = String::new();
    for (i, car) in arr.iter().enumerate() {
        result.push_str(format!("[CAR_{}]\n", i).as_str());
        result.push_str(format!("MODEL={}\n", car.model).as_str());
        result.push_str(format!("SKIN={}\n", car.skin).as_str());
        result.push_str(format!("SPECTATOR_MODE={}\n", car.spectator_mode).as_str());
        result.push_str(format!("DRIVERNAME={}\n", car.drivername).as_str());
        result.push_str(format!("TEAM={}\n", car.team).as_str());
        result.push_str(format!("GUID={}\n", car.guid).as_str());
        result.push_str(format!("BALLAST={}\n", car.ballast).as_str());
        result.push_str(format!("RESTRICTOR={}\n", car.restrictor).as_str());
        result.push_str(format!("AI={}\n\n", car.ai).as_str());
    }
    result
}

fn main() {
    let cars = deserialize();
    if cars.is_err() {
        println!("Deserialize failed, stopping");
        return;
    }
    let cars = cars.unwrap();
    let mut set: HashSet<String> = HashSet::new();
    let mut paste = String::new();
    if cars.is_empty() {
        println!("No cars found, stopping");
        return;
    }
    for car in cars.clone() {
        if set.insert(car.model.clone()) {
            paste += format!("{};", car.model.as_str()).as_str();
        }
    }
    let paste = &paste[0..paste.len() - 1];
    println!("Paste into CARS under server_cfg.ini:\n{}", paste);
    let mut i = 0;
    while Path::new(&(PATH.to_owned() + &format!("\\entry_list.ini.traffic_{}", i))).exists() {
        i += 1;
    }
    let mut file = match File::create(PATH.to_owned() + &format!("\\entry_list.ini.traffic_{}", i))
    {
        Err(why) => panic!("couldn't create {}: {}, stopping", PATH, why),
        Ok(file) => file,
    };
    println!(
        "\nSaving to {}{}",
        PATH,
        &format!("\\entry_list.ini.traffic_{}", i)
    );
    if file.write_all(serialize(cars.clone()).as_bytes()).is_err() {
        println!("Couldn't save");
    } else {
        println!("Saved successfully");
    }
}

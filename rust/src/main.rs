
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::io;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct WeatherData {
    barrio: Option<String>,
    dia: Option<String>,
    direccion_viento_tarde: Option<String>,
    direccion_viento_temprano: Option<String>,
    horas_de_sol: Option<f32>,
    humedad_tarde: Option<f32>,
    humedad_temprano: Option<f32>,
    id: Option<String>,
    llovieron_hamburguesas_hoy: Option<String>,
    mm_evaporados_agua: Option<f32>,
    mm_lluvia_dia: Option<f32>,
    nubosidad_tarde: Option<f32>,
    nubosidad_temprano: Option<f32>,
    presion_atmosferica_tarde: Option<f32>,
    presion_atmosferica_temprano: Option<f32>,
    rafaga_viento_max_direccion: Option<String>,
    rafaga_viento_max_velocidad: Option<f32>,
    temp_max: Option<f32>,
    temp_min: Option<f32>,
    temperatura_tarde: Option<f32>,
    temperatura_temprano: Option<f32>,
    velocidad_viendo_tarde: Option<f32>,
    velocidad_viendo_temprano: Option<f32>,
}

fn file_creation(record: WeatherData, count:i32) -> Result<(), Box<dyn Error>> {
    let dir: &str = "../../data/rust/";
    match fs::create_dir_all(dir) {
        Ok(it) => it,
        Err(err) => println!("Error creating directory: {}", err)
    };
    let output_path: String = format!("{}{}{}", dir, count, ".json");
    let writer: Result<(), io::Error> = fs::write(
        output_path,
        serde_json::to_string_pretty(&record).expect("Couldn't write to file"),
    );
    match writer {
        Ok(t) => t,
        Err(e) => println!("Error writing file: {}", e)
    };
    Ok(())
}

fn main() {

    let mut rdr = match csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path("../data/data.csv") {
        Ok(it) => it,
        Err(err) => {
            println!("Error reading file: {}", err);
            return;
        }
    };
    let mut count = 0;
    for result in rdr.deserialize() {
        let record: WeatherData = match result {
            Ok(it) => it,
            Err(err) => {
                println!("Error reading record: {}", err);
                continue;
            }
        };
        count += 1;
        match file_creation(record,count) {
            Ok(it) => it,
            Err(err) => {
                println!("Error creating file: {}", err);
                continue;
            }
        };
    }
}

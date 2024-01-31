// use axum::Error;
// use csv::ReaderBuilder;
use log::{error, info};
use rusqlite::{params, Connection, Result};
// use std::fmt::Debug;
use std::fs::File;
use std::io::{self};

pub struct City {
    id: i32,
    pub name: String,
    ascii: String,
    alt_name: String,
    lat: f64,
    long: f64,
    feat_class: String,
    feat_code: String,
    country: String,
    cc2: String,
    pub population: i32,
    elevation: i32,
    dem: i32,
    tz: String,
}

impl City {
    pub fn new(
        id: i32,
        name: String,
        ascii: String,
        alt_name: String,
        lat: f64,
        long: f64,
        feat_class: String,
        feat_code: String,
        country: String,
        cc2: String,
        population: i32,
        elevation: i32,
        dem: i32,
        tz: String,
    ) -> City {
        City {
            id,
            name,
            ascii,
            alt_name,
            lat,
            long,
            feat_class,
            feat_code,
            country,
            cc2,
            population,
            elevation,
            dem,
            tz,
        }
    }
}

pub fn read_csv_and_insert(conn: &Connection, file_path: &str) -> io::Result<()> {
    // Open the CSV file
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => {
            error!("Failed to open file: {}", err);
            return Err(err);
        }
    };
    let mut rdr = csv::Reader::from_reader(file);

    // Iterate over CSV records and insert into the SQLite database
    let mut rows = 0;
    for result in rdr.records() {
        let record = result?;
        let id: i32 = match record[0].parse() {
            Ok(id) => id,
            Err(err) => {
                error!("Error parsing id: {}", err);
                continue;
            }
        };
        let name = &record[1];
        let ascii = &record[2];
        let alt_name = &record[3];
        let lat: f64 = match record[4].parse() {
            Ok(lat) => lat,
            Err(err) => {
                error!("Error parsing latitude: {}", err);
                continue;
            }
        };
        let long: f64 = match record[5].parse() {
            Ok(long) => long,
            Err(err) => {
                error!("Error parsing longitude: {}", err);
                continue;
            }
        };
        let feat_class = &record[6];
        let feat_code = &record[7];
        let country = &record[8];
        let cc2 = &record[9];

        let population: i32 = match record[14].parse() {
            Ok(population) => population,
            Err(_) => -1,
        };
        let elevation: i32 = match record[15].parse() {
            Ok(elevation) => elevation,
            Err(_) => -1,
        };
        let dem: i32 = match record[16].parse() {
            Ok(dem) => dem,
            Err(_) => -1,
        };
        let tz = &record[17];
        let modified_at = &record[18];

        // Insert data into the SQLite database
        match conn.execute(
            "INSERT INTO cities_usa_canada (id, name, ascii, alt_name, lat, long, feat_class, feat_code, country, cc2, population, elevation, dem, tz, modified_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
            params![id, name, ascii, alt_name, lat, long, feat_class, feat_code, country, cc2, population, elevation, dem, tz, modified_at],
        ) {
            Ok(_) => {
                rows += 1;
            }
            Err(err) => error!("update failed: {}", err),
        };
    }
    info!("Total Rows: {}", rows);
    Ok(())
}

pub fn create_table(conn: &Connection) -> Result<()> {
    info!("Creating table");
    match conn.execute(
        "CREATE TABLE cities_usa_canada (
                id INT,
                name VARCHAR(255),
                ascii VARCHAR(255),
                alt_name VARCHAR(255),
                lat DECIMAL(10, 5),
                long DECIMAL(10, 5),
                feat_class CHAR(1),
                feat_code VARCHAR(10),
                country CHAR(2),
                cc2 VARCHAR(2),
                population INT,
                elevation INT,
                dem INT,
                tz VARCHAR(50),
                modified_at DATE
              );",
        [],
    ) {
        Ok(_) => {
            info!("Successfully created table");
            return Ok(());
        }
        Err(err) => {
            error!("Error creating table: {}", err);
            return Err(err);
        }
    };
}

pub fn query_data_by_id(conn: &Connection, id: u32) -> Result<Vec<City>> {
    // Query data from the SQLite database
    info!("Querying for rows id: {}", id);
    let mut stmt = conn.prepare("SELECT * FROM cities_usa_canada WHERE id=(?1)")?;
    let mut rows = stmt.query([id])?;
    let mut cities = Vec::<City>::new();
    while let Some(row) = rows.next()? {
        let id: i32 = row.get(0)?;
        let name: String = row.get(1)?;
        let ascii: String = row.get(2)?; // Adjust the type based on your column types
        let alt_name: String = row.get(3)?; // Adjust the type based on your column types
        let lat: f64 = row.get(4)?; // Adjust the type based on your column types
        let long: f64 = row.get(5)?; // Adjust the type based on your column types
        let feat_class: String = row.get(6)?; // Adjust the type based on your column types
        let feat_code: String = row.get(7)?; // Adjust the type based on your column types
        let country: String = row.get(8)?; // Adjust the type based on your column types
        let cc2: String = row.get(9)?; // Adjust the type based on your column types
        let mut population_num: i32 = -1;
        match row.get(10) {
            Ok(population) => {
                population_num = population;
            }
            Err(err) => {
                error!("Error parsing population: {}", err);
            }
        }
        let elevation: i32 = row.get(11)?; // Adjust the type based on your column types
        let dem: i32 = row.get(12)?; // Adjust the type based on your column types
        let tz: String = row.get(13)?; // Adjust the type based on your column types
        info!("Row: id={}, name={}, ascii={}, alt_name={}, lat={}, long={}, feat_class={}, feat_code={}, country={}, cc2={}, population={}, elevation={}, dem={}, tz={}", id, name, ascii, alt_name, lat, long, feat_class, feat_code, country, cc2, population_num, elevation, dem, tz);
        cities.push(City::new(
            id,
            name,
            ascii,
            alt_name,
            lat,
            long,
            feat_class,
            feat_code,
            country,
            cc2,
            population_num,
            elevation,
            dem,
            tz,
        ));
    }
    // Check if any rows were found
    if cities.is_empty() {
        return Err(rusqlite::Error::QueryReturnedNoRows.into());
    }

    Ok(cities)
}
// pub fn query_data_by_id(conn: &Connection, id: u32) -> Result<()> {
//     info!("Querying data for ID: {}", id);
//     // Query data from the SQLite database
//     let mut stmt = conn.prepare("SELECT * FROM cities_usa_canada")?;
//     let rows = stmt
//         .query_map([], |row| {
//             match row.get::<i32>(0) {
//                 Ok(val) => {
//                     info!("Value: {}", val)
//                 }
//                 Err(err) => {
//                     error!("Error getting value: {}", err);
//                 }
//             }
//             Ok(()) // Assuming the query_map closure is used only to count the rows
//         })?
//         .count();
//     info!("{} rows returned", rows);
//     Ok(())
// }

use csv::ReaderBuilder;
use log::{error, info, warn};
use rusqlite::{params, Connection, Result};
use std::fs::File;
use std::io::{self, Read};

struct City {
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
    modified_at: String,
}

impl City {
    fn new(
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
        modified_at: String,
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
            modified_at,
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
            Err(err) => {
                error!("Error parsing population: {}", err);
                continue;
            }
        };
        let elevation: i32 = match record[15].parse() {
            Ok(elevation) => elevation,
            Err(err) => {
                warn!("Error parsing elevation: {}", err);
                0
            }
        };
        let dem: i32 = match record[16].parse() {
            Ok(dem) => dem,
            Err(err) => {
                error!("Error parsing dem: {}", err);
                continue;
            }
        };
        let tz = &record[17];
        let modified_at = &record[18];

        // Insert data into the SQLite database
        match conn.execute(
            "INSERT INTO cities_usa_canada (id, name, ascii, alt_name, lat, long, feat_class, feat_code, country, cc2, population, elevation, dem, tz, modified_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
            params![id, name, ascii, alt_name, lat, long, feat_class, feat_code, country, cc2, population, elevation, dem, tz, modified_at],
        ) {
            Ok(updated) => {
                rows += 1;
                info!("{} rows were updated. Total Rows: {}", updated, rows);
            }
            Err(err) => error!("update failed: {}", err),
        };

        if rows > 9 {
            info!("Inserted 10 rows exiting");
            break;
        }
    }
    Ok(())
}

pub fn create_table(conn: &Connection) -> Result<()> {
    info!("Creating table");
    let ok = match conn.execute(
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
                admin1 INT,
                admin2 INT,
                admin3 INT,
                admin4 INT,
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

pub fn query_data_by_id(conn: &Connection, id: u32) -> Result<()> {
    // Query data from the SQLite database
    info!("Querying for rows id: {}", id);
    let mut stmt = conn.prepare("SELECT * FROM cities_usa_canada")?;
    let rows = stmt.query_map([], |row| {
        let id: i32 = row.get(0)?; // Adjust the type based on your column types
        let name: String = row.get(1)?; // Adjust the type based on your column types
        let ascii: String = row.get(2)?; // Adjust the type based on your column types
        let alt_name: String = row.get(3)?; // Adjust the type based on your column types
        let lat: f64 = row.get(4)?; // Adjust the type based on your column types
        let long: f64 = row.get(5)?; // Adjust the type based on your column types
        let feat_class: String = row.get(6)?; // Adjust the type based on your column types
        let feat_code: String = row.get(7)?; // Adjust the type based on your column types
        let country: String = row.get(8)?; // Adjust the type based on your column types
        let cc2: String = row.get(9)?; // Adjust the type based on your column types
        let admin1: i32 = row.get(10)?; // Adjust the type based on your column types
        let admin2: i32 = row.get(11)?; // Adjust the type based on your column types
        let admin3: i32 = row.get(12)?; // Adjust the type based on your column types
        let admin4: i32 = row.get(13)?; // Adjust the type based on your column types
        let population: i32 = row.get(14)?; // Adjust the type based on your column types
        let elevation: i32 = row.get(15)?; // Adjust the type based on your column types
        let dem: i32 = row.get(16)?; // Adjust the type based on your column types
        let tz: String = row.get(17)?; // Adjust the type based on your column types
        let modified_at: String = row.get(18)?; // Adjust the type based on your column types

        // Log the row and its columns
        info!("Row: id={}, name={}, ascii={}, alt_name={}, lat={}, long={}, feat_class={}, feat_code={}, country={}, cc2={}, admin1={}, admin2={}, admin3={}, admin4={}, population={}, elevation={}, dem={}, tz={}, modified_at={}", id, name, ascii, alt_name, lat, long, feat_class, feat_code, country, cc2, admin1, admin2, admin3, admin4, population, elevation, dem, tz, modified_at);

        Ok(())
    }).map_err(|err| error!("Error iterating over rows: {}", err)); // Log an error if the iterator fails

    Ok(())
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

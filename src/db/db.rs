use log::{error, info};
use sqlx::PgPool;
use sqlx::Row;
use std::fs::File;
use std::io::{self};

#[derive(serde::Serialize)]
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

pub async fn read_csv_and_insert(pool: &PgPool, file_path: &str) -> io::Result<()> {
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
        if rows > 11 {
            break;
        }
        rows += 1;
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
        // let modified_at = &record[18];

        // Insert data into the SQLite database
        let result = sqlx::query(
            "INSERT INTO cities_usa_canada (id, name, ascii, alt_name, lat, long, feat_class, feat_code, country, cc2, population, elevation, dem, tz) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)")
            .bind(id)
            .bind(name)
            .bind(ascii)
            .bind(alt_name)
            .bind(lat)
            .bind(long)
            .bind(feat_class)
            .bind(feat_code)
            .bind(country)
            .bind(cc2)
            .bind(population)
            .bind(elevation)
            .bind(dem)
            .bind(tz)
            // .bind(modified_at)
        .execute(pool).await;
        match result {
            Ok(_) => {
                info!("Insertion succesful")
            }
            Err(err) => error!("update failed: {}", err),
        };
    }
    Ok(())
}

pub async fn create_table(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    info!("Creating table");

    let result = sqlx::query(
        "CREATE TABLE IF NOT EXISTS cities_usa_canada (
                id INT PRIMARY KEY,
                name VARCHAR(255),
                ascii VARCHAR(255),
                alt_name VARCHAR(255),
                lat DOUBLE PRECISION,
                long DOUBLE PRECISION,
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
    )
    .execute(pool)
    .await;

    match result {
        Ok(_) => {
            info!("Successfully created table");
            Ok(())
        }
        Err(err) => {
            error!("Error creating table: {}", err);
            Err(err)
        }
    }
}

pub async fn query_data_by_id(pool: &PgPool, id: i32) -> Result<Vec<City>, sqlx::Error> {
    // Query data from the SQLite database
    info!("Querying for rows id: {}", id);
    let rows = sqlx::query("SELECT * FROM cities_usa_canada WHERE id=($1)")
        .bind(id)
        .fetch_all(pool)
        .await;
    let mut cities = Vec::<City>::new();
    for row in rows? {
        let id: i32 = row.try_get(0)?;
        let name: String = row.try_get(1)?;
        let ascii: String = row.try_get(2)?; // Adjust the type based on your column types
        let alt_name: String = row.try_get(3)?; // Adjust the type based on your column types
        let lat: f64 = row.try_get(4)?; // Adjust the type based on your column types
        let long: f64 = row.try_get(5)?; // Adjust the type based on your column types
        let feat_class: String = row.try_get(6)?; // Adjust the type based on your column types
        let feat_code: String = row.try_get(7)?; // Adjust the type based on your column types
        let country: String = row.try_get(8)?; // Adjust the type based on your column types
        let cc2: String = row.try_get(9)?; // Adjust the type based on your column types
        let mut population_num: i32 = -1;
        match row.try_get(10) {
            Ok(population) => {
                population_num = population;
            }
            Err(err) => {
                error!("Error parsing population: {}", err);
            }
        }
        let elevation: i32 = row.try_get(11)?; // Adjust the type based on your column types
        let dem: i32 = row.try_get(12)?; // Adjust the type based on your column types
        let tz: String = row.try_get(13)?; // Adjust the type based on your column types
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
        return Err(sqlx::Error::RowNotFound);
    }

    Ok(cities)
}

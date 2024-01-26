use csv::ReaderBuilder;
use log::{error, info, warn};
use rusqlite::{params, Connection, Result};
use std::fs::File;
use std::io::{self, Read};

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
                error!("Error parsing age: {}", err);
                continue;
            }
        };
        let name = &record[1];

        // Insert data into the SQLite database
        match conn.execute(
            "INSERT INTO cities_usa_canada (id, name) VALUES (?1, ?2)",
            params![id, name],
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

// fn print_data(conn: &Connection) -> Result<()> {
//     // Query data from the SQLite database
//     let mut stmt = conn.prepare("SELECT id, name, age FROM people")?;
//     let rows = stmt.query_map(params![], |row| {
//         Ok(Person {
//             id: row.get(0)?,
//             name: row.get(1)?,
//             age: row.get(2)?,
//         })
//     })?;

//     // Print the queried data
//     for person in rows {
//         println!("{:?}", person?);
//     }

//     Ok(())
// }

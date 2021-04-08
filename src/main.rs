
use newbbs::settings::Settings;

#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;

pub fn establish_connection(url: &str) -> MysqlConnection {
    MysqlConnection::establish(url)
        .expect(&format!("Error connecting to {}", url))
}

fn main() {
    let settings = Settings::new().unwrap();
    let _conn = establish_connection(&settings.database.url);

    // Print out our settings
    println!("{:?}", settings);
}
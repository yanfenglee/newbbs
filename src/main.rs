
use newbbs::settings::Settings;


fn main() {
    let settings = Settings::new().unwrap();
    //let _conn = establish_connection(&settings.database.url);

    // Print out our settings
    println!("{:?}", settings);
}
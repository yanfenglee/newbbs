
use newbbs::settings::Settings;

fn main() {
    let settings = Settings::new();

    // Print out our settings
    println!("{:?}", settings);
}
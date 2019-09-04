use std::fs::File;
use std::io::Write;

use chrono::DateTime;
use chrono::Utc;

// Name is beeing appended date and time to avoid naming collisions
pub fn write_ppm_to_file(ppm: &String, name: &str) {
  let date: DateTime<Utc> = Utc::now();
  let dateformatted = date.format("%Y-%m-%d_%H_%M_%S").to_string();
  let mut filename: String;
  if cfg!(windows) {
    filename = format!("c:/temp/{}_{}.ppm", name, dateformatted);
  } else {
    filename = format!("/tmp/{}_{}.ppm", name, dateformatted);
  }
  let mut file = File::create(filename.clone()).expect("Create file failed.");
  file
    .write_all(ppm.as_bytes())
    .expect("Writing file failed.");
  println!("File written to disk. in {}", filename);
}

use std::fs::File;
use std::io::Write;

use chrono::DateTime;
use chrono::Utc;

// Name is beeing appended date and time to avoid naming collisions
pub fn write_ppm_to_file(ppm: &str, name: &str) {
  let filename = create_os_path(name, "ppm");
  let mut file = File::create(filename.clone()).expect("Create file failed.");
  file
    .write_all(ppm.as_bytes())
    .expect("Writing file failed.");
  println!("File written to disk. in {}", filename);
}

pub fn create_os_path(name: &str, suffix: &str) -> String {
  let filename: String;
  let date: DateTime<Utc> = Utc::now();
  let dateformatted = date.format("%Y-%m-%d_%H_%M_%S").to_string();
  if cfg!(windows) {
    filename = format!("c:/temp/{}_{}.{}", name, dateformatted, suffix);
  } else {
    filename = format!("/tmp/{}_{}.{}", name, dateformatted, suffix);
  }
  filename
}

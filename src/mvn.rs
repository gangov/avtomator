use std::{fs, thread};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Command, Stdio};

pub struct MvnHandler {}

impl MvnHandler {
  pub fn mvn_clean_install(target_dir: &Path) {
    for entry in fs::read_dir(&target_dir).expect("cannot read dir") {
      let entry = entry.expect("Failed to get entry");
      let sub_dir_path = entry.path();

      if sub_dir_path.is_dir() {
        let sub_dir_path = sub_dir_path.clone();
        let child = thread::spawn(move || {
          let mut cmd = Command::new("mvn")
            .current_dir(sub_dir_path)
            .arg("clean")
            .arg("install")
            .stdout(Stdio::piped())
            .spawn()
            .expect("Err during mvn install");
          let stdout = cmd.stdout.as_mut().unwrap();
          let stdout_reader = BufReader::new(stdout);
          let stdout_lines = stdout_reader.lines();
          for line in stdout_lines {
            println!("Read: {:?}", line);
          }
          cmd.wait().unwrap();
        });

        child.join().expect("thread panicked");
      }
    }
  }
}
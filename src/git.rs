use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Command, Stdio};

pub struct GitHandler {}

impl GitHandler {
  pub fn clone_repo(target_dir: &Path, repo: String) {
    let mut cmd = Command::new("git")
      .current_dir(target_dir)
      .arg("clone")
      .arg(repo)
      .stdout(Stdio::piped())
      .spawn()
      .expect("Err during cloning");

    {
      let stdout = cmd.stdout.as_mut().unwrap();
      let stdout_reader = BufReader::new(stdout);
      let stdout_lines = stdout_reader.lines();

      for line in stdout_lines {
        println!("Read: {}", line.unwrap());
      }
    }

    cmd.wait().unwrap();
  }
}
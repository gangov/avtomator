use std::{env, io, fs, thread};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use rayon::prelude::*;
use dialoguer::MultiSelect;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let home_folder = Path::new("/Users/gangov");
    let repos = vec!["https://github.com/sqshq/piggymetrics",
                     "https://github.com/spring-projects/spring-petclinic",
                     "https://github.com/Yoh0xFF/java-spring-security-example",
                     "https://github.com/mertakdut/Spring-Boot-Sample-Project"];

    let selection = MultiSelect::new()
      .items(&repos)
      .interact()
      .expect("cannot multiselect");

    println!("{:?}", selection);

    println!("Enter new folder name: ");
    let mut new_folder_name = String::new();
    io::stdin().read_line(&mut new_folder_name).expect("cannot run program");

    Command::new("mkdir")
      .current_dir(home_folder)
      .arg(new_folder_name.trim())
      .output()
      .expect("Err creating folder");

    let target_dir = fs::canonicalize(Path::new(home_folder)
      .join(new_folder_name.trim()))
      .expect("cannot create folder");

    repos
      .par_iter()
      .for_each(|repo| {
          let threaded_target_dir = target_dir.clone();
          clone_repo(&threaded_target_dir, repo)
      });

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

fn clone_repo(target_dir: &PathBuf, repo: &&str) {
    let mut cmd = Command::new("git")
      .current_dir(&target_dir)
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
            println!("Read: {:?}", line.unwrap());
        }
    }

    cmd.wait().unwrap();
}
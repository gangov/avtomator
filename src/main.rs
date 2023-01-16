use std::{env, io};
use std::path::Path;
use std::process::{Command};

static REPOS: &'static [&str] = &["https://github.com/sqshq/piggymetrics",
    "https://github.com/spring-projects/spring-petclinic",
    "https://github.com/Yoh0xFF/java-spring-security-example",
    "https://github.com/mertakdut/Spring-Boot-Sample-Project"];

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let home_folder = Path::new("/Users/gangov");

    println!("Enter new folder name: ");
    let mut new_folder_name = String::new();
    io::stdin().read_line(&mut new_folder_name).expect("cannot run program");


    Command::new("mkdir")
      .current_dir(home_folder)
      .arg(new_folder_name.trim())
      .output()
      .expect("Err creating folder");

    let target_dir = std::fs::canonicalize(Path::new(home_folder)
      .join(new_folder_name.trim()))
      .expect("cannot create folder");

    for repo in REPOS.into_iter() {

        Command::new("git")
          .current_dir(&target_dir)
          .arg("clone")
          .arg(repo)
          .output()
          .expect("Err during cloning");
    };
}
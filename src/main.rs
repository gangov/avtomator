use std::io;
use std::process::Command;

use dialoguer::MultiSelect;
use rayon::prelude::*;

mod pathz;
mod git;
mod mvn;

fn main() {
  let info_settings = pathz::Pathz::new(false);
  let home_dir = info_settings.home_dir.clone();

  let selected = MultiSelect::new()
    .items(&info_settings.git_repos)
    .interact()
    .expect("cannot multiselect");

  println!("Enter new folder name: ");
  let mut new_folder_name = String::new();
  io::stdin().read_line(&mut new_folder_name).expect("cannot run program");

  Command::new("mkdir")
    .current_dir(&home_dir)
    .arg(new_folder_name.trim())
    .output()
    .expect("Err creating folder");

  let target_dir = home_dir.join(new_folder_name.trim());

  selected
    .par_iter()
    .for_each(|idx| {
      let threaded_target_dir = target_dir.clone();
      git::GitHandler::clone_repo(&threaded_target_dir, info_settings.git_repos[*idx].clone())
    });


  mvn::MvnHandler::mvn_clean_install(&target_dir);

}
use std::env;
use std::path::PathBuf;

use directories::BaseDirs;

pub struct Pathz {
  pub home_dir: PathBuf,
  pub git_repos: Vec<String>,
}

impl Pathz {
  fn set_log_trace() {
    env::set_var("RUST_BACKTRACE", "1")
  }

  fn set_home_dir() -> PathBuf {
    BaseDirs::new().expect("cannot initialize BaseDir").home_dir().to_path_buf()
  }

  fn set_git_repos() -> Vec<String> {
    vec!["https://github.com/sqshq/piggymetrics".to_string(),
         "https://github.com/spring-projects/spring-petclinic".to_string(),
         "https://github.com/Yoh0xFF/java-spring-security-example".to_string(),
         "https://github.com/mertakdut/Spring-Boot-Sample-Project".to_string()]
  }

  pub fn new(debug_enabled: bool) -> Self {
    match debug_enabled {
      true => {
        Self::set_log_trace();
        Self::make_me_a_struct()
      }
      false => Self::make_me_a_struct()
    }
  }

  fn make_me_a_struct() -> Self {
    Pathz {
      home_dir: Self::set_home_dir(),
      git_repos: Self::set_git_repos(),
    }
  }
}
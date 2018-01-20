
#[macro_use(quick_error)]
extern crate quick_error;
extern crate regex;
extern crate semver;

pub mod config;
pub mod error;
pub mod repository;
mod version;


#[derive(Debug)]
#[derive(PartialEq)]
pub enum VersionBumpBehavior {
    None,
    Auto,
    Major,
    Minor,
    Patch,
}

pub fn get_version(config:  &config::ProjectConfig,repo: &repository::Repository,  bump_behavior: VersionBumpBehavior, release: bool) -> Result<String,error::CalcverError> {
    let commits = repo.get_commits_since_last_tag();
    let last_tag = repo.get_last_tag();

    version::get_next_version(&config, bump_behavior, &commits, last_tag,release)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct DummyRepo {
        pub  commits: Vec<String>,
        pub last_tag : Option<String>
    }
    impl repository::Repository for DummyRepo {
        fn get_last_tag(&self) -> Option<&str> {
            match self.last_tag {
                Some(ref tag) => Some(tag),
                None=> None
            }
        }
        fn get_commits_since_last_tag(&self) -> &Vec<String> {
            &self.commits
        }
    }

    #[test]
    fn smoke_test(){
        let repo = DummyRepo { 
            commits: vec!["feat: smoke test".to_string()],
            last_tag: Some("v1.2.3".to_string())
        };
        let config =  config::ProjectConfig::from_defaults();

        assert_eq!("1.3.0-alpha.1", get_version(&config,&repo,VersionBumpBehavior::Auto,false).unwrap());
        assert_eq!("1.3.0", get_version(&config,&repo,VersionBumpBehavior::Auto,true).unwrap())
    }

    #[test]
    fn smoke_test_release(){
        let repo = DummyRepo { 
            commits: vec![],
            last_tag: Some("v1.2.3".to_string())
        };
        let config =  config::ProjectConfig::from_defaults();
        assert_eq!("1.2.3", get_version(&config,&repo,VersionBumpBehavior::Auto,true).unwrap())
    }
}

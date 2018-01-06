pub mod tpl;
pub mod semver;

pub enum VersionBumpBehavior {
    Auto,
    Major,
    Minor,
    Patch,
}

pub struct Repository {
    path: String,
    commit_template: String,
    tag_template: String
}


pub fn get_version(repo: &Repository, bump_behavior: VersionBumpBehavior, release: bool) -> String {
    
    
    String::from("1.0.0")
}


#[cfg(test)]
mod tests {
    #[test]
    fn placeholder() {
        assert!(false,"placeholder test");
    }
}

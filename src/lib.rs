extern crate git2;
use git2::{Repository, Signature, Commit, ObjectType, Time, DiffOptions};
pub mod tpl;
pub mod semver;

pub enum VersionBumpBehavior {
    Auto,
    Major,
    Minor,
    Patch,
}

pub struct ProjectRepo {
    pub path: String,
    pub commit_template: String,
    pub tag_template: String
}

pub struct Message {
    message_type: String,
    scope: String,
    short_description: String,
    description: String,
    breaking_change: bool,
    raw: String,
}

pub fn get_version(repo:  &ProjectRepo, bump_behavior: VersionBumpBehavior, release: bool) -> String {
    String::from("1.0.0")
}

pub fn format_commit_message(msg: &Message) -> bool {

    true
}


#[cfg(test)]
mod tests {
    #[test]
    fn placeholder() {
        assert!(true,"placeholder test");
    }
}

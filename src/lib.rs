extern crate git2;
use git2::{Repository, Signature, Commit, ObjectType, Time, DiffOptions};

mod tpl;
mod semver;

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
    // get project commits as Message
    // get latest reachable tag
    // get branch name
    // call get_next_version to determine version
    // if release = true
    //      generate commit log (next iteration)
    //      update version number in files
    //      commit changes
    //      tag commit
    // end if
    // return version
    String::from("1.0.0")
}

pub fn format_commit_message(repo:  &ProjectRepo, msg: &Message) -> bool {
    // convert message to commit message using template
    true
}


#[cfg(test)]
mod tests {
    #[test]
    fn placeholder() {
        assert!(true,"placeholder test");
    }
}

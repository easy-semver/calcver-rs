use super::*;

pub fn get_next_version(bump_behavior: VersionBumpBehavior, commits: Vec<super::Message>,last_tag: String, branch: String ) -> String {
    // currentVersion = last_tag || "0.0.0"
    //normalized_bump_behavior = (bump_behavior != auto) ? bump_behavior : get_bump_behavior(commits);
    let current_version = String::from("1.0.0");
    bump_version(bump_behavior,current_version)
}


fn bump_version(bump_behavior: VersionBumpBehavior, current_version: String) -> String {
    // if version bump behavior
    String::from("1.0.0")
}

fn get_bump_behavior(commits: Vec<super::Message>) -> VersionBumpBehavior {
    // if version bump behavior
    VersionBumpBehavior::Patch
}
use super::*;
use regex::{Regex, RegexSet};

pub fn get_next_version(repo: &ProjectRepo, bump_behavior: VersionBumpBehavior, commits: Vec<String>, last_tag: Option<String>, branch: Option<String> ) -> String {
    //normalized_bump_behavior = (bump_behavior != auto) ? bump_behavior : get_bump_behavior(commits);
    let current_version = get_current_version(last_tag);
    bump_version(bump_behavior,current_version)
}


fn bump_version(bump_behavior: VersionBumpBehavior, current_version: String) -> String {
    // if version bump behavior
    String::from("1.0.0")
}

fn get_bump_behavior(commit_messages: Vec<String>, repo: &ProjectRepo) -> VersionBumpBehavior {  
    let set = RegexSet::new(&[
        &repo.major_regex
    ]).unwrap(); 
    let mut matches: Vec<_>;
    let mut bump_stat: Vec<_> = vec![];
    let bump_behavior: VersionBumpBehavior;

    for msg in commit_messages {
        matches = set.matches(&msg).into_iter().collect();
        bump_stat.push(matches);
    }
    
    VersionBumpBehavior::Major
}

fn get_current_version(last_tag: Option<String>) -> String {
    last_tag.unwrap_or(String::from("0.0.0"))
}

#[cfg(test)]
mod tests {
    #[test]
    fn last_tag_is_empty() {
        assert_eq!("0.0.0", super::get_current_version(None));
    }

    #[test]
    fn last_tag_has_value() {
        assert_eq!("1.0.0", super::get_current_version(Some("1.0.0".to_string())));
    }

    #[test]
    fn match_test(){
        let tae = vec!["feat".to_string(), "fix".to_string(), "poop".to_string()];
        let repo =  super::ProjectRepo {
            path: "".to_string(),
            commit_template: "".to_string(),
            tag_template: "".to_string(),
            branch_template: "".to_string(),
            major_regex: "[a-z]".to_string(),
            minor_regex: "".to_string(),
            patch_regex: "".to_string()
        };

        assert_eq!(super::VersionBumpBehavior::Major, super::get_bump_behavior(tae, &repo));
    }
}

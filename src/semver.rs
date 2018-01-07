use super::*;
use super::error;
use regex::{Regex, RegexSet};

pub fn get_next_version(repo: &ProjectRepo, bump_behavior: VersionBumpBehavior, commits: Vec<String>, last_tag: Option<String>, branch: Option<String> ) -> Result<String,error::CalcverError> {
    let normalized_bump_behavior = match bump_behavior {
        VersionBumpBehavior::Auto => get_bump_behavior(&repo, &commits)?,
        _=> bump_behavior
    };
    let current_version = get_current_version(last_tag);
    Ok(bump_version(normalized_bump_behavior,current_version))
}


fn bump_version(bump_behavior: VersionBumpBehavior, current_version: String) -> String {
    // if version bump behavior
    String::from("1.0.0")
}

fn get_bump_behavior(repo: &ProjectRepo, commit_messages: &Vec<String> ) -> Result<VersionBumpBehavior,error::CalcverError> {  
    let set = RegexSet::new(&[
        &repo.major_regex,
        &repo.minor_regex,
        &repo.patch_regex,
    ])?; 
    let mut  bump_behavior = VersionBumpBehavior::Patch;

    for msg in commit_messages {
        let matches = set.matches(&msg);
        if matches.matched(0) {
            return Ok(VersionBumpBehavior::Major)
        } else if matches.matched(1) {
            bump_behavior = VersionBumpBehavior::Minor;
        }
    }
    
    Ok(bump_behavior)
}

fn get_current_version(last_tag: Option<String>) -> String {
    last_tag.unwrap_or(String::from("0.0.0"))
}

#[cfg(test)]
mod tests {
    use super::*;
    fn get_project() -> ProjectRepo {
        ProjectRepo {
            path: "".to_string(),
            commit_template: "".to_string(),
            tag_template: "".to_string(),
            branch_template: "".to_string(),
            major_regex: "BREAKING CHANGE".to_string(),
            minor_regex: "^feat".to_string(),
            patch_regex: "^fix".to_string()
        }
    }

    #[test]
    fn last_tag_is_empty() {
        assert_eq!("0.0.0", get_current_version(None));
    }

    #[test]
    fn last_tag_has_value() {
        assert_eq!("1.0.0", get_current_version(Some("1.0.0".to_string())));
    }

    #[test]
    fn bump_minor(){
        let tae = vec!["feat".to_string(), "fix".to_string(), "poop".to_string()];
        let repo =  get_project();

        assert_eq!(VersionBumpBehavior::Minor, get_bump_behavior(&repo,&tae).unwrap());
    }

    #[test]
    fn bump_major(){
        let tae = vec!["feat: messsage\n\ndesc\n\nBREAKING CHANGES: some breaking change".to_string(), "fix: message".to_string(), "feat: message\n\n".to_string()];
        let repo =  get_project();

        assert_eq!(VersionBumpBehavior::Major, get_bump_behavior(&repo,&tae).unwrap());
    }

    #[test]
    fn bump_patch(){
        let tae = vec!["docs: messsage\n\ndesc\n\ncloses #5".to_string(), "fix: message".to_string(), "fix: message\n\n".to_string()];
        let repo =  get_project();

        assert_eq!(VersionBumpBehavior::Patch, get_bump_behavior(&repo,&tae).unwrap());
    }

     #[test]
    fn patch_if_empty(){
        let tae = vec![];
        let repo =  get_project();

        assert_eq!(VersionBumpBehavior::Patch, get_bump_behavior(&repo,&tae).unwrap());
    }

     #[test]
    fn patch_if_no_matches(){
        let tae = vec!["poop".to_string()];
        let repo =  get_project();

        assert_eq!(VersionBumpBehavior::Patch, get_bump_behavior(&repo,&tae).unwrap());
    }
}

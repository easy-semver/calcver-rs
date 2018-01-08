use super::*;
use super::error;
use regex::{RegexSet,Regex};

pub fn get_next_version(repo: &project::Project, bump_behavior: VersionBumpBehavior, commits: Vec<String>, last_tag: Option<&str>) -> Result<String,error::CalcverError> {
    let normalized_bump_behavior = match bump_behavior {
        VersionBumpBehavior::Auto => get_bump_behavior(&repo, &commits)?,
        _=> bump_behavior
    };
    let current_version = get_current_version(&repo,last_tag)?;
    Ok(bump_version(normalized_bump_behavior,current_version))
}


fn bump_version(bump_behavior: VersionBumpBehavior, current_version: String) -> String {
    // if version bump behavior
    String::from("1.0.0")
}

fn get_bump_behavior(repo: &project::Project, commit_messages: &Vec<String> ) -> Result<VersionBumpBehavior,error::CalcverError> {  
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

fn get_current_version(repo: &project::Project,last_tag: Option<&str>) -> Result<String,error::CalcverError> {
    let r = Regex::new(&repo.tag_regex)?;
    match last_tag {
        Some(tag) => match r.find(&tag) {
            Some(tag) => Ok(tag.as_str().to_string()),
            None=> Ok(String::from("0.0.0"))
        },
        None => Ok(String::from("0.0.0"))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    fn get_project() -> project::Project {
        project::Project::from(".").finalize()
    }

    #[test]
    fn empty_tag_is_version_0() {
        assert_eq!("0.0.0", get_current_version(&get_project(),None).unwrap());
    }

    #[test]
    fn unmatched_tag_is_version_0() {
        assert_eq!("0.0.0", get_current_version(&get_project(),Some("unrelated-tag")).unwrap());
    }

    #[test]
    fn last_tag_has_value() {
        assert_eq!("1.2.3", get_current_version(&get_project(),Some("1.2.3")).unwrap());
    }

    #[test]
    fn bump_behavior_minor(){
        let tae = vec!["feat".to_string(), "fix".to_string(), "poop".to_string()];
        let repo =  get_project();

        assert_eq!(VersionBumpBehavior::Minor, get_bump_behavior(&repo,&tae).unwrap());
    }

    #[test]
    fn bump_behavior_major(){
        let tae = vec!["feat: messsage\n\ndesc\n\nBREAKING CHANGE: some breaking change".to_string(), "fix: message".to_string(), "feat: message\n\n".to_string()];
        let repo =  get_project();

        assert_eq!(VersionBumpBehavior::Major, get_bump_behavior(&repo,&tae).unwrap());
    }

    #[test]
    fn bump_behavior_patch(){
        let tae = vec!["docs: messsage\n\ndesc\n\ncloses #5".to_string(), "fix: message".to_string(), "fix: message\n\n".to_string()];
        let repo =  get_project();

        assert_eq!(VersionBumpBehavior::Patch, get_bump_behavior(&repo,&tae).unwrap());
    }

     #[test]
    fn bump_behavior_patch_if_empty(){
        let tae = vec![];
        let repo =  get_project();

        assert_eq!(VersionBumpBehavior::Patch, get_bump_behavior(&repo,&tae).unwrap());
    }

     #[test]
    fn bump_behavior_patch_if_no_matches(){
        let tae = vec!["poop".to_string()];
        let repo =  get_project();

        assert_eq!(VersionBumpBehavior::Patch, get_bump_behavior(&repo,&tae).unwrap());
    }
}

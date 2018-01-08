use super::*;
use super::error;
use regex::{RegexSet,Regex};
use semver;

pub fn get_next_version(repo: &project::Project, bump_behavior: VersionBumpBehavior, commits: &Vec<String>, last_tag: Option<&str>) -> Result<String,error::CalcverError> {
    let normalized_bump_behavior = match bump_behavior {
        VersionBumpBehavior::Auto => get_bump_behavior(&repo, &commits)?,
        _=> bump_behavior
    };
    let current_version = get_current_version(&repo,last_tag)?;
    bump_version(normalized_bump_behavior,&current_version)
}


fn bump_version(bump_behavior: VersionBumpBehavior, current_version: &str) -> Result<String,error::CalcverError> {
    // if version bump behavior
    let v = semver::Version::parse(&current_version)?;
    
    let output = match bump_behavior {
        VersionBumpBehavior::Major=> semver::Version::new(v.major + 1, 0, 0),
        VersionBumpBehavior::Minor=> semver::Version::new(v.major, v.minor + 1, 0),
        VersionBumpBehavior::Patch=> semver::Version::new(v.major, v.minor , v.patch + 1),
        _=> return Err(error::CalcverError::of(error::CalcverErrorReason::Unknown))
    };
    Ok(output.to_string())
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
    fn bump_major_auto(){
        let tae = vec!["feat: messsage\n\ndesc\n\nBREAKING CHANGE: some breaking change".to_string(), "fix: message".to_string(), "feat: message\n\n".to_string()];
        let repo =  get_project();

        assert_eq!("2.0.0", get_next_version(&repo,VersionBumpBehavior::Auto, &tae, Some("v1.2.3")).unwrap());
    }
    #[test]
    fn bump_minor_auto(){
        let tae = vec!["docs: messsage\n\ndesc\n\ncloses #5".to_string(), "fix: message".to_string(), "fix: message\n\n".to_string()];
        let repo =  get_project();

        assert_eq!("1.2.4", get_next_version(&repo,VersionBumpBehavior::Auto, &tae, Some("v1.2.3")).unwrap());
    }
    #[test]
    fn bump_patch_auto(){
        let tae = vec!["docs: messsage\n\ndesc\n\ncloses #5".to_string(), "fix: message".to_string(), "fix: message\n\n".to_string()];
        let repo =  get_project();

        assert_eq!("1.2.4", get_next_version(&repo,VersionBumpBehavior::Auto, &tae, Some("v1.2.3")).unwrap());
    }
    #[test]
    fn bump_manual(){
        let tae = vec!["feat: messsage\n\ndesc\n\nBREAKING CHANGE: some breaking change".to_string(), "fix: message".to_string(), "feat: message\n\n".to_string()];
        let repo =  get_project();

        assert_eq!("2.0.0", get_next_version(&repo,VersionBumpBehavior::Major, &tae, Some("v1.2.3")).unwrap());
        assert_eq!("1.3.0", get_next_version(&repo,VersionBumpBehavior::Minor, &tae, Some("v1.2.3")).unwrap());
        assert_eq!("1.2.4", get_next_version(&repo,VersionBumpBehavior::Patch, &tae, Some("v1.2.3")).unwrap());
    }
    #[test]
    fn empty_tag_is_0(){
        let tae = vec!["feat: messsage\n\ndesc\n\nBREAKING CHANGE: some breaking change".to_string(), "fix: message".to_string(), "feat: message\n\n".to_string()];
        let repo =  get_project();

        assert_eq!("0.1.0", get_next_version(&repo,VersionBumpBehavior::Minor, &tae, None).unwrap());
    }
    #[test]
    fn unmatched_tag_is_0(){
        let tae = vec!["feat: messsage\n\ndesc\n\nBREAKING CHANGE: some breaking change".to_string(), "fix: message".to_string(), "feat: message\n\n".to_string()];
        let repo =  get_project();

        assert_eq!("0.1.0", get_next_version(&repo,VersionBumpBehavior::Minor, &tae, Some("feature-tag")).unwrap());
    }
    #[test]
    fn bump_removes_prerelease(){
        let tae = vec!["feat: messsage\n\ndesc\n\nBREAKING CHANGE: some breaking change".to_string(), "fix: message".to_string(), "feat: message\n\n".to_string()];
        let repo =  get_project();

        assert_eq!("1.2.4", get_next_version(&repo,VersionBumpBehavior::Patch, &tae, Some("1.2.3-beta.11+commitsha")).unwrap());
    }
}

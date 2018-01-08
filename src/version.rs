use super::*;
use super::error;
use regex::{RegexSet,Regex};
use semver;

pub fn get_next_version(repo: &project::Project, bump_behavior: VersionBumpBehavior, commits: &Vec<String>, last_tag: Option<&str>, release: bool) -> Result<String,error::CalcverError> {
    if commits.len() == 0 && release {
        return Err(error::CalcverError::of(error::CalcverErrorReason::NoCommitsOnRelease))
    }
    let normalized_bump_behavior = match bump_behavior {
        VersionBumpBehavior::Auto => get_bump_behavior(&repo, &commits)?,
        _=> bump_behavior
    };
    let current_version = get_current_version(&repo,last_tag)?;
    let next_version = bump_version(normalized_bump_behavior,&current_version);
    match (release,next_version) {
        (false,Ok(next_version))=> {
            let mut retval = String::from(next_version);
            if commits.len() > 0 {
                retval.push_str("-");
                retval.push_str(&repo.prerelease_prefix);
                retval.push_str(".");
                retval.push_str(&commits.len().to_string());
            }
            Ok(retval)
        },
        (false,Err(e)) => Err(e),
        (true,x) => x
    }
}


fn bump_version(bump_behavior: VersionBumpBehavior, current_version: &str) -> Result<String,error::CalcverError> {
    // if version bump behavior
    let v = semver::Version::parse(&current_version)?;
    
    let output = match bump_behavior {
        VersionBumpBehavior::Major=> semver::Version::new(v.major + 1, 0, 0),
        VersionBumpBehavior::Minor=> semver::Version::new(v.major, v.minor + 1, 0),
        VersionBumpBehavior::Patch=> semver::Version::new(v.major, v.minor , v.patch + 1),
        VersionBumpBehavior::None => semver::Version::new(v.major, v.minor , v.patch),
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
    let mut  bump_behavior = VersionBumpBehavior::None;

    for msg in commit_messages {
        let matches = set.matches(&msg);
        if matches.matched(0) {
            return Ok(VersionBumpBehavior::Major)
        } else if matches.matched(1) {
            bump_behavior = VersionBumpBehavior::Minor;
        } else {
            bump_behavior = VersionBumpBehavior::Patch;
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
    fn no_bump_if_no_commits(){
        let tae = vec![];
        let repo =  get_project();

        assert_eq!("1.2.3", get_next_version(&repo,VersionBumpBehavior::Auto, &tae, Some("v1.2.3"),false).unwrap());
    }
    #[test]
    fn error_if_release_but_no_commits(){
        let tae = vec![];
        let repo =  get_project();

        assert!(get_next_version(&repo,VersionBumpBehavior::Auto, &tae, Some("v1.2.3"),true).is_err());
    }
    #[test]
    fn bump_major_auto(){
        let tae = vec![
            "feat: messsage\n\ndesc\n\nBREAKING CHANGE: some breaking change".to_string(), 
            "fix: message".to_string(), 
            "feat: message\n\n".to_string()];
        let repo =  get_project();

        assert_eq!("2.0.0", get_next_version(&repo,VersionBumpBehavior::Auto, &tae, Some("v1.2.3"),true).unwrap());
        assert_eq!("2.0.0-alpha.3", get_next_version(&repo,VersionBumpBehavior::Auto, &tae, Some("v1.2.3"),false).unwrap());
    }
    #[test]
    fn bump_minor_auto(){
        let tae = vec![
            "docs: messsage\n\ndesc\n\ncloses #5".to_string(),
            "fix: message".to_string(),
            "feat: message\n\n".to_string()];
        let repo =  get_project();

        assert_eq!("1.3.0", get_next_version(&repo,VersionBumpBehavior::Auto, &tae, Some("v1.2.3"),true).unwrap());
        assert_eq!("1.3.0-alpha.3", get_next_version(&repo,VersionBumpBehavior::Auto, &tae, Some("v1.2.3"),false).unwrap());
    }
    #[test]
    fn bump_patch_auto(){
        let tae = vec![
            "docs: messsage\n\ndesc\n\ncloses #5".to_string(),
            "fix: message".to_string(),
            "fix: message\n\n".to_string()];
        let repo =  get_project();

        assert_eq!("1.2.4", get_next_version(&repo,VersionBumpBehavior::Auto, &tae, Some("v1.2.3"),true).unwrap());
        assert_eq!("1.2.4-alpha.3", get_next_version(&repo,VersionBumpBehavior::Auto, &tae, Some("v1.2.3"),false).unwrap());
    }
    #[test]
    fn bump_patch_if_there_are_commits_even_if_no_match(){
        let tae = vec![
            "docs: messsage\n\ndesc\n\ncloses #5".to_string(),
            "test: message".to_string(),
            "test: message\n\n".to_string()];
        let repo =  get_project();

        assert_eq!("1.2.4", get_next_version(&repo,VersionBumpBehavior::Auto, &tae, Some("v1.2.3"),true).unwrap());
        assert_eq!("1.2.4-alpha.3", get_next_version(&repo,VersionBumpBehavior::Auto, &tae, Some("v1.2.3"),false).unwrap());
    }
    #[test]
    fn bump_manual_pre(){
        let tae = vec![
            "feat: messsage\n\ndesc\n\nBREAKING CHANGE: some breaking change".to_string(), 
            "fix: message".to_string(),
            "feat: message\n\n".to_string()];
        let repo =  get_project();

        assert_eq!("2.0.0-alpha.3", get_next_version(&repo,VersionBumpBehavior::Major, &tae, Some("v1.2.3"),false).unwrap());
        assert_eq!("1.3.0-alpha.3", get_next_version(&repo,VersionBumpBehavior::Minor, &tae, Some("v1.2.3"),false).unwrap());
        assert_eq!("1.2.4-alpha.3", get_next_version(&repo,VersionBumpBehavior::Patch, &tae, Some("v1.2.3"),false).unwrap());
    }
    #[test]
    fn bump_manual_release(){
        let tae = vec![
            "feat: messsage\n\ndesc\n\nBREAKING CHANGE: some breaking change".to_string(), 
            "fix: message".to_string(),
            "feat: message\n\n".to_string()];
        let repo =  get_project();

        assert_eq!("2.0.0", get_next_version(&repo,VersionBumpBehavior::Major, &tae, Some("v1.2.3"),true).unwrap());
        assert_eq!("1.3.0", get_next_version(&repo,VersionBumpBehavior::Minor, &tae, Some("v1.2.3"),true).unwrap());
        assert_eq!("1.2.4", get_next_version(&repo,VersionBumpBehavior::Patch, &tae, Some("v1.2.3"),true).unwrap());
    }
    #[test]
    fn empty_tag_is_0(){
        let tae = vec![
            "feat: messsage\n\ndesc\n\nBREAKING CHANGE: some breaking change".to_string(),
            "fix: message".to_string(),
            "feat: message\n\n".to_string()];
        let repo =  get_project();

        assert_eq!("0.1.0", get_next_version(&repo,VersionBumpBehavior::Minor, &tae, None,true).unwrap());
        assert_eq!("0.1.0-alpha.3", get_next_version(&repo,VersionBumpBehavior::Minor, &tae, None,false).unwrap());
    }
    #[test]
    fn unmatched_tag_is_0(){
        let tae = vec![
            "feat: messsage\n\ndesc\n\nBREAKING CHANGE: some breaking change".to_string(),
            "fix: message".to_string(),
            "feat: message\n\n".to_string()];
        let repo =  get_project();

        assert_eq!("0.1.0", get_next_version(&repo,VersionBumpBehavior::Minor, &tae, Some("feature-tag"),true).unwrap());
        assert_eq!("0.1.0-alpha.3", get_next_version(&repo,VersionBumpBehavior::Minor, &tae, Some("feature-tag"),false).unwrap());
    }
    #[test]
    fn bump_removes_meta(){ // not sure if needed
        let tae = vec![
            "feat: messsage\n\ndesc\n\nBREAKING CHANGE: some breaking change".to_string(),
            "fix: message".to_string(),
            "feat: message\n\n".to_string()];
        let repo =  get_project();

        assert_eq!("1.2.4", get_next_version(&repo,VersionBumpBehavior::Patch, &tae, Some("1.2.3-beta.11+commitsha"),true).unwrap());
        assert_eq!("1.2.4-alpha.3", get_next_version(&repo,VersionBumpBehavior::Patch, &tae, Some("1.2.3-beta.11+commitsha"),false).unwrap());
    }
}

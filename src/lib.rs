#[macro_use(quick_error)]
extern crate quick_error;
extern crate git2;
extern crate handlebars;



use git2::{Repository, Signature, Commit, ObjectType, Time, DiffOptions};
use handlebars::{to_json, Handlebars, Helper, JsonRender, RenderContext, RenderError};

mod semver;
mod error;

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

pub fn format_commit_message(repo:  &ProjectRepo, msg: &Message) -> Result<String,error::CalcverError> {
    // convert message to commit message using template
    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("default",repo.commit_template.to_string())?;
    Ok("".to_string())
}


#[cfg(test)]
mod tests {
    #[test]
    fn placeholder() {
        assert!(true,"placeholder test");
    }
}

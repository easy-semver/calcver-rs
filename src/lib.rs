
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
#[macro_use(quick_error)]
extern crate quick_error;
extern crate git2;
extern crate handlebars;
extern crate regex;

use git2::{Repository, Signature, Commit, ObjectType, Time, DiffOptions};
use handlebars::{to_json, Handlebars, Helper, JsonRender, RenderContext, RenderError};

mod semver;
mod error;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum VersionBumpBehavior {
    Auto,
    Major,
    Minor,
    Patch,
}

pub struct ProjectRepo {
    pub path: String,
    pub commit_template: String,
    pub tag_template: String,
    pub branch_template: String,
    pub major_regex: String,
    pub minor_regex: String,
    pub patch_regex: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    message_type: String,
    scope: Option<String>,
    short_description: String,
    description: String,
    foot: String,
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
    Ok(handlebars.render("default",msg)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_project() -> ProjectRepo {
        ProjectRepo {
            commit_template: String::from("{{message_type}}{{#if scope}}({{ scope }}){{/if}}: {{short_description}}\n\n{{description}}\n\n{{foot}}"),
            path: String::from(""),
            tag_template: String::from(""),
            branch_template:String::from(""),
            major_regex: String::from(""),
            minor_regex: String::from(""),
            patch_regex: String::from(""),
        }
    }
    fn get_message(hasScope: bool) -> Message {
        Message{
            message_type: String::from("feat"),
            scope: match hasScope {
                true => Some(String::from("semver")),
                _ => None
            },
            short_description: String::from("short"),
            description: String::from("long"),
            foot:String::from( "BREAKING CHANGE"),
        }
    }

    #[test]
    fn format_commit_with_scope() {
        let repo = get_project();
        let msg = get_message(true);

        let res = format_commit_message(&repo,&msg).unwrap();
        assert_eq!("feat(semver): short\n\nlong\n\nBREAKING CHANGE",res);
    }
    
    #[test]
    fn format_commit_without_scope() {
        let repo = get_project();
        let msg = get_message(false);

        let res = format_commit_message(&repo,&msg).unwrap();
        assert_eq!("feat: short\n\nlong\n\nBREAKING CHANGE",res);
    }
}

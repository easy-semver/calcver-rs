
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
#[macro_use(quick_error)]
extern crate quick_error;
extern crate git2;
extern crate handlebars;
extern crate regex;
extern crate semver;

use git2::{Repository};
use handlebars::Handlebars;
use std::collections::HashMap;

pub mod project;
pub mod error;
mod version;



#[derive(Debug)]
#[derive(PartialEq)]
pub enum VersionBumpBehavior {
    None,
    Auto,
    Major,
    Minor,
    Patch,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    message_type: String,
    scope: Option<String>,
    short_description: String,
    description: String,
    foot: String,
}

pub fn get_version(repo:  &project::Project, bump_behavior: VersionBumpBehavior, release: bool) -> Result<String,error::CalcverError> {
    let r = Repository::open(&repo.path).unwrap();
    let tags = try!(r.tag_names(Some("*")));
    let tag_map: HashMap<_,_> = tags.iter().filter_map(|t| {
        let name = t.unwrap();
        let obj = r.revparse_single(name).unwrap();

        if let Some(tag) = obj.as_tag() {
            if let Ok(commit) = tag.peel() {
                Some((commit.id(),String::from(name)))
            } else {
                None
            }
        }else if let Some(commit) = obj.as_commit() {
            Some((commit.id(),String::from(name)))
        } else {
            None
        }
    }).collect();
    
    let mut revwalk = r.revwalk()?;
    let mut tag:Option<&str> = None;
    let mut commits:Vec<String> = vec![];
    try!(revwalk.push_head());
    
    for c in revwalk {
        let commit =r.find_commit(c.unwrap()).unwrap();
        if let Some(tg) = tag_map.get(&commit.id()) {
            tag = Some(tg);
            break;
        }
        commits.push(commit.message().unwrap().to_string());
    }

    version::get_next_version(&repo, bump_behavior, &commits, tag,release)
}

pub fn format_commit_message(repo:  &project::Project, msg: &Message) -> Result<String,error::CalcverError> {
    // convert message to commit message using template
    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("default",repo.commit_template.to_string())?;
    Ok(handlebars.render("default",msg)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_project() -> project::Project {
        project::Project::from(".").finalize()
    }
    fn get_message(has_scope: bool) -> Message {
        Message{
            message_type: String::from("feat"),
            scope: match has_scope {
                true => Some(String::from("semver")),
                _ => None
            },
            short_description: String::from("short"),
            description: String::from("long"),
            foot:String::from( "BREAKING CHANGE: ")
        }
    }

    #[test]
    fn format_commit_with_scope() {
        let repo = get_project();
        let msg = get_message(true);

        let res = format_commit_message(&repo,&msg).unwrap();
        assert_eq!("feat(semver): short\n\nlong\n\nBREAKING CHANGE: ",res);
    }
    
    #[test]
    fn format_commit_without_scope() {
        let repo = get_project();
        let msg = get_message(false);

        let res = format_commit_message(&repo,&msg).unwrap();
        assert_eq!("feat: short\n\nlong\n\nBREAKING CHANGE: ",res);
    }
}

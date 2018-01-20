
pub trait Repository {
    fn get_last_tag(&self) -> Option<&str>;
    fn get_commits_since_last_tag(&self) -> &Vec<String>;
}


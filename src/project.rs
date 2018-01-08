
static commit_template_default: &'static str = "{{message_type}}{{#if scope}}({{ scope }}){{/if}}: {{short_description}}\n\n{{description}}\n\n{{foot}}";
static tag_regex_default: &'static str = r"\d+\.\d+\.\d+";
static major_regex_default: &'static str = "BREAKING CHANGE:";
static minor_regex_default: &'static str = "^feat";
static patch_regex_default: &'static str = "^fix";


pub struct Project {
    pub path: String,
    pub commit_template: String,
    pub tag_regex: String,
    pub major_regex: String,
    pub minor_regex: String,
    pub patch_regex: String
}

pub struct ProjectBuilder {
    path : String,
    commit_template: String,
    tag_regex: String,
    major_regex: String,
    minor_regex: String,
    patch_regex: String
}

impl Project {
    pub fn from (path: &str) -> ProjectBuilder {
        ProjectBuilder {
            path: String::from(path),
            commit_template: String::from(commit_template_default),
            tag_regex: String::from(tag_regex_default),
            major_regex: String::from(major_regex_default),
            minor_regex: String::from(minor_regex_default),
            patch_regex: String::from(patch_regex_default),
        }
    }
}

impl ProjectBuilder {
    pub fn with_template(&mut self, commit: &str) ->  &mut ProjectBuilder {
        self.commit_template = String::from(commit);
        self
    }
    pub fn with_regex(&mut self, tag: &str, major: &str, minor: &str, patch: &str) -> &mut ProjectBuilder {
        self.tag_regex = String::from(tag);
        self.major_regex = String::from(major);
        self.minor_regex = String::from(minor);
        self.patch_regex = String::from(patch);
        self
    }

     pub fn finalize(self) -> Project {
        Project {
            path : self.path,
            commit_template: self.commit_template,
            tag_regex: self.tag_regex,
            major_regex: self.major_regex,
            minor_regex: self.minor_regex,
            patch_regex: self.patch_regex
        }
    }
}

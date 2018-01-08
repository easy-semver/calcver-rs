
static COMMIT_TEMPLATE_DEFAULT: &'static str = "{{message_type}}{{#if scope}}({{ scope }}){{/if}}: {{short_description}}\n\n{{description}}\n\n{{foot}}";
static TAG_REGEX_DEFAULT: &'static str = r"\d+\.\d+\.\d+";
static MAJOR_REGEX_DEFAULT: &'static str = "BREAKING CHANGE:";
static MINOR_REGEX_DEFAULT: &'static str = "^feat";
static PATCH_REGEX_DEFAULT: &'static str = "^fix";
static PRERELEASE_PREFIX_DEFAULT: &'static str = "alpha";


pub struct Project {
    pub path: String,
    pub commit_template: String,
    pub prerelease_prefix: String,
    pub tag_regex: String,
    pub major_regex: String,
    pub minor_regex: String,
    pub patch_regex: String
}

pub struct ProjectBuilder {
    path : String,
    commit_template: String,
    prerelease_prefix: String,
    tag_regex: String,
    major_regex: String,
    minor_regex: String,
    patch_regex: String
}

impl Project {
    pub fn from (path: &str) -> ProjectBuilder {
        ProjectBuilder {
            path: String::from(path),
            commit_template: String::from(COMMIT_TEMPLATE_DEFAULT),
            prerelease_prefix: String::from(PRERELEASE_PREFIX_DEFAULT),
            tag_regex: String::from(TAG_REGEX_DEFAULT),
            major_regex: String::from(MAJOR_REGEX_DEFAULT),
            minor_regex: String::from(MINOR_REGEX_DEFAULT),
            patch_regex: String::from(PATCH_REGEX_DEFAULT),
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
            prerelease_prefix: self.prerelease_prefix,
            tag_regex: self.tag_regex,
            major_regex: self.major_regex,
            minor_regex: self.minor_regex,
            patch_regex: self.patch_regex
        }
    }
}

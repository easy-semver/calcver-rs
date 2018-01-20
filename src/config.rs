
pub static COMMIT_TEMPLATE_DEFAULT: &'static str = "{{message_type}}{{#if scope}}({{ scope }}){{/if}}: {{short_description}}\n\n{{description}}\n\n{{foot}}";
pub static TAG_REGEX_DEFAULT: &'static str = r"\d+\.\d+\.\d+";
pub static MAJOR_REGEX_DEFAULT: &'static str = "BREAKING CHANGE:";
pub static MINOR_REGEX_DEFAULT: &'static str = "^feat";
pub static PATCH_REGEX_DEFAULT: &'static str = "^fix";
pub static PRERELEASE_PREFIX_DEFAULT: &'static str = "alpha";


pub struct ProjectConfig {
    pub commit_template: String,
    pub prerelease_prefix: String,
    pub tag_regex: String,
    pub major_regex: String,
    pub minor_regex: String,
    pub patch_regex: String
}

impl ProjectConfig {
    pub fn from_defaults () -> ProjectConfig {
        ProjectConfig {
            commit_template: String::from(COMMIT_TEMPLATE_DEFAULT),
            prerelease_prefix: String::from(PRERELEASE_PREFIX_DEFAULT),
            tag_regex: String::from(TAG_REGEX_DEFAULT),
            major_regex: String::from(MAJOR_REGEX_DEFAULT),
            minor_regex: String::from(MINOR_REGEX_DEFAULT),
            patch_regex: String::from(PATCH_REGEX_DEFAULT),
        }
    }
}
use std::error::Error;
use std::fmt;
use handlebars;
use regex;
use semver;
use git2;


quick_error! {
    #[derive(PartialEq, Debug, Clone)]
    pub enum CalcverErrorReason {
        Unknown {
            display("I dunno what the error is!!!")
            description("Unknown!")
        }
        Library(c: String) {
            display("Library error: {:?}", c)
            description("Handlebars error")
        }
        NoCommitsOnRelease {
            display("Release cannot be true if there are no commits")
            description("Release cannot be true if there are no commits")
        }
    }
}


/// Generic error for now
#[derive(Debug, PartialEq)]
pub struct CalcverError {
    pub reason: CalcverErrorReason,
}

impl CalcverError {
    pub fn of(e: CalcverErrorReason) -> CalcverError {
        CalcverError {
            reason: e,
        }
    }
    pub fn with<E>(cause: E) -> CalcverError
        where
        E: Error + 'static,
    {
        CalcverError::of(CalcverErrorReason::Library(cause.description().to_string()))
    }
}

impl Error for CalcverError {
    fn description(&self) -> &str {
        self.reason.description()
    }
}

impl fmt::Display for CalcverError {
    fn fmt(&self, e: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(e, "Error: {}",self.reason.description())
    }
}


impl From<handlebars::TemplateError> for CalcverError {
     fn from(e: handlebars::TemplateError) -> CalcverError {
        CalcverError::with(e)
    }
}
impl From<handlebars::RenderError> for CalcverError {
     fn from(e: handlebars::RenderError) -> CalcverError {
        CalcverError::with(e)
    }
}

impl From<regex::Error> for CalcverError {
     fn from(e: regex::Error) -> CalcverError {
        CalcverError::with(e)
    }
}

impl From<semver::SemVerError> for CalcverError {
     fn from(e: semver::SemVerError) -> CalcverError {
        CalcverError::with(e)
    }
}

impl From<git2::Error> for CalcverError {
     fn from(e: git2::Error) -> CalcverError {
        CalcverError::with(e)
    }
}

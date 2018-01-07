use std::error::Error;
use std::fmt;
use handlebars;
use regex;


quick_error! {
    #[derive(PartialEq, Debug, Clone)]
    pub enum CalcverErrorReason {
        Unknown {
            display("I dunno what the error is!!!")
            description("Unknown!")
        }
        Handlebars(c: String) {
            display("Handlebars error: {:?}", c)
            description("Handlebars error")
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
        CalcverError::of(CalcverErrorReason::Handlebars(cause.description().to_string()))
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

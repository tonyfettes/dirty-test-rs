use core::fmt::Result as FmtResult;
use core::fmt::{Debug, Display, Formatter};

#[derive(Copy, Clone, Debug)]
pub struct Metadata {
    pub target: &'static str,
    pub feature: Option<&'static str>,
}

impl Display for Metadata {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.feature {
            Some(feature) => f.write_fmt(format_args!("{} ({})", self.target, feature)),
            None => f.write_fmt(format_args!("{}", self.target)),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Error<'a> {
    pub metadata: Metadata,
    pub cause: core::fmt::Arguments<'a>,
}

impl<'a> Display for Error<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_fmt(format_args!(
            "{} test failed at '{}'",
            self.metadata, self.cause
        ))
    }
}

pub type Result<'a> = ::core::result::Result<Metadata, Error<'a>>;

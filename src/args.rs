use std::env;
use std::ffi::OsString;

use clap::{App, AppSettings, ArgMatches};

use wfwalk::errors::*;

pub struct Args<'a> {
    matches: ArgMatches<'a>,
}

impl<'a> Args<'a> {
    pub fn parse() -> Result<Args<'a>> {
        Ok(Args {
            matches: parse_from(env::args_os())?,
        })
    }
}

fn parse_from<'a, I, T>(itr: I) -> Result<ArgMatches<'a>>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
{
    App::new("wfwalk")
        .about("Pull stock information from my Workflowy export.")
        .author(crate_authors!())
        .version(crate_version!())
        .setting(AppSettings::StrictUtf8)
        .setting(AppSettings::UnifiedHelpMessage)

        // Arguments
        .get_matches_from_safe(itr)
        .map_err(Error::from)
}

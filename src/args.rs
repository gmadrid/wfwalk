use clap::{App, AppSettings, Arg, ArgMatches};
use std::env;
use std::ffi::OsString;
use std::path::PathBuf;
use wfwalk::errors::*;

const FILE: &str = "FILE";
const FILE_ENV: &str = "WFWALK_FILE";
const FILE_DEFAULT: &str =
    "/Users/gmadrid/Dropbox/Apps/WorkFlowy/WorkFlowy (gmadrid@gmail.com).txt";

pub struct Args<'a> {
    matches: ArgMatches<'a>,
}

impl<'a> Args<'a> {
    pub fn parse() -> Result<Args<'a>> {
        Ok(Args {
            matches: parse_from(env::args_os())?,
        })
    }

    pub fn file(&self) -> PathBuf {
        self.matches.value_of_os(FILE).unwrap().into()
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
        .arg(
            Arg::with_name(FILE)
                .help("The path to the outline file to use")
                .index(1)
                .env(FILE_ENV)
                .default_value(FILE_DEFAULT)
                .hide_default_value(true),
        )
        .get_matches_from_safe(itr)
        .map_err(Error::from)
}

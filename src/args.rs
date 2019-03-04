use clap::{App, AppSettings, Arg, ArgMatches};
use std::env;
use std::ffi::OsString;
use std::path::PathBuf;
use wfwalk::errors::*;
use std::borrow::Cow;

const FILE: &str = "FILE";
const FILE_ENV: &str = "WFWALK_FILE";
const FILE_DEFAULT: &str =
    "/Users/gmadrid/Dropbox/Apps/WorkFlowy/WorkFlowy (gmadrid@gmail.com).txt";
const SANITY_CHECK: &str = "SANITY_CHECK";
const TOKEN: &str = "TOKEN";
const TOKEN_ENV: &str = "WFWALK_TOKEN";

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
        // unwrap() safe for argument with default value.
        self.matches.value_of_os(FILE).unwrap().into()
    }

    pub fn token(&self) -> Cow<str> {
        // unwrap() safe for required argument.
        self.matches.value_of_lossy(TOKEN).unwrap()
    }

    pub fn do_sanity_check(&self) -> bool {
        self.matches.is_present(SANITY_CHECK)
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
        .arg(
            Arg::with_name(SANITY_CHECK)
                .help("check each stock subtree for inconsistencies and correct form")
                .long("check")
        )
        .arg(
            Arg::with_name(TOKEN)
                .help("the Alphavantage client token")
                .short("t")
                .long("token")
                .required(true)
                .env(TOKEN_ENV)
        )
        .get_matches_from_safe(itr)
        .map_err(Error::from)
}

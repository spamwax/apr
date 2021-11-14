// #![cfg_attr(feature = "dev", feature(plugin))]
// #![cfg_attr(feature = "dev", plugin(clippy))]
// #![cfg_attr(
//     feature = "dev",
//     warn(
//         cast_possible_truncation, cast_possible_wrap, cast_precision_loss, cast_sign_loss, mut_mut,
//         non_ascii_literal, result_unwrap_used, shadow_reuse, shadow_same, unicode_not_nfc,
//         wrong_self_convention, wrong_pub_self_convention
//     )
// )]
// #![cfg_attr(feature = "dev", allow(string_extend_chars))]

extern crate chrono;

#[macro_use]
extern crate failure;
extern crate dirs;
extern crate semver;
extern crate serde;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[macro_use]
extern crate structopt;
// extern crate structopt_derive;

extern crate env_logger;

#[macro_use]
extern crate log;

#[macro_use]
extern crate if_chain;

extern crate alfred;
extern crate alfred_rs;
extern crate rusty_pin;

use std::borrow::Cow;
use std::collections::HashMap;
use std::env;
use std::io;
use std::process;

use alfred_rs::Updater;
use failure::Error;
use rusty_pin::Pinboard;
use structopt::StructOpt;

mod cli;
mod commands;
mod workflow_config;

use crate::cli::{Opt, SubCommand};
use crate::workflow_config::Config;

use crate::commands::Runner;
// use commands::{config, delete, list, post, search, update};
use crate::commands::config;

// TODO: Add a command to search pins that have 'toread' enabled. <01-09-21, Hamid> //
// TODO: We need to come up with actual meaningful tests and deploy them to CIs. <23-08-21, Hamid> //
// TODO: Always add an item that exactly reflects what user has typed. The issue is that when user
// invokes workflow and starts typing a tag we show all tags matching the current input, this
// causes an issue when the top item is, say, "notes", and we want to send only "note" to alfred. <23-08-21, Hamid> //
// TODO: parse Alfred preferences and get number of visible items?
// TODO: Check for all alfred related env. variables before doing anything else.
//       This will prevent unnecessary loading and checking of cache files and then
//       panicing due to missing env. variables.
// TODO: running ./alfred-pinboard-rs update from command line panics (starting from
//       fetch_latest_release)
// TODO: Make sure that we don't show any json-like error in macOS's notification (check issue#27)
// TODO: add an option to disable/enable update checks
// TODO: Dont show full JSON errors after alfred's window has closed, just send a notification <01-04-20, hamid>
// TODO: Can we do something about failuse of parsing user's bookmarks or when the network times out
// TODO: Try to reduce number of calls to get_browser_info in list.rs <04-04-20, hamid>
// TODO: Separate findinig the browser's info into a new separate sud-command so that delete.rs
// does one thing which is deleting and not trying to find the browser's. <07-04-20, hamid>

#[derive(Debug, Fail)]
pub enum AlfredError {
    #[fail(display = "Config file may be corrupted")]
    ConfigFileErr,
    #[fail(display = "Missing config file (did you set API token?)")]
    MissingConfigFile,
    #[fail(display = "Cache: {}", _0)]
    CacheUpdateFailed(String),
    #[fail(display = "Post: {}", _0)]
    Post2PinboardFailed(String),
    #[fail(display = "Delete: {}", _0)]
    DeleteFailed(String),
    #[fail(display = "What did you do?")]
    Other,
}

fn main() {
    /*
         - export alfred_workflow_version=0.11.1
         - export alfred_workflow_data=$HOME/tmp/apr
         - export alfred_workflow_cache=$HOME/tmp/apr/
         - export alfred_workflow_uid=hamid63
         - export alfred_workflow_name="Rusty Pin"
         - export alfred_workflow_bundleid=cc.hamid.alfred-pinboard-rs
         - export alfred_version=3.6
    */
    // env::set_var("alfred_workflow_data", "/Volumes/Home/hamid/tmp/rust");
    // env::set_var("alfred_workflow_cache", "/Volumes/Home/hamid/tmp/rust");
    // env::set_var("alfred_workflow_uid", "hamid63");
    // env::set_var("alfred_workflow_name", "alfred-pinboard-rs");
    // env::set_var("alfred_version", "3.6");
    // env::set_var("RUST_LOG", "rusty_pin=debug,alfred_pinboard_rs=debug");
    // If user has Alfred's debug panel open, print all debug info
    // by setting RUST_LOG environment variable.
    if alfred::env::is_debug() {
        env::set_var(
            "RUST_LOG",
            "rusty_pin=debug,alfred_rs=debug,alfred_pinboard_rs=debug",
        );
    }
    env_logger::init();

    debug!("Parsing input arguments.");
    let opt: Opt = Opt::from_args();

    debug!("Checking if alfred_workflow_* env. variables");
    use env::var_os;
    let env_flags = (
        var_os("alfred_workflow_version").is_some(),
        var_os("alfred_workflow_data").is_some(),
        var_os("alfred_workflow_cache").is_some(),
        var_os("alfred_workflow_uid").is_some(),
        var_os("alfred_workflow_name").is_some(),
        var_os("alfred_version").is_some(),
    );
    match env_flags {
        (true, true, true, true, true, true) => (),
        _ => {
            show_error_alfred(
                "Your workflow is not set up properly. Check alfred_workflow_* env var.",
            );
            process::exit(1);
        }
    }

    let pinboard;
    let config;
    debug!("Deciding on which command branch");
    match opt.cmd {
        SubCommand::Config { .. } => config::run(opt.cmd),
        _ => {
            // If user is not configuring, we will abort upon any errors.
            let setup = setup().unwrap_or_else(|err| {
                show_error_alfred(err.to_string());
                process::exit(1);
            });

            let mut updater = Updater::gh("spamwax/alfred-pinboard-rs").unwrap();

            // If running ./alfred-pinboard-rs self -c, we have to make a network call
            // We do this by forcing the check interval to be zero
            if let SubCommand::SelfUpdate { check, .. } = opt.cmd {
                if check {
                    updater.set_interval(0);
                }
            }
            // updater.set_version("0.13.1");
            // updater.set_interval(60);
            updater.init().expect("cannot start updater!");

            pinboard = setup.1;
            config = setup.0;
            let mut runner = Runner {
                config: Some(config),
                pinboard: Some(pinboard),
                updater: Some(updater),
            };
            match opt.cmd {
                SubCommand::Update => {
                    runner.update_cache();
                }
                SubCommand::List { .. } => {
                    runner.list(opt);
                }
                SubCommand::Search { .. } => {
                    runner.search(opt.cmd);
                }
                SubCommand::Post { .. } => {
                    runner.post(opt.cmd);
                }
                SubCommand::Delete { .. } => {
                    runner.delete(opt.cmd);
                }
                SubCommand::SelfUpdate { .. } => {
                    runner.upgrade(&opt.cmd);
                }
                SubCommand::Rename { .. } => {
                    runner.rename(&opt.cmd);
                }
                _ => unimplemented!(),
            }
        }
    }
}

fn setup<'a, 'p>() -> Result<(Config, Pinboard<'a, 'p>), Error> {
    debug!("Starting in setup");
    let config = Config::setup()?;
    let mut pinboard = Pinboard::new(config.auth_token.clone(), alfred::env::workflow_cache())?;
    pinboard.enable_fuzzy_search(config.fuzzy_search);
    pinboard.enable_tag_only_search(config.tag_only_search);
    pinboard.enable_private_new_pin(config.private_new_pin);
    pinboard.enable_toread_new_pin(config.toread_new_pin);

    Ok((config, pinboard))
}

fn write_to_alfred<'a, 'b, I, J>(items: I, supports_json: bool, vars: Option<J>)
where
    I: IntoIterator<Item = alfred::Item<'a>>,
    J: IntoIterator<Item = (&'b str, &'b str)>,
{
    let exec_counter = env::var("apr_execution_counter").unwrap_or_else(|_| "1".to_string());
    let output_items = items.into_iter().collect::<Vec<alfred::Item>>();
    let mut variables: HashMap<&str, &str> = HashMap::new();

    variables.insert("apr_execution_counter", exec_counter.as_str());
    if let Some(items) = vars {
        items.into_iter().for_each(|(k, v)| {
            variables.insert(k, v);
        });
    }

    debug!("variables: {:?}", variables);
    // Depending on alfred version use either json or xml output.
    if supports_json {
        alfred::json::Builder::with_items(output_items.as_slice())
            .variables(variables)
            .write(io::stdout())
            .expect("Couldn't write items to Alfred");
    } else {
        alfred::xml::write_items(io::stdout(), &output_items)
            .expect("Couldn't write items to Alfred");
    }
}
fn show_error_alfred<'a, T: Into<Cow<'a, str>>>(s: T) {
    debug!("Starting in show_error_alfred");
    let item = alfred::ItemBuilder::new("Error")
        .subtitle(s)
        .icon_path("erroricon.icns")
        .into_item();
    alfred::json::write_items(io::stdout(), &[item]).expect("Can't write to stdout");
    std::process::exit(1);
}

fn alfred_error_item<'a, T: Into<Cow<'a, str>>>(s: T) -> alfred::Item<'a> {
    debug!("Starting in alfred_error");
    alfred::ItemBuilder::new("Error")
        .subtitle(s)
        .icon_path("erroricon.icns")
        .into_item()
}

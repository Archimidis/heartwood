use std::ffi::OsString;
use std::process::Command;
use std::time;

use anyhow::anyhow;

use radicle::cob::patch::{Patch, Patches};
use radicle::node::{Event, Handle};
use radicle::prelude::ReadStorage;
use radicle::storage::git::Repository;
use radicle::storage::RefUpdate;

use crate::terminal as term;
use crate::terminal::args::{Args, Error, Help};

pub const HELP: Help = Help {
    name: "ci-broker",
    description: "Start the CI broker",
    version: env!("CARGO_PKG_VERSION"),
    usage: r#"
Usage
    rad ci-broker

    The `track` command takes either an NID or an RID. Based on the argument, it will
    The `ci-broker` command subscribes to node events. When an announcement for patch
    creation or update is received it will trigger the pipeline for that specific
    patch.

Options

    --help                 Print help
"#,
};

#[derive(Debug)]
pub struct Options;

impl Args for Options {
    fn from_args(args: Vec<OsString>) -> anyhow::Result<(Self, Vec<OsString>)> {
        use lexopt::prelude::*;

        let mut parser = lexopt::Parser::from_args(args);

        while let Some(arg) = parser.next()? {
            match &arg {
                Long("help") => {
                    return Err(Error::Help.into());
                }
                _ => {
                    return Err(anyhow!(arg.unexpected()));
                }
            }
        }

        Ok((Options, vec![]))
    }
}

pub fn run(_options: Options, ctx: impl term::Context) -> anyhow::Result<()> {
    term::info!("CI Broker init ...");
    let profile = ctx.profile()?;

    let node = radicle::Node::new(profile.socket());
    term::info!("Subscribing to node events ...");
    let events = node.subscribe(time::Duration::MAX)?;

    for event in events {
        let event = event?;

        term::info!("Received event {:?}", event);
        match event {
            Event::RefsFetched { remote: _, rid, updated } => {
                for refs in updated {
                    match refs {
                        RefUpdate::Updated { name, old, new } => {
                            term::info!("RefUpdate::Updated: {}, old {}, new {}", name.clone(), term::format::oid(old), term::format::oid(new));
                            if name.contains("xyz.radicle.patch") {
                                let patch_id = name.split('/').last().unwrap();
                                let repository = profile.storage.repository(rid)?;
                                let patch = show_patch_info(&repository, patch_id)?;
                                term::info!("RefUpdate::Updated: {}, oid {}, {}", name, patch_id, patch.title());
                            }
                        }
                        RefUpdate::Created { name, oid } => {
                            term::info!("RefUpdate::Updated: {}, {}", name.clone(), term::format::oid(oid));
                            if name.contains("xyz.radicle.patch") {
                                let patch_id = name.split('/').last().unwrap();
                                let repository = profile.storage.repository(rid)?;
                                let patch = show_patch_info(&repository, patch_id)?;
                                term::info!("RefUpdate::Created: {}, oid {}, {}", name, patch_id, patch.title());
                            }
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }
    Ok(())
}

fn show_patch_info(repository: &Repository, patch_id: &str) -> Result<Patch, anyhow::Error> {
    let patches = Patches::open(repository)?;
    let patch = patches.get(&patch_id.parse().unwrap()).unwrap().unwrap();
    term::info!("--------------------");
    term::info!("Title {}", patch.title());
    term::info!("Head {:?}", *patch.head());

    Ok(patch)
}

pub trait CI {
    fn trigger_job(&self) -> Result<(), anyhow::Error>;
}

struct Concourse {
    // pipeline_config_path: String,
    // user: String,
    // password: String,
    // target: String,
    // url: String,
}

impl CI for Concourse {
    fn trigger_job(&self) -> Result<(), anyhow::Error> {
        let output = Command::new("pwd").output()?;
        println!("{}", String::from_utf8(output.stdout)?);

        // fly -t tutorial login -c http://localhost:8080 -u test -p test
        let output = Command::new("fly")
            .args([
                "-t", "tutorial",
                "login",
                "-c", "http://localhost:8080",
                "-u", "test",
                "-p", "test",
            ])
            .output()?;
        println!("{}", String::from_utf8(output.stdout)?);
        println!("{}", String::from_utf8(output.stderr)?);

        // fly -t tutorial set-pipeline -p hello-world -c hello-world.yml
        let output = Command::new("fly")
            .args([
                "-t", "tutorial",
                "set-pipeline",
                "-p", "hello-world",
                "-c", "/Users/nikolas/Projects/rust/heartwood/radicle-cli/src/commands/hello-world.yml"
            ])
            .output()?;
        println!("{}", String::from_utf8(output.stdout)?);
        println!("{}", String::from_utf8(output.stderr)?);

        // fly -t tutorial trigger-job --job hello-world/hello-world-job
        let output = Command::new("fly")
            .args([
                "-t", "tutorial",
                "trigger-job",
                "--job", "hello-world/hello-world-job",
            ])
            .output()?;
        println!("{}", String::from_utf8(output.stdout)?);
        println!("{}", String::from_utf8(output.stderr)?);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Error;

    use crate::commands::rad_ci_broker::{CI, Concourse};

    #[test]
    fn hello() {
        let ci = Concourse {};
        match ci.trigger_job() {
            Ok(_) => println!("everything went fine"),
            Err(_) => println!("something went wrong"),
        }
    }
}
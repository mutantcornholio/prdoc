//! todo
mod opts;

use clap::{crate_name, crate_version, Parser};
use color_eyre::eyre::bail;
use env_logger::Env;
use log::*;
use opts::*;
use prdoclib::commands::{
	check::CheckCmd, generate::GenerateCmd, load::LoadCmd, schema::SchemaCmd, version::VersionCmd,
};
use std::env;

/// Main entry point of the cli
fn main() -> color_eyre::Result<()> {
	env_logger::Builder::from_env(Env::default().default_filter_or("none")).init();
	color_eyre::install()?;

	let opts: Opts = Opts::parse();
	trace!("opts: {opts:#?}");

	match opts.subcmd {
		Some(SubCommand::Generate(cmd_opts)) => {
			debug!("cmd_opts: {cmd_opts:#?}");
			let _ = GenerateCmd::run(cmd_opts.save, cmd_opts.number, cmd_opts.title, &cmd_opts.output_dir);
			Ok(())
		}

		Some(SubCommand::Check(cmd_opts)) => {
			debug!("cmd_opts: {cmd_opts:#?}");
			CheckCmd::run(&cmd_opts.directory, cmd_opts.file, cmd_opts.number);
			Ok(())
		}

		Some(SubCommand::Scan(cmd_opts)) => {
			debug!("cmd_opts: {cmd_opts:#?}");
			SchemaCmd::run();
			Ok(())
		}

		Some(SubCommand::Load(cmd_opts)) => {
			debug!("cmd_opts: {cmd_opts:#?}");
			let result =
				LoadCmd::run(&cmd_opts.directory, cmd_opts.file, cmd_opts.number, cmd_opts.list, opts.json).unwrap();

			if result.is_some_and(|r| !r) {
				bail!("There were errors found while processing files in {}", cmd_opts.directory.display());
			} else {
				std::process::exit(exitcode::OK);
			}
		}

		Some(SubCommand::Schema(cmd_opts)) => {
			debug!("cmd_opts: {cmd_opts:#?}");
			SchemaCmd::run();
			Ok(())
		}

		None => {
			if opts.version {
				let name = crate_name!();
				let version = crate_version!();
				VersionCmd::run(name, version, opts.json);
				Ok(())
			} else {
				unreachable!("We show help if there is no arg");
			}
		}
	}
}

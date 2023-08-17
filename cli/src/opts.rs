//! This module contains the definition of all commands, sub-commands and arguments
//! supported by the cli.

use clap::{crate_authors, crate_version, ColorChoice, Parser, Subcommand};
use std::path::{Path, PathBuf};

use crate::common::PRNumber;

#[derive(Parser, Debug)]
#[clap(color=ColorChoice::Auto, disable_version_flag = true, arg_required_else_help = true )]
pub struct Opts {
	/// Output as json
	#[clap(short, long, global = true, display_order = 99)]
	pub json: bool,

	// /// Less output
	// #[clap(short, long, global = true, display_order = 99)]
	// pub quiet: bool,

	// /// Do not write color information to the output. This is recommended for scripts.
	// #[clap(short, long, global = true, env = "NO_COLOR", display_order = 99)]
	// pub no_color: bool,

	#[allow(missing_docs)]
	#[clap(subcommand)]
	pub subcmd: Option<SubCommand>,

	/// Show the version
	#[clap(short, long, alias = "V")]
	pub version: bool,
}

/// Define the list of all sub-commands.
#[derive(Subcommand, Debug)]
pub enum SubCommand {
	#[allow(missing_docs)]
	#[clap(version = crate_version!(), author = crate_authors!())]
	Generate(GenOpts),

	#[allow(missing_docs)]
	#[clap(version = crate_version!(), author = crate_authors!())]
	Check(CheckOpts),

	#[allow(missing_docs)]
	#[clap(version = crate_version!(), author = crate_authors!())]
	Scan(ScanOpts),

	#[allow(missing_docs)]
	#[clap(version = crate_version!(), author = crate_authors!())]
	Load(LoadOpts),

	#[allow(missing_docs)]
	#[clap(version = crate_version!(), author = crate_authors!())]
	Schema(SchemaOpts),
}
/// todo
#[derive(Parser, Debug)]
pub struct GenOpts {
	#[clap(index = 1)]
	pub number: PRNumber,

	#[clap()]
	pub title: String,

	#[clap()]
	pub out: Option<PathBuf>,
}

/// Check one or some prdoc files
#[derive(Parser, Debug)]
pub struct CheckOpts {
	/// directory path
	#[clap(exclusive = true)]
	pub file: Option<PathBuf>,

	#[clap(short, long, exclusive = true)]
	pub directory: Option<PathBuf>,

	#[clap(short, long, exclusive = true)]
	pub number: Option<PRNumber>,
}

/// Scan a directory for prdoc files
#[derive(Parser, Debug)]
pub struct ScanOpts {
	/// directory path
	#[clap(index = 1, default_value = ".")]
	pub directory: PathBuf,

	#[clap(short, long)]
	pub all: bool
}

/// Load one or more prdoc
#[derive(Parser, Debug)]
pub struct LoadOpts {
	/// directory path
	#[clap(index = 1, default_value = ".")]
	pub directory: Option<PathBuf>,
}

/// Retrieve the JSON schema that is used internally
#[derive(Parser, Debug)]
pub struct SchemaOpts {
}

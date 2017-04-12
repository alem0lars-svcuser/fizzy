/// CLI module
///
/// Provides facilities to build command line user interactions.
///
/// # CommandLine arguments
///
/// Part of the public API are for parsing commandline arguments:
///
/// ```
/// let matches = fizzy::cli::build_cli().get_matches();
/// let cli_args = fizzy::cli::parse_arguments(&matches);
/// ```
///

use clap::{Arg, App, ArgMatches};
use std::path::Path;

/// Fizzy's high-level representation of commandline arguments.
pub struct CliArgs<'a> {
    pub cfg_file_path: Option<&'a Path>,
    pub verbosity_level: u64,
    pub simulate: bool,
}

/// Build commandline arguments, specifying the allowed values.
pub fn build_cli() -> App<'static, 'static> {
    return app_from_crate!()
        .arg(Arg::with_name("cfg")
            .short("c")
            .long("cfg")
            .aliases(&["config", "configuration"])
            .takes_value(true)
            .value_name("CONFIGURATION_FILE")
            .validator(is_valid_cfg_file)
            .help("Sets a custom configuration file"))
        .arg(Arg::with_name("simulate")
            .short("s")
            .long("simulate")
            .alias("dry-run")
            .help("Dry run in simulation mode, system will be untouched"))
        .arg(Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .alias("verbosity")
            .multiple(true)
            .help("Sets the level of verbosity"));
}

/// Parse the arguments into fizzy's representation of commandline arguments.
pub fn parse_arguments<'a>(matches: &'a ArgMatches) -> CliArgs<'a> {
    let cfg_file_path = match matches.value_of("cfg") {
        Some(val) => Some(Path::new(val)),
        None => None,
    };
    let verbosity_level: u64 = matches.occurrences_of("verbose");
    let simulate = matches.is_present("simulate");

    return CliArgs {
        cfg_file_path: cfg_file_path,
        verbosity_level: verbosity_level,
        simulate: simulate,
    };
}

/// Validate if the provided argument is an existing file.
fn is_valid_cfg_file(val: String) -> Result<(), String> {
    let cfg_file_path = Path::new(&val);

    if !cfg_file_path.is_file() {
        Err(String::from("the configuration file doesn't exist"))
    } else {
        Ok(())
    }
}

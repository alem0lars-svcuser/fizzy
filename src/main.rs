#[macro_use]
extern crate log;

extern crate fizzy;

fn main() {
    // 1: Parse commandline arguments.
    let matches = fizzy::cli::build_cli().get_matches();
    let cli_args = fizzy::cli::parse_arguments(&matches);

    // 2: Read configuration.
    let cfg = fizzy::cfg::read_fizzy_cfg(cli_args.cfg_file_path);

    let verbosity_level = if cli_args.verbosity_level == 0 {
        match cfg.verbosity_level {
            Some(verbosity_level) => verbosity_level,
            None => cli_args.verbosity_level,
        };
    } else {
        cli_args.verbosity_level;
    };

    fizzy::misc::log::init(cli_args.verbosity_level);
    debug!("Logger initialized");

    debug!("Successfully parsed commandline arguments:");
    if cli_args.cfg_file_path.is_some() {
        debug!("Configuration file: {}",
               cli_args.cfg_file_path.unwrap().display());
    }
    debug!("Verbosity level: {}", cli_args.verbosity_level);
    debug!("Simulate: {}", cli_args.simulate);
}

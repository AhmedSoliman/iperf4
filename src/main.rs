use clap::*;

use iperf4::opts::IPerfCliOpts;

fn main() -> anyhow::Result<()> {
    let parser = IPerfCliOpts::parse();
    parser.verbose.log_level_filter();
    env_logger::Builder::new()
        .filter_level(parser.verbose.log_level_filter())
        .init();
    iperf4::run::run_iperf(parser)
}

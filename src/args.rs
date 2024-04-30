use clap::Parser;

const DEFAULT_PLAYER_NUM: u32 = 8;
const DEFAULT_NUM_THOUSAND_ITERATIONS: u32 = 1;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Number of players per table.
    #[arg(default_value_t = DEFAULT_PLAYER_NUM)]
    pub num_players: u32,

    /// Number of thousands of iteratons to run.
    #[arg(default_value_t = DEFAULT_NUM_THOUSAND_ITERATIONS)]
    pub num_thousand_iterations: u32,

    /// Get data for all possible two card combinations.
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,

    /// Log info.
    #[arg(short, long, default_value_t = false)]
    pub debug: bool,
}

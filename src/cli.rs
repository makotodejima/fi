use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "fi",
    after_help = "You can also run `blog SUBCOMMAND -h` to get more information about that subcommand."
)]
pub enum Cli {
    /// Pull account and snapshot data from notion table
    #[structopt(name = "pull")]
    Pull {
        /// Currency to pull data
        #[structopt(short = "c")]
        currency: String,
    },
    /// Display history of accounts
    #[structopt(name = "history")]
    History {
        /// Currency to show history
        #[structopt(short = "c")]
        currency: String,
    },
    /// Display sum
    #[structopt(name = "sum")]
    Sum {
        /// Currency to display sum
        #[structopt(short = "c")]
        currency: String,
    },
}

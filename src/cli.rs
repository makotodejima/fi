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
    /// Display latest sum for given currency
    #[structopt(name = "sum")]
    Sum {
        /// Currency to display sum
        #[structopt(short = "c")]
        currency: String,
    },
    /// Display net worth in given currency
    #[structopt(name = "networth")]
    NetWorth {
        /// Currency to display total in
        #[structopt(short = "c")]
        currency: String,
    },
    /// Delete all data
    #[structopt(name = "delete")]
    Delete,
}

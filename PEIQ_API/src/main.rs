pub mod customers;

use clap::Parser;

/**
 * Minimum offset to start look from
 */
const PEIQ_STARTING_OFFSET: i32 = 80282;
const PEIQ_STARTING_DATE: &str = "2024-05-31";

/**
 * PEIQ has a hard limit of 100 results per api call.
 */
const PEIQ_QUERY_LIMIT: i32 = 100;

#[derive(clap::ValueEnum, Clone, Debug)]
enum ENV {
    PROD,
    TEST,
}

#[derive(Parser, Debug)]
#[clap(author="Stanimir", version="0.1.0", about="about text", long_about = "long long about text")]
struct Args {
    #[clap(long, help="FTP account")]
    ftp_user: String,

    #[clap(value_enum, long, help="Possible values are TEST and PROD")]
    env: ENV,
}

// protected $signature = 'ads:import:peiq
// {ftp_user : FTP account}
// {env=PROD : Possible values are TEST and PROD}
// {ids?* : Import a list of ads manually}
// {--S|--search-start : search for an offset from the start}
// ';
#[warn(dead_code)]
fn main() {

    let args = Args::parse();
dbg!(args);


}

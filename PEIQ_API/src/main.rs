use std::fmt::Display;

use clap::Parser;

use crate::customers::UserDetails;
use crate::ftp_users::FtpUser;

mod customers;
mod ftp_users;

//  Minimum offset to start look from
const _PEIQ_STARTING_OFFSET: i32 = 80282;
const _PEIQ_STARTING_DATE: &str = "2024-05-31";

// PEIQ has a hard limit of 100 results per api call.
const _PEIQ_QUERY_LIMIT: i32 = 100;

#[derive(clap::ValueEnum, Clone, Debug)]
enum ENV {
    PROD,
    TEST,
}

impl Display for ENV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

struct PEIQConfig<'a> {
    // url: String,
    // token: String,
    ftp: Option<FtpUser<'a>>,
    ftp_user: String,
    // ftp_settings: Option<HashMap<String, Option<String>>>,
    // ftp_map_regions: Option<Vec<String>>,
    // event_name_type: String,
    env: String,
    // offset: Option<i32>,
}

#[derive(Parser, Debug)]
#[clap(
    author = "Stanimir",
    version = "0.1.0",
    about = "PEIQ API xml creation and import script",
    long_about = "PEIQ API xml creation and import script"
)]
#[derive(Clone)]
struct Args {
    #[clap(long, help = "FTP account")]
    ftp_user: String,

    #[clap(value_enum, long, help = "Environment to run the script.")]
    env: ENV,

    #[clap(long, short, help = "Start from beginning.")]
    start: bool,

    #[clap(long, short, help = "Download and replace current pdf file in directory.")]
    file: bool,

    #[clap(
        help = "Import a list of ads manually", long, value_parser, num_args = 1.., value_delimiter = ' '
    )]
    ids: Vec<i32>,
}

impl PEIQConfig<'_> {
    fn new(args: Args) -> Self {
        let _ftp = FtpUser::find_ftp_user(&args.ftp_user);
        let _api_settings = api_setting(args.clone());
        Self {
            ftp_user: args.ftp_user,
            env: args.env.to_string(),
            // event_name_type: String::from(""),
            // url: "https://svg-create-test.peiq.cloud/adapi/v1/".to_string(),
            // token: String::from(""),
            // offset: None,

            ftp: None,
            // ftp_settings: Some(HashMap::from([
            //     ("peiq_date".to_string(), Some("".to_string())),
            //     ("peiq_offset".to_string(), None),
            // ])),
            // ftp_map_regions: None,
        }
    }
}

fn api_setting(args: Args) -> Option<UserDetails> {
    customers::users().get(
        &args.env.to_string()
    )
        .unwrap()
        .users
        .get(args.ftp_user.as_str())
        .to_owned()
        .cloned()


    // $this->token = $user['token'];
    // $this->url = $this->users[$this->env]['url'];
    // $this->eventNameType = $this->users[$this->env]['event'];
    //
    // $this->http = Http::withToken($this->token)
    // ->timeout(300)
    // ->withHeaders([
    //               'Cache-control' => 'no-cache',
    //               ])
    // ->acceptJson();
}

#[warn(dead_code)]
fn main() {
    let args = Args::parse();

    let _peiq_config = PEIQConfig::new(args);
}

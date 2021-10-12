#[derive(Debug)]
pub struct AppConfig {
    pub path_to_watch: String,
    pub host: String,
    pub port: u32,
}

pub type StdErr = Box<dyn std::error::Error>;

pub fn read_config() -> Result<AppConfig, StdErr> {
    let app = clap::App::new("Local API server for wot-replays-app")
        .arg(
            clap::Arg::with_name("replays_path")
                .short("p")
                .long("replays-path")
                .takes_value(true)
                .required(true)
                .env("REPLAYS_PATH")
                .help("Path to the replays directory"),
        )
        .arg(
            clap::Arg::with_name("host")
                .long("host")
                .required(true)
                .takes_value(true)
                .env("HOST")
                .default_value("127.0.0.1")
                .help("IP address to bind the server"),
        )
        .arg(
            clap::Arg::with_name("port")
                .long("port")
                .required(true)
                .takes_value(true)
                .env("PORT")
                .default_value("8080")
                .help("Port to bind the server"),
        );
    
    let args = app.get_matches();
    let app_config = AppConfig {
        path_to_watch: args.value_of("replays_path").unwrap_or_default().to_owned(),
        host: args.value_of("host").unwrap_or_default().to_owned(),
        port: args.value_of("port").unwrap_or_default().parse::<u32>()?,
    };
    Ok(app_config)
}
use echo_bot::config::ConfigBuilder;
use echo_bot::logger::{ConsoleLogger, Logger};

fn main() {
    // I feel like here we obfuscate a bit what actually happens.
    // It seems that ConfigBuilder isn't really necessary, we can write the code directly here.

    let config_file = std::env::args().nth(1) // additionally, it's better to keep env::args parsing in main.rs
        .unwrap_or_else(|| "config.yaml".to_string());
    let config_text = std::fs::read_to_string(&config_file)?;
    let config_file = serde_yaml::from_str(&config_text)?;

    // As a bit of general philosophy, I think what happens here is that you
    // want _everything_ related to "config" be in the same file, but that's not
    // necessary the best way to factor functionality.
    //
    // Rather, it's useful to separate IO from program logic, and keep all IO in
    // main.



    let config = ConfigBuilder::new()
        .extract_path()
        .extract_config_body()
        .build();

    let logger = ConsoleLogger::init(config.log_level.clone());
    logger.log_info("logger with config success build");

    echo_bot::run_bot(config, &logger);
}

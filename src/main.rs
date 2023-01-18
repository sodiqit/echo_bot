use echo_bot::config::ConfigBuilder;
use echo_bot::logger::{ConsoleLogger, Logger};

fn main() {
    let config = ConfigBuilder::new()
        .extract_path()
        .extract_config_body()
        .build();

    let logger = ConsoleLogger::init(config.log_level.clone());
    logger.log_info("logger with config success build");

    echo_bot::run_bot(config, &logger);
}

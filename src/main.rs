use echo_bot::config::ConfigBuilder;

fn main() {
    let config = ConfigBuilder::new()
        .extract_path()
        .extract_config_body()
        .build();

    echo_bot::run_bot(config);
}

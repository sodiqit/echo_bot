mod config;

use config::ConfigBuilder;

fn main() {
    let config = ConfigBuilder::new()
        .extract_path()
        .extract_config_body()
        .build();

}

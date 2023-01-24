# Echo-bot

This is simply bot for sending back user message. It's bot support 2 client: telegram and console

## How to use

```sh
git clone git@github.com:sodiqit/echo_bot.git echo_bot
```

To launch app create `config.yaml` see `./config.example.yaml` and use `cargo run` command.

`If you config.yaml file is in a different location from the root dir - you can pass relative path via env args`:

```sh
cargo run -- --config directory/config.yaml
```

Run tests:

```sh
cargo test
```

## Features

### commands

* `/help` - print message from config(help_msg).
* `/repeat` - print message from config(repeat_msg) with current value repeat number. If mode set to `console`. User must type integer(must be greater then 0). If mode set to `telegram` - user can choice number from inline keyboard in chat with bot.

For only `console` mode user can use `/exit` command to disable bot.

`telegram` mode support plain text and video messages.

### configurable

log_level, help message, repeat message, default repeat number can be configurable. See `./config.example.yaml`

`For telegram mode bot_token must be exist!`

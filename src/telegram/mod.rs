mod client;
pub mod client_types;
mod handler;
mod keyboard;
mod response_parser;
mod update_converter;

use std::{thread::sleep, time::Duration};

use self::{
    client::{TelegramClient, TelegramHttpClient},
    client_types::{ClientError, TelegramCommand},
    handler::{Handler, TelegramHandler, TelegramState},
};
use crate::{commands::Command, config::Config, logger::Logger};

pub fn run_bot(config: &Config, logger: &impl Logger) -> Result<(), ClientError> {
    let token = config.bot_token.as_ref().unwrap();
    let client = TelegramHttpClient::new(token, logger);
    let mut state = TelegramState::new();
    let handler = TelegramHandler::new(logger, &client);

    client.set_commands(vec![
        TelegramCommand::new(
            Command::new("/help"),
            "provide help message about bot".to_string(),
        ),
        TelegramCommand::new(
            Command::new("/repeat"),
            "provide menu for choose repeat number".to_string(),
        ),
    ])?;

    loop {
        communicate(&mut state, &client, &handler, config)?;
        sleep(Duration::from_secs(1));
    }
}

fn communicate<T: TelegramClient, H: Handler<T>>(
    state: &mut TelegramState,
    client: &T,
    handler: &H,
    config: &Config,
) -> Result<(), T::E> {
    let offset = state.last_update_id.map_or(0, |v| v + 1);
    client
        .get_updates(offset)?
        .into_iter()
        .try_for_each(|update| handler.handle(config, state, update))
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use crate::config::{BotMode, ConfigBuilder};

    use super::{
        client_types::{CallbackQuery, Chat, Message, Payload, RawUpdate, User, Video},
        keyboard::InlineKeyboardMarkup,
        *,
    };

    struct MockLogger {}

    impl Logger for MockLogger {
        fn log(&self, _log_level: crate::logger::LogLevel, _msg: &str) {}
    }

    struct MockTelegramClient {
        pub updates: RefCell<Vec<RawUpdate>>,
        pub handled_ids: RefCell<Vec<u64>>,
        pub commands: RefCell<Vec<TelegramCommand>>,
        pub messages: RefCell<Vec<Message>>,
        pub videos: RefCell<Vec<Video>>,
        pub answers_on_callback: RefCell<Vec<CallbackQuery>>,
        pub keyboards: RefCell<Vec<InlineKeyboardMarkup>>,
    }

    impl MockTelegramClient {
        fn new(updates: Vec<RawUpdate>) -> Self {
            Self {
                updates: RefCell::new(updates),
                handled_ids: RefCell::new(vec![]),
                commands: RefCell::new(vec![]),
                messages: RefCell::new(vec![]),
                videos: RefCell::new(vec![]),
                answers_on_callback: RefCell::new(vec![]),
                keyboards: RefCell::new(vec![]),
            }
        }
    }

    impl TelegramClient for MockTelegramClient {
        type E = ();
        fn get_updates(&self, offset: u64) -> Result<Vec<RawUpdate>, Self::E> {
            self.handled_ids.borrow_mut().push(offset);
            Ok(self.updates.borrow().clone())
        }

        fn send(&self, chat_id: u64, payload: client_types::Payload) -> Result<Message, Self::E> {
            let mut video: Option<Video> = None;
            let mut text: Option<String> = None;

            match payload {
                Payload::Text(txt) => {
                    text = Some(txt.to_string());
                    self.messages.borrow_mut().push(Message {
                        chat: Chat { id: chat_id },
                        from: None,
                        video: video.clone(),
                        text: text.clone(),
                    });
                }
                Payload::Video(file_id) => {
                    video = Some(Video {
                        file_id: file_id.to_string(),
                    });
                    self.videos.borrow_mut().push(Video {
                        file_id: file_id.to_string(),
                    });
                }
                Payload::TextWithKeyboard(keyboard, txt) => {
                    text = Some(txt.to_string());
                    self.messages.borrow_mut().push(Message {
                        chat: Chat { id: chat_id },
                        from: None,
                        video: video.clone(),
                        text: text.clone(),
                    });
                    self.keyboards.borrow_mut().push(keyboard);
                }
            }

            Ok(Message {
                chat: Chat { id: chat_id },
                from: Some(User {
                    id: chat_id,
                    is_bot: false,
                }),
                video,
                text,
            })
        }

        fn answer_callback_query(&self, id: &str, text: &str) -> Result<bool, Self::E> {
            self.answers_on_callback.borrow_mut().push(CallbackQuery {
                id: id.to_string(),
                message: Message {
                    chat: Chat { id: 1 },
                    from: None,
                    video: None,
                    text: Some(text.to_string()),
                },
                data: "".to_string(),
            });

            Ok(true)
        }

        fn set_commands(&self, commands: Vec<TelegramCommand>) -> Result<bool, Self::E> {
            commands.into_iter().for_each(|command| {
                self.commands.borrow_mut().push(command);
            });

            Ok(true)
        }
    }

    fn prepare(updates: Vec<RawUpdate>) -> (TelegramState, MockLogger, MockTelegramClient, Config) {
        let config = ConfigBuilder::build_default(BotMode::Telegram);
        let logger = MockLogger {};
        let client = MockTelegramClient::new(updates);
        let state = TelegramState::new();

        (state, logger, client, config)
    }

    #[test]
    fn should_success_set_commands() {
        let (_, _, client, _) = prepare(vec![]);
        let commands: Vec<TelegramCommand> = vec![
            TelegramCommand::new(
                Command::new("/help"),
                "provide help message about bot".to_string(),
            ),
            TelegramCommand::new(
                Command::new("/repeat"),
                "provide menu for choose repeat number".to_string(),
            ),
        ];

        client.set_commands(commands.clone()).unwrap();

        assert_eq!(client.commands.borrow().clone(), commands);
    }

    #[test]
    fn should_success_repeat_messages_with_default_number() {
        let msg = Message {
            chat: Chat { id: 1 },
            from: None,
            video: None,
            text: Some("test".to_string()),
        };
        let updates = vec![RawUpdate {
            update_id: 1,
            message: Some(msg.clone()),
            callback_query: None,
        }];
        let (mut state, logger, client, config) = prepare(updates);
        let handler = TelegramHandler::new(&logger, &client);

        communicate(&mut state, &client, &handler, &config).unwrap();

        assert_eq!(client.messages.borrow().clone(), vec![msg]);
    }

    #[test]
    fn should_success_save_last_handled_id() {
        let msg = Message {
            chat: Chat { id: 1 },
            from: None,
            video: None,
            text: Some("test".to_string()),
        };
        let updates = vec![
            RawUpdate {
                update_id: 1,
                message: Some(msg.clone()),
                callback_query: None,
            },
            RawUpdate {
                update_id: 2,
                message: Some(msg),
                callback_query: None,
            },
        ];
        let (mut state, logger, client, config) = prepare(updates);
        let handler = TelegramHandler::new(&logger, &client);

        communicate(&mut state, &client, &handler, &config).unwrap();
        communicate(&mut state, &client, &handler, &config).unwrap();

        assert_eq!(state.last_update_id, Some(2));
        assert_eq!(client.handled_ids.borrow().clone(), vec![0, 3]);
    }

    #[test]
    fn should_ignore_update_if_bot_him_send() {
        let msg = Message {
            chat: Chat { id: 1 },
            from: Some(User {
                id: 1,
                is_bot: true,
            }),
            video: None,
            text: Some("test".to_string()),
        };
        let updates = vec![RawUpdate {
            update_id: 1,
            message: Some(msg),
            callback_query: None,
        }];
        let (mut state, logger, client, config) = prepare(updates);
        let handler = TelegramHandler::new(&logger, &client);

        communicate(&mut state, &client, &handler, &config).unwrap();

        assert_eq!(state.last_update_id, Some(1));
        assert_eq!(client.messages.borrow().clone(), vec![]);
    }

    #[test]
    fn should_success_repeat_video_messages() {
        let msg = Message {
            chat: Chat { id: 1 },
            from: None,
            video: Some(Video {
                file_id: "1".to_string(),
            }),
            text: None,
        };
        let updates = vec![RawUpdate {
            update_id: 1,
            message: Some(msg.clone()),
            callback_query: None,
        }];
        let (mut state, logger, client, config) = prepare(updates);
        let handler = TelegramHandler::new(&logger, &client);

        communicate(&mut state, &client, &handler, &config).unwrap();

        assert_eq!(client.videos.borrow().clone(), vec![msg.video.unwrap()]);
    }

    #[test]
    fn should_success_handle_commands() {
        let mut msg1 = Message {
            chat: Chat { id: 1 },
            from: None,
            video: None,
            text: Some("/help".to_string()),
        };
        let mut msg2 = Message {
            chat: Chat { id: 1 },
            from: None,
            video: None,
            text: Some("/repeat".to_string()),
        };
        let mut msg3 = Message {
            chat: Chat { id: 1 },
            from: None,
            video: None,
            text: Some("/invalid".to_string()),
        };
        let updates = vec![
            RawUpdate {
                update_id: 1,
                message: Some(msg1.clone()),
                callback_query: None,
            },
            RawUpdate {
                update_id: 2,
                message: Some(msg2.clone()),
                callback_query: None,
            },
            RawUpdate {
                update_id: 3,
                message: Some(msg3.clone()),
                callback_query: None,
            },
        ];
        let (mut state, logger, client, config) = prepare(updates);
        let handler = TelegramHandler::new(&logger, &client);

        communicate(&mut state, &client, &handler, &config).unwrap();

        msg1.text = Some(config.help_msg);
        msg2.text = Some(format!(
            "{}\nCurrent repeat number is {}",
            config.repeat_msg, config.default_repeat_number
        ));
        msg3.text = Some("get unknown command: /invalid".to_string());

        assert_eq!(client.messages.borrow().clone(), vec![msg1, msg2, msg3]);
    }

    #[test]
    fn should_success_send_keyboard() {
        let msg = Message {
            chat: Chat { id: 1 },
            from: None,
            video: None,
            text: Some("/repeat".to_string()),
        };
        let updates = vec![RawUpdate {
            update_id: 1,
            message: Some(msg),
            callback_query: None,
        }];
        let (mut state, logger, client, config) = prepare(updates);
        let handler = TelegramHandler::new(&logger, &client);

        communicate(&mut state, &client, &handler, &config).unwrap();

        assert_eq!(client.keyboards.borrow().clone().len(), 1);
    }

    #[test]
    fn should_success_change_repeat_number_via_answer_callback() {
        let msg = Message {
            chat: Chat { id: 1 },
            from: None,
            video: None,
            text: Some("/repeat".to_string()),
        };
        let updates = vec![
            RawUpdate {
                update_id: 1,
                message: Some(msg.clone()),
                callback_query: None,
            },
            RawUpdate {
                update_id: 2,
                message: None,
                callback_query: Some(CallbackQuery {
                    id: "1".to_string(),
                    message: msg,
                    data: "3".to_string(),
                }),
            },
        ];
        let (mut state, logger, client, config) = prepare(updates);
        let handler = TelegramHandler::new(&logger, &client);

        communicate(&mut state, &client, &handler, &config).unwrap();

        assert_eq!(state.repeat_numbers.get(&1), Some(&3));
        assert_eq!(
            client.answers_on_callback.borrow().clone(),
            vec![CallbackQuery {
                id: "1".to_string(),
                message: Message {
                    chat: Chat { id: 1 },
                    from: None,
                    video: None,
                    text: Some("Repeats number was changed to 3".to_string())
                },
                data: "".to_string(),
            }]
        );
    }

    #[test]
    fn should_success_repeat_messages_after_change_repeat_number() {
        let msg = Message {
            chat: Chat { id: 1 },
            from: None,
            video: None,
            text: Some("/repeat".to_string()),
        };
        let msg1 = Message {
            chat: Chat { id: 1 },
            from: None,
            video: None,
            text: Some("test".to_string()),
        };
        let msg2 = Message {
            chat: Chat { id: 1 },
            from: None,
            video: Some(Video {
                file_id: "1".to_string(),
            }),
            text: None,
        };
        let updates = vec![
            RawUpdate {
                update_id: 1,
                message: Some(msg.clone()),
                callback_query: None,
            },
            RawUpdate {
                update_id: 2,
                message: None,
                callback_query: Some(CallbackQuery {
                    id: "1".to_string(),
                    message: msg,
                    data: "2".to_string(),
                }),
            },
            RawUpdate {
                update_id: 3,
                message: Some(msg1.clone()),
                callback_query: None,
            },
            RawUpdate {
                update_id: 4,
                message: Some(msg2.clone()),
                callback_query: None,
            },
        ];
        let (mut state, logger, client, config) = prepare(updates);
        let handler = TelegramHandler::new(&logger, &client);

        communicate(&mut state, &client, &handler, &config).unwrap();

        assert_eq!(state.repeat_numbers.get(&1), Some(&2));
        assert_eq!(
            client
                .messages
                .borrow()
                .clone()
                .into_iter()
                .rev()
                .take(2)
                .collect::<Vec<_>>(),
            vec![msg1.clone(), msg1]
        );
        assert_eq!(client.messages.borrow().len(), 3);
        assert_eq!(
            client.videos.borrow().clone(),
            vec![msg2.clone().video.unwrap(), msg2.video.unwrap()]
        );
    }
}

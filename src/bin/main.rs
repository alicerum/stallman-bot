extern crate config;
extern crate futures;
extern crate stallman_bot;
extern crate telegram_bot;
extern crate tokio_core;

use futures::Stream;
use stallman_bot::database;
use stallman_bot::settings;
use std::env;
use std::process;
use telegram_bot::CanReplySendMessage;
use telegram_bot::types::{MessageKind, UpdateKind};
use tokio_core::reactor::Core;

fn main() {
    let mut args = env::args();

    if args.len() != 2 {
        panic!("Must have one argument, path to config file!");
    }

    args.next();
    let config_file = args.next().unwrap();

    let settings = match settings::Settings::read(&config_file) {
        Ok(settings) => settings,
        Err(e) => {
            eprintln!("Parsing error: {}", e);
            process::exit(1);
        }
    };

    database::init_database(&settings.database.path);

    run_bot(settings);
}

fn run_bot(settings: settings::Settings) {
    let mut core = Core::new().unwrap();

    let api = telegram_bot::Api::configure(&settings.telegram.api_key)
        .build(core.handle()).unwrap();

    let future = api.stream().for_each(|update| {
        if let UpdateKind::Message(message) = update.kind {
            if let MessageKind::Text {ref data, ..} = message.kind {
                let mut a;
                if let Some(ref nick) = message.from.username {
                    a = nick;
                } else {
                    a = &message.from.first_name;
                }

                println!("<{}>: {}", a, data);

                api.spawn(message.text_reply(
                    format!("Hi, {}! You just wrote '{}'!", a, data)
                ));
            }
        }

        Ok(())
    });

    core.run(future).unwrap();
}

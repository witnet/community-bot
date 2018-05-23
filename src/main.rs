extern crate futures;
extern crate telegram_bot;
extern crate tokio_core;

use std::env;
use std::borrow::Cow;
use futures::Stream;
use tokio_core::reactor::Core;
use telegram_bot::{Api, Message, MessageKind, ParseMode,  UpdateKind, User};
use telegram_bot::prelude::*;
use std::time::{Duration, Instant};

fn handle_text(api: Api, message: Message, data: &String) {
    // Print received text message to stdout.

    match data.as_str() {
        "/faq" => {
            api.spawn(message.text_reply(format!(
                "our faq is located at  '{} {}'",
                &message.from.first_name, data
            )));
        }
        _ => {
            api.spawn(message.text_reply(format!(
                "Hi, {}! You just wrote '{}'",
                &message.from.first_name, data
            )));
        }
    }
}

fn handle_sticker(api: Api, message: Message) {
    api.spawn(message.delete());

}

fn handle_new_members(api: Api, message: Message, new_users: &Vec<User>){
    let fmt_members = new_users.iter().fold(String::new(), |mut acc, user| {
        acc.push_str(
            &user.username.as_ref()
                .map(|username| format!("@{}, ", &username))
                .unwrap_or(format!("{}, ", &user.first_name))
        );
        acc
    });

    let reply_message = format!(
        "**Hi {} welcome to the official Telegram channel for the Witnet English-speaking\
         community.**

Thanks for joining. Please feel free to ask any questions about the project. Your feedback and \
suggestions are also much appreciated!

If you are new to Witnet, here are some links to start with:
+ **1-minute explainer video** → goo.gl/B9Ao5F
+ **Frequently Asked Questions** → https://witnet.io/#/faq
+ Official website → https://witnet.io
+ Tech whitepaper → https://goo.gl/vGrU9s
+ Community Code of Conduct → https://goo.gl/QJWdNd
+ Comunity chatroom at Gitter → https://goo.gl/xfZfBC", fmt_members);

    api.spawn(message.chat.text(reply_message).parse_mode(ParseMode::Markdown).disable_preview());
}

fn handle_left_member(api: Api, users: Vec<t>, new_users: &Vec<User>) {

}

struct TrackUser {
    user: User,
    time: Instant,
    greeted: bool
}

impl TrackUser {
    fn from_user(user: User) {
        TrackUser { user: User, time: Instant::now() , greeted: false}
    }
}

fn main() {
    let mut core = Core::new().unwrap();

    let mut joined = Vec::<TrackUser>::new();

    let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();
    let api = Api::configure(token).build(core.handle()).unwrap();

    // Fetch new updates via long poll method
    let future = api.stream().for_each(|update| {
        if let UpdateKind::Message(message) = update.kind {
            match message.kind {
                MessageKind::Text {ref data, ..} => {
                    handle_text(api.clone(), message.clone(), data)
                }
                MessageKind::Sticker {..} => {
                    handle_sticker(api.clone(), message.clone() )
                }
                MessageKind::NewChatMembers {ref data, ..} => {
                    handle_new_members(api.clone(), message.clone(), data )
                }
                MessageKind::LeftChatMember {ref data, ..} => {
                    joined.push(d)
                }
                _ => (),
            }
        }

        Ok(())
    });

    core.run(future).unwrap();
}

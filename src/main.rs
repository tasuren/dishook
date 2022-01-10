//! dishook

use std::collections::HashMap;

use fltk:: { frame::Frame, button::Button, input::Input, input::MultilineInput };
use fltk:: { app, prelude::*, window::Window };


const TITLE: &str = "dishook";


fn main() {
    let app = app::App::default();
    let mut main = Window::default()
        .with_size(350, 360)
        .center_screen()
        .with_label(TITLE);
    Frame::default()
        .with_size(30, 30)
        .with_pos(80, 20)
        .with_label(TITLE)
        .set_label_size(50);
    let mut url = Input::default()
        .with_size(270, 20)
        .with_pos(60, 70)
        .with_label("URL:");
    url.set_label_size(20);
    let mut avatar = Input::default()
        .with_size(190, 20)
        .with_pos(140, 100)
        .with_label("アバターURL:");
    avatar.set_label_size(20);
    let mut username = Input::default()
        .with_size(210, 20)
        .with_pos(120, 130)
        .with_label("ユーザー名:");
    username.set_label_size(20);
    let mut content_frame = Frame::default()
        .with_size(30, 30)
        .with_pos(70, 150)
        .with_label("メッセージ内容:");
    content_frame.set_label_size(20);
    let content = MultilineInput::default()
        .with_size(330, 130)
        .with_pos(10, 180);
    let client = reqwest::blocking::Client::new();
    Button::default()
        .with_size(330, 30)
        .with_pos(10, 320)
        .with_label("送信")
        .set_callback(
            move |_| {
                let mut data = HashMap::new();
                data.insert("username", username.value());
                data.insert("avatar_url", avatar.value());
                data.insert("content", content.value());
                client.post(url.value()).json(&data).send().unwrap();
            }
        );

    main.end();
    main.show();
    app.run().unwrap();
}
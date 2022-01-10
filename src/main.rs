//! dishook

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
    Button::default()
        .with_size(330, 30)
        .with_pos(10, 320)
        .with_label("送信")
        .set_callback(
            move |_| {
                request(
                    url.value(), username.value(),
                    avatar.value(), content.value()
                );
            }
        );

    main.end();
    main.show();
    app.run().unwrap();
}


/// リクエストを行います。
fn request(
    url: String, username: String, avatar: String, content: String
) {
    let client = reqwest::blocking::Client::new();
    let r = client.post(url).header("Content-Type", "application/json").body(
        format!(
            r#"{{"username": "{}", "avatar_url": "{}", {} }}"#,
            username, avatar,
            if content.starts_with("# ") {
                format!(
                    r#""embeds": [{}]"#, embed(content)
                )
            } else {
                format!(
                    r#""content": "{}""#, escape(content)
                )
            }
        ).replace("\n", "\\n")
    ).send().unwrap();
    println!("Result: {:?}", r.text());
}


/// JSON用にエスケープを行います。
fn escape(text: String) -> String {
    text.replace(r#"""#, r#"\""#)
}


/// 渡されたマークダウンをDiscordで表現可能なものに変換します。
fn decoration(markdown: String) -> String {
    let mut new = String::new();
    let mut space;
    let mut fact_line;
    for line in markdown.lines() {
        fact_line = if line.starts_with("#") {
            match line.find(" ") {
                Some(index) => space = index + 1,
                None => space = 0usize
            }
            format!("**#** {}", &line[space..])
        } else { line.to_string() };
        if line.starts_with("\n") || line.starts_with("**#**") {
            fact_line = format!("\n{}", fact_line);
        }
        new += &(fact_line + "\n");
    }
    new
}


/// 渡された文字列を改行で分ける。
fn separate(text: String) -> (String, String) {
    let i = text.find("\n").unwrap_or_else(|| text.len()-1);
    (text[..i].to_string(), text[i+1..].to_string())
}


/// 渡されたマークダウンを埋め込みに変える。
fn embed(markdown: String) -> String {
    let (mut title, _fields) = separate(markdown);
    title = title[2..].to_string();
    let mut fields: Vec<&str> = _fields.split("\n## ").collect();
    let description = fields[0].to_string();
    fields.remove(0);
    let mut data_fields = Vec::new();
    let (mut inline, mut data_field);
    for field in fields {
        let (mut name, value) = separate(field.to_string());
        inline = if name.starts_with("!") {
            name = name[1..].to_string();
            false
        } else { true };
        data_field = format!(
            r#"{{"name": "{}", "value": "{}", "inline": {}}}"#,
            escape(name), escape(decoration(value)), inline.to_string()
        );
        data_fields.push(data_field);
    }
    format!(
        r#"{{"title": "{}", "description": "{}", "fields": [{}]}}"#,
        escape(title), escape(decoration(description)), data_fields.join(",")
    )
}
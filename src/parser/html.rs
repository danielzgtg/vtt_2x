use crate::{parse_caption_timing, parse_chat_timing, Caption, Chat};
use std::iter::Peekable;
use std::str::Lines;

pub(crate) struct Html<'a> {
    pub(crate) chat: Vec<Chat<'a>>,
    pub(crate) captions: Vec<Caption<'a>>,
}

impl Html<'_> {
    fn with_capacity(chat: usize, captions: usize) -> Self {
        Html {
            chat: Vec::with_capacity(chat),
            captions: Vec::with_capacity(captions),
        }
    }
}

impl<'a> Html<'a> {
    pub(crate) fn from_bypass(captions: Vec<Caption<'a>>) -> Self {
        Html {
            chat: vec![],
            captions,
        }
    }
}

#[inline]
fn parse_object_key<'a>(
    key: &'static str,
    prefix: &'static str,
    suffix: &'static str,
    html: &mut Peekable<Lines<'a>>,
) -> &'a str {
    let line = loop {
        let line = html.next().expect("EOF in middle of JSON object");
        if line.starts_with(key) {
            break line;
        }
    };
    assert!(
        line[key.len()..].starts_with(prefix),
        "Wrong JSON object value prefix"
    );
    assert!(line.ends_with(suffix), "Wrong JSON object value suffix");
    assert!(
        key.len() + prefix.len() + suffix.len() <= line.len(),
        "Overlapping JSON prefix/suffix"
    );
    &line[key.len() + prefix.len()..line.len() - suffix.len()]
}

#[inline]
fn parse_object_footer(footer: &'static str, html: &mut Peekable<Lines>) {
    while html.next().expect("EOF at end of JSON object") != footer {}
}

pub(crate) fn parse_html(html: &str) -> Html {
    let mut result = {
        let mut chat = 0;
        let mut captions = 0;
        for line in html.lines().filter(|x| x.starts_with("window")) {
            match line {
                "window.__data__.transcriptList.push( {" => captions += 1,
                "window.__data__.chatList.push({" => chat += 1,
                _ => {}
            }
        }
        Html::with_capacity(chat, captions)
    };
    let mut html = html.lines();
    assert!(
        matches!(html.next(), Some("<!DOCTYPE html>")),
        "HTML Header"
    );
    {
        let mut count = 2;
        loop {
            let line = html.next().expect("EOF looking for window data start");
            if line == "};" {
                count -= 1;
                if count == 0 {
                    break;
                }
                continue;
            }
            if line.contains("push") {
                panic!("Window data header missing");
            }
        }
    }
    let mut html = html.peekable();
    loop {
        if *html.peek().expect("EOF in captions") != "window.__data__.transcriptList.push( {" {
            break;
        }
        html.next();
        let username = parse_object_key("username", ":  \"", "\" ,", &mut html);
        let start = parse_object_key("ts", ": \"", "\",", &mut html);
        let text = parse_object_key("text", ": \"", "\",", &mut html);
        let end = parse_object_key("endTs", ": \"", "\",", &mut html);
        parse_object_footer("} );", &mut html);
        let start = parse_caption_timing::<true>(
            start.as_bytes().try_into().expect("Parse JSON start time"),
        );
        let end =
            parse_caption_timing::<true>(end.as_bytes().try_into().expect("Parse JSON end time"));
        result
            .captions
            .push(Caption::new(start, end, text, "", Some(username)));
    }
    loop {
        if *html.peek().expect("EOF in chat") != "window.__data__.chatList.push({" {
            break;
        }
        html.next();
        let username = parse_object_key("username", ": \"", "\",", &mut html);
        let time = parse_object_key("time", ": \"", "\",", &mut html);
        let text = parse_object_key("content", ": \"", "\"", &mut html);
        parse_object_footer("})", &mut html);
        let time = parse_chat_timing(time.as_bytes());
        result.chat.push(Chat::new(time, text, username));
    }
    assert!(
        matches!(html.next(), Some("</script>")),
        "Window data footer"
    );
    let mut last = html.next().expect("EOF after window data footer");
    while let Some(line) = html.next() {
        last = line;
    }
    assert!(matches!(last, "</html>"), "HTML footer");
    result
}

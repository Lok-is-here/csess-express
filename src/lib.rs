#[macro_use]
extern crate horrorshow;
extern crate chrono;

use horrorshow::prelude::*;
use horrorshow::helper::doctype;
use chrono::Local;
use regex::Regex;
use wasm_bindgen::prelude::*;

fn make_paragraph<'a>(paragraph: &'a str, index: usize, style: Option<&'a str>) -> Box<dyn Render + 'a> {
    // If a block starts and ends with a HTML tag, render it as is
    if paragraph.starts_with("<") && paragraph.ends_with(">") {
        return box_html! {
            : Raw(paragraph);
        };
    };

    // If a block is divider, render it as <hr>
    if paragraph == "------" {
        return box_html! {
            br;
            hr(style="border: 0.1px solid #ccc; margin: 0 auto; width: 100%;");
            br;
        };
    };

    // Replace markdown-style links with raw HTML-style links
    let re = Regex::new(r"\[(.*?)\]\((.*?)\)").unwrap();
    let result = re.replace_all(paragraph, |caps: &regex::Captures| {
        let link_name = &caps[1];
        let link_url = &caps[2];
        format!("<a style=\"color: #651FFF; text-decoration: none;\" href=\"{}\">{}</a>", link_url, link_name)
    });

    return box_html! {
        @ if index != 0 && !paragraph.starts_with("Dear") { p }
        @ if let Some(style) = style {
            p(style=style) {
                : Raw(result.replace("\n", "<br>"));
            }
        } else {
            p {
                : Raw(result.replace("\n", "<br>"));
            }
        }
    };
}

fn make_font_style() -> Box<dyn Render> {
    box_html! {
        link(rel="stylesheet", href="https://fonts.googleapis.com/css?family=Noto+Sans");
        style(type="text/css") {
            : "p, li, td { font-family: 'Noto Sans', sans-serif; font-weight: 400; font-size: 15px; text-rendering: optimizeLegibility; -moz-osx-font-smoothing: grayscale; font-smoothing: antialiased; -webkit-font-smoothing: antialiased; text-shadow: rgba(0, 0, 0, .01) 0 0 1px; }";
        }
    }
}

fn inline(extra: Option<&str>) -> String {
    let default = "margin: 0; padding: 0; width: 100% !important; border-collapse: collapse !important;";
    match extra {
        Some(extra) => format!("{} {}", default, extra),
        None => default.into(),
    }
}

macro_rules! inline {
    () => { inline(None) };
    ($x: expr) => { inline(Some($x)) };
}

fn make_table<'a>(body: &'a str, disclaimer: Option<&'a str>) -> Box<dyn Render + 'a> {
    box_html! {
        table(style=inline!("background-color: #f0f0f0;")) {
            tbody {
                @ if let Some(disclaimer) = disclaimer {
                    tr {
                        td(style=inline!(
                            "padding: 16px 10% !important; background-color: #50235f; color: #fff; text-align: center;")) {
                            @ for (index, paragraph) in disclaimer.split("\n\n").enumerate() {
                                : make_paragraph(paragraph.trim(), index, Some("font-size: 12px; font-weight: bold;"));
                            }
                        }
                    }
                }
                tr {
                    td(style=inline!(), align="center") {
                        table(style=inline!("margin: 5% 0 !important; max-width: 800px !important;")) {
                            tbody {
                                tr {
                                    td(style=inline!(
                                        "background-color: #210d41; border-top: 6px solid rgb(107, 65, 244); border-bottom: 6px solid rgb(35, 125, 250);")) {
                                        img(src="https://csess.su.hkust.edu.hk/email/mail_header_2425.png", width="100%");
                                    }
                                }
                                tr {
                                    td(style=inline!(
                                        "padding: 48px 56px !important; color: #212121; background-color: #fff; text-align: left;")) {
                                        @ for (index, paragraph) in body.split("\n\n").enumerate() {
                                            : make_paragraph(paragraph.trim(), index, None);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                tr {
                    td(style=inline!("padding: 32px 10% !important; background-color: #50235f; color: #fff; text-align: center;")) {
                        p(style="font-size: 12px; font-weight: bold;") {
                            : "The Computer Science and Engineering Students’ Society, Hong Kong University of Science and Technology Students’ Union"; br;
                            : "香港科技大學學生會計算機科學及工程學系學生會";
                            @ if disclaimer.is_none() {
                                br; br;
                                : "You received this email because you are a member of CSESS."; : " ";
                                a(style="color: #ccc; text-decoration: none;", href="http://lists.ust.hk/sympa/home", target="_blank", rel="noopener") {
                                    : "Unsubscribe";
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[wasm_bindgen]
pub fn make_html(raw: &str) -> String {
    // Check if the email headers exist
    let headers_pattern = Regex::new(r"^(.*: .*\n)+\n======\n\n").unwrap();
    if !headers_pattern.is_match(&raw) {
        return "! invalid headers".into();
    }

    // Check if the disclaimer exists
    let disclaimer_pattern = Regex::new(r"^(.*: .*\n)+\n======\n\n(.|\n)*\n\n======\n").unwrap();
    let disclaimer_exist = disclaimer_pattern.is_match(&raw);

    let mut split = raw.splitn(if disclaimer_exist { 3 } else { 2 }, "\n======\n");
    let headers = split.next().unwrap();
    let disclaimer = if disclaimer_exist { split.next() } else { None };
    let body = split.next().unwrap();

    let date = Local::now().format("Date: %a, %d %b %Y %H:%M:%S %z");
    let content_type = "Content-Type: text/html; charset=utf-8";
    let mime_version = "MIME-Version: 1.0";
    let headers = format!("{}\n{}\n{}\n{}\n\n", headers.trim(), date, content_type, mime_version);

    format!("{}", html! {
        : Raw(headers.to_owned());
        : doctype::HTML;
        html {
            head {
                meta(http-equiv="Content-Type", content="text/html; charset=utf-8");
                meta(name="viewport", content="width=device-width, initial-scale=1.0");
                title: "CSESS Express";
                : make_font_style();
            }
            body(style=inline!()) {
                : make_table(body, disclaimer);
            }
        }
    })
}

#[wasm_bindgen]
pub fn make_div(raw: &str) -> String {
    // Check if the email headers exist
    let headers_pattern = Regex::new(r"^(.*: .*\n)+\n======\n\n").unwrap();
    if !headers_pattern.is_match(&raw) {
        return "".into();
    }

    // Check if the disclaimer exists
    let disclaimer_pattern = Regex::new(r"^(.*: .*\n)+\n======\n\n(.|\n)*\n\n======\n").unwrap();
    let disclaimer_exist = disclaimer_pattern.is_match(&raw);

    let mut split = raw.splitn(if disclaimer_exist { 3 } else { 2 }, "\n======\n");
    let _headers = split.next();    // ignore headers
    let disclaimer = if disclaimer_exist { split.next() } else { None };
    let body = split.next().unwrap();

    format!("{}", box_html! {
        : make_font_style();
        : make_table(body, disclaimer);
    })
}

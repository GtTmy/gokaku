use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use reqwest::header;

#[derive(Debug)]
pub struct CharElement {
    pub character: Option<char>,
    pub strokes: u32,
}

impl CharElement {
    pub fn new(character: char, strokes: u32) -> CharElement {
        CharElement {
            character: Some(character),
            strokes,
        }
    }

    pub fn new_dummy() -> CharElement {
        CharElement {
            character: None,
            strokes: 1,
        }
    }
}

pub struct KanjiApi {
    client: reqwest::blocking::Client,
}

pub struct KanjiAliveApi {
    client: reqwest::blocking::Client,
}

pub trait Strokes {
    fn get_strokes(&self, kanji: char) -> u32;
}

impl KanjiApi {
    pub fn new() -> KanjiApi {
        let client = reqwest::blocking::Client::builder().build().unwrap();
        KanjiApi { client }
    }
}

impl Default for KanjiApi {
    fn default() -> Self {
        Self::new()
    }
}

impl Strokes for KanjiApi {
    fn get_strokes(self: &KanjiApi, kanji: char) -> u32 {
        let mut apiurl = String::from("https://apino.yukiyuriweb.com/api/kanji/v1/chars/");
        let encoded_kanji =
            percent_encoding::utf8_percent_encode(&kanji.to_string()[..], NON_ALPHANUMERIC)
                .to_string();
        apiurl += &encoded_kanji[..];
        let res = self.client.get(apiurl).send().unwrap();

        let text = res.text().unwrap();
        let parse: serde_json::Value = serde_json::from_str(&text[..]).unwrap();

        if let serde_json::Value::Array(results) = parse {
            if let serde_json::Value::Object(info) = &results[0] {
                if let serde_json::Value::String(stroke_s) = &info["stroke"] {
                    let stroke: u32 = stroke_s.parse().unwrap();
                    stroke
                } else {
                    panic!("error")
                }
            } else {
                panic!("error")
            }
        } else {
            panic!("error")
        }
    }
}

impl KanjiAliveApi {
    #[allow(dead_code)]
    fn new(api_host: &str, api_key: &str) -> KanjiAliveApi {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "x-rapidapi-host",
            header::HeaderValue::from_str(api_host).unwrap(),
        );
        headers.insert(
            "x-rapidapi-key",
            header::HeaderValue::from_str(api_key).unwrap(),
        );
        let client = reqwest::blocking::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        KanjiAliveApi { client }
    }
}

impl Strokes for KanjiAliveApi {
    fn get_strokes(&self, kanji: char) -> u32 {
        let mut apiurl = String::from("https://kanjialive-api.p.rapidapi.com/api/public/kanji/");
        let encoded_kanji =
            utf8_percent_encode(&kanji.to_string()[..], NON_ALPHANUMERIC).to_string();
        apiurl += &encoded_kanji[..];
        let res = self.client.get(apiurl).send().unwrap();

        let text = res.text().unwrap();
        let parse: serde_json::Value = serde_json::from_str(&text[..]).unwrap();
        if let serde_json::Value::Object(x) = parse {
            println!("{}", kanji);
            let obj = &x.get_key_value("kanji");
            if let Some(x2) = obj {
                if let serde_json::Value::Object(y) = &x2.1 {
                    if let serde_json::Value::Object(z) = &y["strokes"] {
                        if let serde_json::Value::Number(p) = &z["count"] {
                            p.as_u64().unwrap() as u32
                        } else {
                            panic!("key not found");
                        }
                    } else {
                        panic!("key not found");
                    }
                } else {
                    panic!("key not found");
                }
            } else {
                0
            }
        } else {
            panic!("key not found")
        }
    }
}

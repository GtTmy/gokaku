use reqwest;
use reqwest::header;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};


#[derive(Debug)]
struct CharElement {
    character: Option<char>,
    strokes: u32,
}

impl CharElement {
    fn new(character: char, strokes: u32) -> CharElement{
        CharElement {
            character: Some(character),
            strokes
        }
    }

    fn new_dummy() -> CharElement {
        CharElement {
            character: None,
            strokes: 1
        }
    }
}

struct Result {
    tenkaku: u32,
    gaikaku: u32,
    jinkaku: u32,
    dikaku: u32,
    soukaku: u32,
}

impl Result {
    fn new(tenkaku: u32, gaikaku: u32, jinkaku: u32, dikaku: u32, soukaku: u32) -> Result {
        Result {
            tenkaku,
            gaikaku,
            jinkaku,
            dikaku,
            soukaku
        }
    }
}

struct KanjiApi {
    client: reqwest::blocking::Client
}

struct KanjiAliveApi {
    client: reqwest::blocking::Client
}

trait Strokes {
    fn get_strokes(self: &Self, kanji: char) -> u32;
}

impl KanjiApi {
    fn new() -> KanjiApi {
        let client = reqwest::blocking::Client::builder()
            .build()
            .unwrap();
        KanjiApi {
            client
        }
    }
}

impl Strokes for KanjiApi {
    fn get_strokes(self: &KanjiApi, kanji: char) -> u32 {
        let mut apiurl = String::from("https://apino.yukiyuriweb.com/api/kanji/v1/chars/");
        let encoded_kanji = percent_encoding::utf8_percent_encode(&kanji.to_string()[..], NON_ALPHANUMERIC).to_string();
        apiurl = apiurl + &encoded_kanji[..];
        let res = self.client.get(apiurl)
            .send()
            .unwrap();
        
        let text = res.text().unwrap();
        let parse: serde_json::Value = serde_json::from_str(&text[..]).unwrap();
    
        if let serde_json::Value::Array(results) = parse {
            if let serde_json::Value::Object(info) = &results[0] {
                if let serde_json::Value::String(stroke_s) = &info["stroke"] {
                    let stroke: u32 = stroke_s.parse().unwrap();
                    return stroke;
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
    fn new(api_host: &str, api_key: &str) -> KanjiAliveApi {
        let mut headers = header::HeaderMap::new();
        headers.insert("x-rapidapi-host", header::HeaderValue::from_str(api_host).unwrap());
        headers.insert("x-rapidapi-key", header::HeaderValue::from_str(api_key).unwrap());
        let client = reqwest::blocking::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        
        KanjiAliveApi {
            client
        }
    }
}

impl Strokes for KanjiAliveApi {
    fn get_strokes(self: &Self, kanji: char) -> u32 {
        let mut apiurl = String::from("https://kanjialive-api.p.rapidapi.com/api/public/kanji/");
        let encoded_kanji = utf8_percent_encode(&kanji.to_string()[..], NON_ALPHANUMERIC).to_string();
        apiurl = apiurl + &encoded_kanji[..];
        let res = self.client.get(apiurl)
            .send()
            .unwrap();
        
        let text = res.text().unwrap();
        let parse: serde_json::Value = serde_json::from_str(&text[..]).unwrap();
        if let serde_json::Value::Object(x) = parse {
            println!("{}", kanji);
            let obj = &x.get_key_value("kanji");
            if let Some(x2) = obj {
                if let serde_json::Value::Object(y) = &x2.1 {
                    if let serde_json::Value::Object(z) = &y["strokes"] {
                        if let serde_json::Value::Number(p) = &z["count"] {
                            return p.as_u64().unwrap() as u32;
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

fn calc_jikaku(last_name: &str, first_name: &str) -> Result {

    let last_name_length = last_name.chars().count();
    let first_name_length = first_name.chars().count();

    let mut last_name_items = Vec::new();
    let mut first_name_items = Vec::new();

    let api = KanjiApi::new();

    if first_name_length > last_name_length {
        for _ in 0..first_name_length-last_name_length {
            last_name_items.push(CharElement::new_dummy());
        }
    }
    for c in last_name.chars() {
        let count = api.get_strokes(c);
        last_name_items.push(CharElement::new(c, count));
    }
    for c in first_name.chars() {
        let count = api.get_strokes(c);
        first_name_items.push(CharElement::new(c, count));
    }
    if last_name_length > first_name_length {
        for _ in 0..last_name_length-first_name_length {
            first_name_items.push(CharElement::new_dummy());
        }
    }
    
    dbg!(last_name_items);
    dbg!(first_name_items);

    Result::new(0, 0, 0, 0, 0)
}

fn main() {
    let last_name = String::from("山田");
    let first_name = String::from("花子");
 
    calc_jikaku(&last_name[..], &first_name[..]);
}

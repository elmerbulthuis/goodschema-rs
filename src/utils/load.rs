use std::fs::File;

use serde_json::Value;
use url::Url;

pub fn load_json(url: &Url) -> Result<Value, &'static str> {
    match url.scheme() {
        "file" => {
            let path = url.path();
            let reader = File::open(path).or(Err("error reading file"))?;

            let value =
                serde_json::from_reader(reader).or(Err("error deserializing file content"))?;
            let value = value;

            Ok(value)
        }
        _ => Err("not supported"),
    }
}

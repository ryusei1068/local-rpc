use async_std::io::{ReadExt, WriteExt};
use async_std::os::unix::net::{UnixListener, UnixStream};
use async_std::prelude::*;
use async_std::task;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct Request {
    method: String,
    params: Vec<String>,
    params_types: Vec<String>,
    id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    result: String,
    result_type: String,
    id: i64,
}

struct Aid {
    request: Request,
}

impl Aid {
    fn new(req: Request) -> Aid {
        Aid { request: req }
    }

    fn floor(&self, x: f64) -> f64 {
        x.floor()
    }

    fn nroot(&self, n: f64, x: f64) -> f64 {
        x.powf(1_f64 / n as f64)
    }

    fn reverse(&self, s: String) -> String {
        s.chars().rev().collect::<String>()
    }

    fn valid_anagram(&self, s: String, t: String) -> bool {
        self.char_counts(&s) == self.char_counts(&t)
    }

    fn char_counts(&self, s: &String) -> HashMap<char, u32> {
        let mut map = HashMap::new();
        for c in s.chars() {
            map.entry(c).and_modify(|num| *num += 1).or_insert(1);
        }

        map
    }

    fn sort(&self, mut s: Vec<String>) -> Vec<String> {
        s.sort();
        s
    }

    fn call_optimum_method(&self) -> Response {
        let method = &self.request.method;
        let params = &self.request.params;
        let identify = self.request.id.clone();

        if *method == "floor".to_string() {
            return create_response(
                self.floor(params[0].parse().unwrap()).to_string(),
                "f64".to_string(),
                identify,
            );
        } else if *method == "nroot".to_string() {
            let p1 = params[0].parse().unwrap();
            let p2 = params[1].parse().unwrap();

            return create_response(self.nroot(p1, p2).to_string(), "f64".to_string(), identify);
        } else if *method == "reverse".to_string() {
            return create_response(
                self.reverse(params[0].parse().unwrap()),
                "String".to_string(),
                identify,
            );
        } else if *method == "validAnagram".to_string() {
            let p1 = params[0].parse().unwrap();
            let p2 = params[1].parse().unwrap();

            return create_response(
                self.valid_anagram(p1, p2).to_string(),
                "bool".to_string(),
                identify,
            );
        } else if *method == "sort" {
            let p_clone = params.clone();
            return create_response(self.sort(p_clone).join(" "), "String".to_string(), identify);
        }

        create_response("error".to_string(), "invalid method".to_string(), identify)
    }
}

fn create_response(res: String, res_type: String, identify: i64) -> Response {
    Response {
        result: res,
        result_type: res_type,
        id: identify,
    }
}

async fn serve(mut stream: UnixStream) -> std::io::Result<()> {
    let mut request = String::new();
    stream.read_to_string(&mut request).await?;

    let deserialized: Request = serde_json::from_str(&request).unwrap();

    let aid = Aid::new(deserialized);

    let serialized = serde_json::to_string(&aid.call_optimum_method()).unwrap();
    let buf: &[u8] = serialized.as_str().as_bytes();
    stream.write(buf).await?;
    stream.flush().await?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    async_std::task::block_on(async {
        let socket_path = "/socket_file";

        let path = Path::new(&socket_path);

        if path.exists() {
            fs::remove_file(path).expect("File delete failed");
        }

        let listener = UnixListener::bind(socket_path).await?;
        let mut incoming = listener.incoming();

        while let Some(stream) = incoming.next().await {
            let stream = stream?;
            task::spawn(async {
                log_error(serve(stream).await);
            });
        }
        Ok(())
    })
}

fn log_error(result: std::io::Result<()>) {
    if let Err(error) = result {
        eprintln!("Error: {}", error);
    }
}

#[cfg(test)]
mod tests {
    use crate::Aid;
    use crate::Request;

    #[test]
    fn floor_test() {
        let reqeust = Request {
            method: "floor".to_string(),
            params: vec!["3.12".to_string()],
            params_types: vec!["float".to_string()],
            id: 1,
        };
        let aid = Aid { request: reqeust };
        assert_eq!(aid.floor(4.0_f64), 4.0);
    }
}

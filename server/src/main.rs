use async_std::io::{ReadExt, WriteExt};
use async_std::os::unix::net::UnixListener;
use async_std::prelude::*;
use async_std::task;
use fake::faker::name::raw::*;
use fake::locales::*;
use fake::Fake;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct Request {
    method: String,
    params: Vec<f64>,
    params_types: Vec<String>,
    id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    result: String,
    result_type: String,
    id: i64,
}

fn floor(x: f64) -> f64 {
    x.floor()
}

fn nroot(n: f64, x: f64) -> f64 {
    x.powf(1_f64 / n as f64)
}

fn reverse(s: String) -> String {
    s.chars().rev().collect::<String>()
}

fn valid_anagram(s: String, t: String) -> bool {
    char_counts(&s) == char_counts(&t)
}

fn char_counts(s: &String) -> HashMap<char, u32> {
    let mut map = HashMap::new();
    for c in s.chars() {
        map.entry(c).and_modify(|num| *num += 1).or_insert(1);
    }

    map
}

fn sort(mut s: Vec<String>) -> Vec<String> {
    s.sort();
    s
}

async fn run() -> std::io::Result<()> {
    let socket_path = "/socket_file";

    let path = Path::new(&socket_path);

    if path.exists() {
        fs::remove_file(path).expect("File delete failed");
    }

    let listener = UnixListener::bind(socket_path).await?;
    let mut incoming = listener.incoming();

    while let Some(stream) = incoming.next().await {
        let mut stream = stream?;
        println!("connection from {:?}", stream.local_addr().unwrap());

        let mut request = String::new();
        stream.read_to_string(&mut request).await?;

        let deserialized: Request = serde_json::from_str(&request).unwrap();

        println!("We received this message: {:?}\nReplying...", deserialized);

        let name: String = Name(EN).fake();
        let mut greeting: String = "Hello, ".to_string();
        greeting.push_str(&name);
        let buf: &[u8] = greeting.as_str().as_bytes();

        stream.write_all(buf).await?;
    }

    Ok(())
}

fn main() {
    let result = task::block_on(run());
    println!("{:?}", result);
}

#[test]
fn test_floor() {
    assert_eq!(-11_f64, floor(-10.7));
    assert_eq!(10_f64, floor(10.7));
}

#[test]
fn test_nroot() {
    assert_eq!(2_f64, nroot(3.0, 8.0));
    assert_eq!(4_f64, nroot(2.0, 16.0));
}

#[test]
fn test_reverse() {
    assert_eq!("olleh", reverse("hello".to_string()));
}

#[test]
fn test_valid_anagram() {
    assert_eq!(false, valid_anagram("rat".to_string(), "car".to_string()));
    assert_eq!(
        true,
        valid_anagram("anagram".to_string(), "nagaram".to_string())
    );
}

#[test]
fn test_sort() {
    let s = vec!["world".to_string(), "hello".to_string()];
    assert_eq!(vec!["hello".to_string(), "world".to_string()], sort(s));
}

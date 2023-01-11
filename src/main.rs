use std::net::{TcpStream};
use std::io::{BufReader, BufRead};
use std::io::Write;
use clap::{Arg, App};
use json;
use base64 as b64;

static DEFAULT_PORT : &str = "10304";

fn main() {
    let matches = App::new("Clipboard Client")
        .version("0.1")
        .author("mirmik")
        .about("Clipboard Client")
        .arg(Arg::with_name("port")
            .short("p")
            .long("port")
            .value_name("PORT")
            .help("Sets a custom port")
            .takes_value(true))
        .arg(Arg::with_name("host")
            .short("h")
            .long("host")
            .value_name("HOST")
            .help("Sets a custom host")
            .takes_value(true))
        .arg(Arg::with_name("upload")
            .short("u")
            .long("upload")
            .value_name("UPLOAD")
            .help("Uploads a file")
            .takes_value(true))
        .get_matches();

    let host_string : String = match matches.value_of("host") {
        Some(host) => host.to_string(),
        None => { 
            // try to read env var
            match std::env::var("CLIPBOARD_HOST") {
                Ok(v) => v,
                Err(_) => "127.0.0.1".to_string(),
            }
        },
    };
    let host = host_string.as_str();

    let port = matches.value_of("port").unwrap_or(DEFAULT_PORT);
    let up = matches.value_of("upload");

    match up {
        Some(_) => upload(host, port, up.unwrap()),
        None => download(host, port),
    }
}

fn upload(host: &str, port: &str, text : &str)
{
    let mut stream = TcpStream::connect(format!("{}:{}", host, port)).unwrap();

    let mut json = json::JsonValue::new_object();
    json["command"] = "clipboard_upload_base64".into();
    json["data"] = b64::encode(text).into();
    let json_str = json.dump();
    let json_str_nl = format!("{}\r\n", json_str);

    stream.write(json_str_nl.as_bytes()).unwrap();
}

fn download(host: &str, port: &str)
{
    let mut stream = TcpStream::connect(format!("{}:{}", host, port)).unwrap();
    println!("Connected to {}:{}", host, port);

    let mut json = json::JsonValue::new_object();
    json["command"] = "clipboard_download_base64".into();
    let json_str = json.dump();
    let json_str_nl = format!("{}\r\n", json_str);

    stream.write(json_str_nl.as_bytes()).unwrap();

    let mut reader = BufReader::new(stream);
    let mut line = String::new();

    reader.read_line(&mut line).unwrap();
    let json = json::parse(&line).unwrap();
    let data = match json["data"].as_str() {
        Some(data) => data,
        None => panic!("No data"),
    };
    let data = b64::decode(data).unwrap();

    println!("{}", String::from_utf8(data).unwrap());
}

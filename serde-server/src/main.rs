use serde::{Deserialize, Serialize};
use std::fmt;
use std::io::{stdin, Error};
use std::net::{TcpListener, TcpStream};
use std::{env, str, thread};

#[derive(Serialize, Deserialize, Debug)]
struct Point3D {
    x: u32,
    y: u32,
    z: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Value {
    data: f64,
}

impl From<&Point3D> for Value {
    fn from(point: &Point3D) -> Self {
        let value = point.x.pow(2) + point.y.pow(2) + point.z.pow(2);

        Self {
            data: f64::from(value).sqrt(),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}

fn handle_client(stream: TcpStream) -> Result<(), Error> {
    println!("Incoming connection from: {}", stream.peer_addr()?);

    let points = serde_json::Deserializer::from_reader(&stream).into_iter::<Point3D>();

    for point in points {
        let value = Value::from(&point?);

        serde_json::to_writer(&stream, &value)?;
    }

    Ok(())
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Please provide --client or --server as argument");
        std::process::exit(1);
    }
    if args[1] == "--server" {
        let listener = TcpListener::bind("0.0.0.0:8888").expect("Could not bind");
        for stream in listener.incoming() {
            match stream {
                Err(e) => eprintln!("failed: {}", e),
                Ok(stream) => {
                    // TODO: drop the thread on connection close
                    thread::spawn(move || {
                        handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
                    });
                }
            }
        }
    } else if args[1] == "--client" {
        let stream = TcpStream::connect("127.0.0.1:8888").expect("Could not connect to server");

        println!("Please provide a 3D point as three comma separated integers");

        loop {
            let mut input = String::new();
            stdin()
                .read_line(&mut input)
                .expect("Failed to read from stdin");
            let parts: Vec<&str> = input.trim_matches('\n').split(',').collect();
            let point = Point3D {
                x: parts[0].parse().unwrap(),
                y: parts[1].parse().unwrap(),
                z: parts[2].parse().unwrap(),
            };

            serde_json::to_writer(&stream, &point).unwrap();

            let value = serde_json::Deserializer::from_reader(&stream)
                .into_iter::<Value>()
                .next()
                .unwrap()
                .unwrap();

            println!("Response from server {}", value);
        }
    }
}

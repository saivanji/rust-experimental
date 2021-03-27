use crate::Engine;
use anyhow::Result;
use std::net::{TcpListener, TcpStream, ToSocketAddrs};

pub struct Server<E: Engine> {
    engine: E,
}

impl<E: Engine> Server<E> {
    pub fn new(engine: E) -> Self {
        Server { engine }
    }

    pub fn run<A: ToSocketAddrs>(mut self, addr: A) -> Result<()> {
        let listener = TcpListener::bind(addr)?;

        for stream in listener.incoming() {
            println!("New connection");
        }

        Ok(())
    }
}

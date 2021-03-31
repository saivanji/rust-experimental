use crate::{Action, Engine, Reply};
use anyhow::Result;
use log::error;
use serde_json::{to_writer, Deserializer};
use std::io::{BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};

pub struct Server<E: Engine> {
    engine: E,
}

impl<E: Engine> Server<E> {
    pub fn new(engine: E) -> Self {
        Server { engine }
    }

    pub fn run<A: ToSocketAddrs>(&mut self, addr: A) -> Result<()> {
        let listener = TcpListener::bind(addr)?;

        for request in listener.incoming() {
            match request {
                Ok(request) => {
                    let tcp_reader = BufReader::new(&request);
                    // TODO: put inside of "handle_action"
                    let action = Deserializer::from_reader(tcp_reader)
                        .into_iter::<Action>()
                        .next();

                    match action {
                        Some(action) => {
                            // TODO: if "action" has Err then it should be related to that
                            // condition
                            if let Err(e) = self.handle_action(action?, &request) {
                                error!("Error on serving client: {}", e);
                            }
                        }
                        None => error!("Empty request"),
                    }
                }
                Err(e) => error!("Connection failed: {}", e),
            }
        }

        Ok(())
    }

    fn handle_action(&mut self, action: Action, request: &TcpStream) -> Result<()> {
        let reply = match action {
            Action::Get { key } => {
                let format_err = |e| Err(format!("{}", e));

                Reply::Get(self.engine.get(key).or_else(format_err))
            }
            Action::Set { key, value } => {
                let format_err = |e| Err(format!("{}", e));

                Reply::Set(self.engine.set(key, value).or_else(format_err))
            }
            Action::Remove { key } => {
                let format_err = |e| Err(format!("{}", e));

                Reply::Remove(self.engine.remove(key).or_else(format_err))
            }
        };

        self.respond(request, reply)
    }

    fn respond(&self, request: &TcpStream, reply: Reply) -> Result<()> {
        let mut tcp_writer = BufWriter::new(request);

        to_writer(&mut tcp_writer, &reply)?;
        tcp_writer.flush()?;

        Ok(())
    }
}

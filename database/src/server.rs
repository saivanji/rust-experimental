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
        self.run_with_callback(addr, || {})
    }

    pub fn run_with_callback<A: ToSocketAddrs, C: FnOnce() -> ()>(
        &mut self,
        addr: A,
        callback: C,
    ) -> Result<()> {
        let listener = TcpListener::bind(addr)?;

        callback();

        for request in listener.incoming() {
            match request {
                Ok(request) => {
                    if let Err(e) = self.handle_request(&request) {
                        error!("Error on serving client: {}", e);
                    }
                }
                Err(e) => error!("Connection failed: {}", e),
            }
        }

        Ok(())
    }

    fn handle_request(&mut self, request: &TcpStream) -> Result<()> {
        let tcp_reader = BufReader::new(request);
        let action_reader = Deserializer::from_reader(tcp_reader).into_iter::<Action>();

        for action in action_reader {
            let action = action?;
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

            self.respond(request, reply)?;
        }

        Ok(())
    }

    fn respond(&self, request: &TcpStream, reply: Reply) -> Result<()> {
        let mut tcp_writer = BufWriter::new(request);

        to_writer(&mut tcp_writer, &reply)?;
        tcp_writer.flush()?;

        Ok(())
    }
}

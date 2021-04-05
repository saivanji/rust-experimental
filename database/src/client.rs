use crate::{Action, Reply};
use anyhow::{anyhow, Result};
use serde::Deserialize;
use serde_json::de::IoRead;
use serde_json::{to_writer, Deserializer};
use std::io::{BufReader, BufWriter, Write};
use std::net::{TcpStream, ToSocketAddrs};

const INVALID_RESPONSE: &str = "Invalid server response";

pub struct Client {
    deserializer: Deserializer<IoRead<BufReader<TcpStream>>>,
    writer: BufWriter<TcpStream>,
}

impl Client {
    pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let reader = TcpStream::connect(addr)?;
        let writer = reader.try_clone()?;

        Ok(Client {
            deserializer: Deserializer::from_reader(BufReader::new(reader)),
            writer: BufWriter::new(writer),
        })
    }

    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        self.send_action(Action::Get { key })?;

        match self.read_reply()? {
            Reply::Get(reply) => reply.or_else(|err| Err(anyhow!(err))),
            _ => Err(anyhow!(INVALID_RESPONSE)),
        }
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        self.send_action(Action::Set { key, value })?;

        match self.read_reply()? {
            Reply::Set(reply) => reply.or_else(|err| Err(anyhow!(err))),
            _ => Err(anyhow!(INVALID_RESPONSE)),
        }
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        self.send_action(Action::Remove { key })?;

        match self.read_reply()? {
            Reply::Remove(reply) => reply.or_else(|err| Err(anyhow!(err))),
            _ => Err(anyhow!(INVALID_RESPONSE)),
        }
    }

    fn send_action(&mut self, action: Action) -> Result<()> {
        to_writer(&mut self.writer, &action)?;
        self.writer.flush()?;

        Ok(())
    }

    fn read_reply(&mut self) -> Result<Reply> {
        let reply = Reply::deserialize(&mut self.deserializer)?;

        Ok(reply)
    }
}

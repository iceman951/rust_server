use std::net::TcpListener;
use std::io::Read;
use std::str;
use crate::http::Result;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(&self) -> Result<()> {
        print!("Listening on {} \n", self.addr);

        let listener = TcpListener::bind(&self.addr)?;

        for stream in listener.incoming() {
            println!("Connection established!");
            let mut stream = stream?;
            let mut buf: [u8; 1024] = [0; 1024];
            stream.read(&mut buf)?;
            // if let Ok(request) = str::from_utf8(&buf) {
            //     print!("Request: {}", request);
            // }
            let request = str::from_utf8(&buf)?;

            println!("Request: {}", request);

        }

        Ok(())

    }
}
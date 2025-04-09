use std::net::TcpListener;
use std::io::Read;
use std::str;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(&self) -> std::io::Result<()> {
        print!("Listening on {} \n", self.addr);

        let listener = TcpListener::bind(&self.addr)?;

        for stream in listener.incoming() {
            println!("Connection established!");
            let mut stream = stream?;
            let mut buf: [u8; 1024] = [0; 1024];
            stream.read(&mut buf)?;
            if let Ok(request) = str::from_utf8(&buf) {
                print!("Request: {}", request);
            }

        }

        Ok(())

    }
}
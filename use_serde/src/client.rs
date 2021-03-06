use serde::{ Deserialize, Serialize };
use std::io::{self, BufReader, BufRead, Write };
use std::net::TcpStream;

#[derive( Debug, Serialize, Deserialize )]
struct Point3D {
    x: u32,
    y: u32,
    z: u32,
}

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect( "127.0.0.1:9090" )?;

    loop {
        let mut input = String::new();
        let mut buffer: Vec<u8> = Vec::new();

        io::stdin().read_line( &mut input )
                    .expect( "Failed to read from stdin" );
        
        let x: &[_] = &['\r', '\n'];
        let parts: Vec<&str> = input.trim_matches( x ).split( ',' ).collect();

        let point = Point3D {
            x: parts[0].parse().unwrap(),
            y: parts[1].parse().unwrap(),
            z: parts[2].parse().unwrap(),
        };

        stream.write_all( serde_json::to_string( &point ).unwrap().as_bytes() )
                .expect( "Failed to write" );
        stream.write_all( b"\n" )?;

        let mut reader = BufReader::new( &stream );
        reader.read_until( b'\n', &mut buffer )?;
        let input = std::str::from_utf8( &buffer ).unwrap();
        if input == "" {
            eprintln!( "Empty response" );
        }

        print!( "Response: {}", input );
    }
}
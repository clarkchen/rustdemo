use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;

#[test]
fn test_main() {
    match TcpStream::connect("localhost:3333") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");

            for i in 0..5{
                let msg =b"Hello!";   // => "The number is 1"


                stream.write(msg).unwrap();
                println!("Sent Hello {}, awaiting reply...", i);

                let mut data = [0 as u8; 6]; // using 6 byte buffer
                match stream.read_exact(&mut data) {
                    Ok(_) => {
                        if &data == msg {
                            println!("Reply is ok!");
                        }
                        // } else {
                        let text = from_utf8(&data).unwrap();
                        println!("reply data is {}", text);
                        // }
                    },
                    Err(e) => {
                        println!("Failed to receive data: {}", e);
                    }
                }
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }

    }
    println!("Terminated.");
}
use std::io::prelude::*;
use std::net::TcpStream;
use std::io;
use std::thread;
use std::sync::{Arc, Mutex};


fn make_connection(ip: &str, port: &str) -> io::Result<TcpStream>{

    let addr = format!("{}:{}", ip, port);

    match TcpStream::connect(addr) {
        Ok(stream) => {
            println!("connected to host: {} on port {}\n", ip, port);
            Ok(stream)
        }
        Err(e) => {
            println!("error connecting to host: {} on port {}", ip, port);
            Err(e)
    
        }
    }
}


fn main(){

    let mut ip = String::new();
    let mut port = String::new();
    

    println!("enter IP to connect to:\n");
    io::stdin()
        .read_line(&mut ip)
        .expect("Failed to read line");

    println!("enter port to connect via:\n");
    io::stdin()
        .read_line(&mut port)
        .expect("Failed to read line"); 


    println!("Type `send 'text'` to send a message");
    println!("Type `quit` to close the connection");

    ip = ip.trim().to_string();
    port = port.trim().to_string();

    if let Ok(mut stream) = make_connection(&ip, &port){
        
        let mut read_stream = stream.try_clone().expect("Failed to clone stream");

        thread::spawn(move || {
         loop {
        let mut buffer = [0u8; 128];
                match read_stream.read(&mut buffer) {
                Ok(n) => {
                    println!("received {} bytes", n);
                    let received = &buffer[..n];
                    println!("Raw bytes: {:?}", received);
                    if let Ok(text) = std::str::from_utf8(received) {
                        println!("As string: {}", text);
                    }
                }
                Err(e) => {
                    println!("error reading/receiving bytes")
                }
            }
        }
        });
        
        loop {
            let mut user_command = String::new();

            println!("> ");
            io::stdout().flush().unwrap();


            io::stdin()
            .read_line(&mut user_command)
            .expect("Failed to read line");


            user_command = user_command.trim().to_string();




            if user_command == "quit" {
                println!("quitting");
                break;

            } else if user_command.starts_with("send ") && user_command.len() > 5 {
                let message = &user_command[5..];

                match stream.write(message.as_bytes()) {
                    Ok(n) => println!("Sent {} bytes", n),
                    Err(e) => println!("Error sending message: {}", e),
                }



            } else if user_command.starts_with("send") {
                print!("You must provide a message to send. Usage: send 'message'\n");
                io::stdout().flush().unwrap();
            } else {
                 print!("malformed command\n");
                 io::stdout().flush().unwrap();
             }
            

        }

    }
}

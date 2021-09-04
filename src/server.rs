use std::io::Read;
use std::net::TcpListener;
use std::convert::TryFrom;
use crate::http::{Request,Response,StatusCode, ParseErrror};
pub trait  Handler {
    fn handle_request(&self, request: &Request) -> Response;
    fn handle_bad_request(&self, e: &ParseErrror) -> Response{
        println!("Failed to parse a request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}
pub struct Server{
    addr: String,
}

impl Server{
    pub fn new(addr: String) -> Self{ // -> Server, funkcija vraća instance of a Server, Self allias
        Self { // Self allias
            addr //addr: addr, je sintaksa ali obzirom da je isti naziv varijable kao gore u struct, može se skraćeno pisati 
        }
    }

    pub fn run(self, handler: impl Handler){
        println!("Listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap(); //unwrap makes it unrecoverable error if err happens
        loop{ //infinite loop
            match listener.accept(){
                Ok((mut stream, _)) => { //(stream, addr) ali obzirom da nam addr ne treba onda da zadovoljimo compiler je dovoljno staviti underscore _ za drugi argument koji daje Ok()
                    let mut buf = [0;1024]; //niz nula dužine 1024, 1024 bajta iskorištena
                    match stream.read(&mut buf){
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buf));
                            
                            let response = match Request::try_from(&buf[..]){
                                Ok(request)=>{
                                    /*dbg!(request);
                                    Response::new(StatusCode::Ok,
                                    Some("<h1>TEST</h1>".to_string()),
                                )*/ //bez handlera, dole sa handlerom
                                    handler.handle_request(&request)
                                }
                                Err(e) => {
                                    /*println!("Failed to parse a request: {}", e);
                                    Response::new(StatusCode::BadRequest, None)*/
                                    handler.handle_bad_request(&e)
                                }
                            };
                            if let Err(e) = response.send(&mut stream){
                                println!("Failed to send response: {}", e);
                            }
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                },
                Err(e) => println!("Failed to establish a connection: {}", e), // _ => {} default case
            }
        }
    }
}
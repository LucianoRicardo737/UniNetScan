
use pinger::{ping, PingResult};
use std::vec::Vec;
use std::string::String;
use local_ip_address::local_ip;
use std::net::IpAddr;
use std::string::*;

fn main() {
    let start_number: i32 = 1;
    let end_number: i32 = 256;

let mut list_of_ips:Vec<String>  = Vec::with_capacity(1000);

let my_local_ip:IpAddr = local_ip().unwrap();
let separate_ip:String =  my_local_ip.to_string();
let strings: Vec<&str> = separate_ip.split(".").collect(); 

// let ip_rage_start:&str = strings[0] + "." + strings[1] + "." + strings[2] + ".";
// println!("This is my local IP address: {:?}", strings);
let first_number_ip:&str = strings[0];
let second_number_ip:&str = strings[1];
let third_number_ip:&str = strings[2];


let concat_ip:String = String::from(first_number_ip) + "." + second_number_ip + "." + third_number_ip;

    for i in start_number..end_number {
        let ip_address = format!("{}.{}",concat_ip, i);
        let stream = ping(ip_address.to_string()).expect("Error pinging");

        for message in stream {
            match message {
                PingResult::Pong(duration, _) => {
                    list_of_ips.push(ip_address);
                    // println!("{:?}, {:?}", duration, ip_address)
                    println!("Encontrada");

            },
                PingResult::Timeout(_) => println!("Escaneando..."),
                // PingResult::Timeout(_) => println!("{:?} sin conexión", ip_address),
                PingResult::Unknown(_line) => ()
            }
            break;
        }
        
        
    }
    println!("Hay {:?} dispositivos conectados a su red", list_of_ips.len());
    println!("{:?}", list_of_ips);
    println!("Su ip local es: {:?}", my_local_ip);
   
}
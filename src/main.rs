use std::env;
use std::fs::File;
use std::io::Read;
use tokio::net::*;
use tokio::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg_len = args.len();
    match arg_len {
        2 => println!("Get descriptor: \"{}\"", args[1]),
        _ => {
            println!("Specify only one file or adress in format <host:port>");
            return;
        }
    };
    let _f = match File::open(&args[1]) {
        Ok(f) => {
            println!("File was found");
            parse_file(f)
        }
        Err(_err) => {
            println!("No file was found, proceed to connect to the server");
            parse_server(&args[1]);
        }
    };
}

fn process(numbers: Vec<i64>) {
    for i in numbers {
        println!("{:.4}", (i as f64).sqrt());
    }
}

fn parse_file(mut file: File) {
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(ok) => ok,
        Err(_err) => {
            println!("Unable to read values");
            return;
        }
    };
    let v: Vec<&str> = contents.trim().split(|c| c == '\n').collect();
    let cap = match v[0].parse::<usize>() {
        Ok(ok) if ok <= 0 => {
            println!("Numbers count cannot be 0 or less");
            return;
        }
        Ok(ok) => ok,
        Err(_err) => {
            println!("Unable to read amount of numbers");
            return;
        }
    };
    let mut numbers: Vec<i64> = Vec::new();
    if v.len() - 1 < cap {
        println!("Not enough data");
        return;
    }
    for n in 0..(std::cmp::min(v.len() - 1, cap)) {
        let num = match v[(n + 1) as usize].parse::<i64>() {
            Ok(ok) => ok,
            Err(_err) => {
                println!("Unable to read number");
                return;
            }
        };
        numbers.push(num);
    }
    process(numbers);
}

#[tokio::main]
async fn parse_server(addr: &String) {
    let mut stream = match TcpStream::connect(addr).await {
        Ok(ok) => ok,
        Err(_err) => {
            println!("Unable to connect");
            return;
        }
    };
    let cap: i64 = match stream.read_i64().await {
        Ok(ok) if ok <= 0 => {
            println!("Numbers count cannot be 0 or less");
            return;
        }
        Ok(ok) => ok,
        Err(_err) => {
            println!("Unable to read amount of numbers");
            return;
        }
    };
    let mut numbers = Vec::<i64>::with_capacity(cap as usize);
    numbers.resize(cap as usize, 0);
    for i in &mut numbers {
        *i = match stream.read_i64().await {
            Ok(ok) => ok,
            Err(_err) => {
                println!("Unable to read number");
                return;
            }
        };
    }
    process(numbers);
}

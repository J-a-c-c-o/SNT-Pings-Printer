use std::str::FromStr;
use std::time::Duration;
use std::vec;

use figlet_rs::FIGfont;
pub use image::imageops;
use pnet::packet::util;
use pnet::packet::Packet;

const MAX_X: u64 = 1920;
const MAX_Y: u64 = 1080;

pub fn run(args: Vec<String>) {
    // args are image_path, position, scale, part of ipv6
    if args.len() != 3 {
        println!("Usage: {} <wait time> <server>", args[0]);
        return;
    }

    // Welcome message
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("ChaChiPrint");
    println!("{}", figure.unwrap());

    let server = match args[2].parse::<String>() {
        Ok(value) => value,
        Err(_) => {
            println!("Error parsing server");
            return;
        }
    };
    
    let mut ipv6s = match build_ipv6(&server) {
        Some(value) => value,
        None => return,
    };

    // calculate bandwidth for 1 run each packet is 16 bytes
    let mut txv6 = create_tx();

    let wait_time: f64 = match args[1].parse::<f64>() {
        Ok(value) => value,
        Err(_) => {
            println!("Error parsing wait time");
            return;
        }
    };

    let seconds = Duration::from_secs(60);
    let mut start = std::time::Instant::now();
    let mut counter: u64 = 0;
    println!("Starting to print image");
    loop {
        if start.elapsed() > seconds {
            println!("Sent {} packets", counter);
            println!("Refreshing image");
            ipv6s = match build_ipv6(&server) {
                Some(value) => value,
                None => return,
            };

            start = std::time::Instant::now();
        }
        
        print_image(&ipv6s, &mut txv6);

        counter += ipv6s.len() as u64;
        
        std::thread::sleep(std::time::Duration::from_secs_f64(wait_time));
    }


    

}

fn build_ipv6(server: &str) -> Option<Vec<String>> {
    // read image from server
    let img;

    match reqwest::blocking::get(server.to_owned() + "/sntpings/image") {
        Ok(response) => {
            let img_bytes = response.bytes().unwrap();
            img = Some(image::load_from_memory(&img_bytes).unwrap());
        },
        Err(_) => {
            println!("Error getting image from server");
            return None;
        }
    
    }

    // get image size
    let scale_x: u64;
    let scale_y: u64;

    match reqwest::blocking::get(server.to_owned() + "/sntpings/size") {
        Ok(response) => {
            let json = response.json::<serde_json::Value>().unwrap();
            scale_x = json["x"].as_u64().unwrap();
            scale_y = json["y"].as_u64().unwrap();
        
        },
        Err(_) => {
            println!("Error getting image size from server");
            return None;
        }
    
    }

    // resize image
    let img = imageops::resize(&img.unwrap(), scale_x as u32, scale_y as u32, image::imageops::FilterType::Nearest);
    
    // get image position
    let x: u64;
    let y: u64;

    match reqwest::blocking::get(server.to_owned() + "/sntpings/location") {
        Ok(response) => {
            let json = response.json::<serde_json::Value>().unwrap();
            x = json["x"].as_u64().unwrap();
            y = json["y"].as_u64().unwrap();
        
        },
        Err(_) => {
            println!("Error getting image position from server");
            return None;
        }
    
    }

    if x + img.width() as u64 > MAX_X || y + img.height() as u64 > MAX_Y {
        println!("Image is too big for the screen current size is {}x{} and image size is {}x{}", MAX_X, MAX_Y, img.width(), img.height());
        return None;
    }

    let mut pixels_array: Vec<Vec<[u8; 4]>> = vec![vec![[0; 4]; img.width() as usize] ; img.height() as usize];

    for i in 0..img.height() {
        for j in 0..img.width() {
            let pixel = img.get_pixel(j, i);
            let rgba = pixel.0;
            pixels_array[i as usize][j as usize] = rgba;
        }
    }

    let mut ipv6 ;

    match reqwest::blocking::get(server.to_owned() + "/sntpings/ip") {
        Ok(response) => {
            let json = response.json::<serde_json::Value>().unwrap();
            ipv6 = Some(json["ip"].as_str().unwrap().to_string());
            ipv6 = Some(ipv6.unwrap().replace("\"", ""));
        
        },
        Err(_) => {
            println!("Error getting ipv6 prefix from server");
            return None;
        }
    
    }

    let mut ipv6s: Vec<String> = Vec::new();
    let ipv6string = ipv6.unwrap();
    
    for i in 0..img.height() {
        for j in 0..img.width() {
            let pixel = pixels_array[i as usize][j as usize];
            let blue = pixel[2];
            let green = pixel[1];
            let red = pixel[0];
            let alpha = pixel[3];
        
            let ipv6 = format!("{}:{:04x}:{:04x}:{:02x}{:02x}:{:02x}{:02x}", ipv6string, x + j as u64, y + i as u64, blue, green, red, alpha);
        
            ipv6s.push(ipv6);
        }
    }

    //shuffle ipv6s
    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();
    ipv6s.shuffle(&mut rng);
    Some(ipv6s)
}

fn print_image(ipv6s: &Vec<String>, txv6: &mut pnet::transport::TransportSender) {
    for i in 0..ipv6s.len() {
        sendecho((*ipv6s[i].clone()).to_string(), txv6);
    }
}

fn create_tx() -> pnet::transport::TransportSender {
    let protocolv6 = pnet::transport::TransportChannelType::Layer4(pnet::transport::TransportProtocol::Ipv6(pnet::packet::ip::IpNextHeaderProtocols::Icmpv6));
    
    let (txv6,_) = pnet::transport::transport_channel(4096, protocolv6).unwrap();
    return txv6;
}


fn sendecho(ipv6: String, txv6: &mut pnet::transport::TransportSender) {
    let mut vec: Vec<u8> = vec![0; 16];
    let addr = std::net::IpAddr::from_str(&ipv6).unwrap();

    let mut echo_packet = pnet::packet::icmpv6::MutableIcmpv6Packet::new(&mut vec[..]).unwrap();
    echo_packet.set_icmpv6_type(pnet::packet::icmpv6::Icmpv6Types::EchoRequest);
    let csum = util::checksum(echo_packet.packet(),1);
    echo_packet.set_checksum(csum);


    let err = txv6.send_to(echo_packet, addr);

    if err.is_err() {
        println!("Error sending packet: {}", err.err().unwrap());
    }
}
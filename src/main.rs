use std::str::FromStr;
use std::vec;

pub use image::ImageReader;
pub use image::imageops;
use pnet::packet::icmpv6::MutableIcmpv6Packet;
use pnet::packet::util;
use pnet::packet::Packet;

fn main() {
    // read arguments
    let args: Vec<String> = std::env::args().collect();
    // args are image_path, position, scale, part of ipv6
    if args.len() != 7 {
        println!("Usage: {} <image_path> <x> <y> <scaleX> <scaleY> <part of ipv6>", args[0]);
        return;
    }

    // read image
    let image_path = &args[1];
    let img = image::open(image_path).unwrap().to_rgba8();
    // resize image
    let scale_x = args[4].parse::<u32>().unwrap();
    let scale_y = args[5].parse::<u32>().unwrap();
    let img = imageops::resize(&img, scale_x, scale_y, image::imageops::FilterType::Nearest);

    // read position
    let x = args[2].parse::<u32>().unwrap();
    let y = args[3].parse::<u32>().unwrap();

    let max_x: u32 = 1920;
    let max_y: u32 = 1080;

    if x + img.width() > max_x || y + img.height() > max_y {
        println!("Image is too big for the screen current size is {}x{} and image size is {}x{}", max_x, max_y, img.width(), img.height());
        return;
    }


    // create pixel array B G R A
    // let mut pixelsArray: Vec<Vec<u8>> = vec![vec![0; img.width() as usize] ; img.height() as usize];
    let mut pixels_array: Vec<Vec<[u8; 4]>> = vec![vec![[0; 4]; img.width() as usize] ; img.height() as usize];

    for i in 0..img.height() {
        for j in 0..img.width() {
            let pixel = img.get_pixel(j, i);
            let rgba = pixel.0;
            pixels_array[i as usize][j as usize] = rgba;
        }
    }

    // create ipv6 addresses 
    // 2001:610:1908:a000:<X>:<Y>:<BLUE><GREEN>:<RED><ALPHA>
    let ipv6 = args[6].clone();
    let mut ipv6s: Vec<String> = Vec::new();

    for i in 0..img.height() {
        for j in 0..img.width() {
            let pixel = pixels_array[i as usize][j as usize];
            let blue = pixel[2];
            let green = pixel[1];
            let red = pixel[0];
            let alpha = pixel[3];
            
            let ipv6 = format!("{}:{:04x}:{:04x}:{:02x}{:02x}:{:02x}{:02x}", ipv6, x + j, y + i, blue, green, red, alpha);
            ipv6s.push(ipv6);
        }
    }

    //shuffle ipv6s
    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();
    ipv6s.shuffle(&mut rng);




    // ping ipv6 addresses using icmpv6 just run no return expected

    // let mut txv6 = create_tx();
    // loop {
    //     print_image(&ipv6s, &mut txv6);
    // }

    // calculate bandwidth for 1 run each packet is 16 bytes
    let mut txv6 = create_tx();
    let mut wait_time: f64 = 0.0;
    loop {
        println!("Sending image");
        let size = ipv6s.len() * 16 * 160;
        let current = std::time::Instant::now();
        print_image(&ipv6s, &mut txv6);
        let elapsed = current.elapsed().as_secs_f64();
        let bandwidth = size as f64 / elapsed;
        println!("Bandwidth: {} bps", bandwidth);
        wait_time = size as f64 / bandwidth * 2.0;

        println!("Waiting for {} seconds", wait_time);
        std::thread::sleep(std::time::Duration::from_secs_f64(wait_time));
    }


    

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
    let csum = ipv6_checksum(&echo_packet);
    echo_packet.set_checksum(csum);


    let err = txv6.send_to(echo_packet, addr);

    

    if err.is_err() {
        println!("Error sending packet: {}", err.err().unwrap());
    }
}


fn ipv6_checksum(packet: &MutableIcmpv6Packet) -> u16 {
    util::checksum(packet.packet(),1)
}
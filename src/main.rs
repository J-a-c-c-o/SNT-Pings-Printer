mod local_printer;
mod server_printer;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 3 {
        local_printer::run(args);
    } else {
        server_printer::run(args);
    }
}
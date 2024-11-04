# SNT-Pings Printer

# SNT-Pings Printer

Welcome to the snt-Pings Printer project! This tool allows you to draw images on a shared canvas by sending pings. Let's make some art!

## What is SNT-Pings?

Imagine a digital canvas, 1920 by 1080 pixels, displayed across the campus. You can draw on it by sending IPv6 ping packets. It's like r/place for tech enthusiasts! Learn more at [pings.utwente.io](https://pings.utwente.io).

## What is this project?

This project is a simple yet powerful printer that sends IPv6 ping packets to the snt-Pings canvas to create images. It has two main components:

1. **Local Printer**: Prints an image from a local file.
2. **Server Printer**: Prints an image fetched from a server.

## Dependencies

Ensure you have the following dependencies in your `Cargo.toml`:

- `figlet-rs`
- `image`
- `pnet`
- `reqwest`
- `serde_json`
- `rand`

## License

This project is licensed under the MIT License.

## Contributing

We welcome contributions! If you have ideas to improve the project or find a bug, feel free to open an issue or submit a pull request.

## Contact

To contact me, you can reach out via:

- Discord: [Jacco.](https://discord.gg/users/290894031200714752)

## Acknowledgements

Special thanks to the snt-Pings team and all contributors who made this project possible.

## Screenshots

Check out some amazing artworks created using the snt-Pings Printer:

![Artwork 1](path/to/artwork1.png)
![Artwork 2](path/to/artwork2.png)

Let's create something awesome together!
## Getting Started

### Prerequisites

Before you begin, ensure you have the following installed:

- Rust
- Cargo

### Installation

Clone the repository:

```sh
git clone https://github.com/your-username/sntPings.git
cd sntPings
```

Build the project:

```sh
cargo build --release
```

### Running the Printer

#### Local Printer

To print an image from a local file, use:

```sh
cargo run -- <image_path> <pos x> <pos y> <scale x> <scale y> <ipv6> <wait_time>
```

Replace the placeholders with your specific parameters.

#### Server Printer

To print an image fetched from a server, use:

```sh
cargo run -- <wait time> <server>
```

Replace the placeholders with your specific parameters.

##### Server URLS
The following URLs needs to be supported by your server:

- `GET /sntpings/image`: Returns a random image from the server.
- `GET /sntpings/size`: Return json with the size of the image. Example: `{"x": 1920, "y": 1080}`
- `GET /sntpings/ipv6`: Return json with the ipv6 address of the server. Example: `{"ipv6": "2001:db8::1"}`
- `GET /sntpings/location`: Return json with the position of the image. Example: `{"x": 0, "y": 0}`

## Troubleshooting

If you encounter any issues, check the following:

- Ensure all dependencies are installed.
- Verify your IPv6 configuration.
- Check the image path and server URL.

For further assistance, open an issue on GitHub or join our community channels.

## Support

If you need help, feel free to reach out via discord or open an issue on GitHub. We're here to help!


Thank you for using snt-Pings Printer! Let's make some amazing art together!
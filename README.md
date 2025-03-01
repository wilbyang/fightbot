# Bot Fighter

## Overview

The ID Obfuscating Proxy is one of the ways to fight bots. It is a reverse proxy server implemented in Rust using the Pingora framework. It intercepts HTTP requests and responses, obfuscating HTML element IDs to make it more difficult for web scrapers and bots to automate interactions with web applications. The proxy can also route requests based on a configurable YAML file.

## Features

- **ID Obfuscation**: Replaces HTML element IDs with randomly generated strings.
- **JavaScript and CSS Updates**: Automatically updates references to obfuscated IDs in JavaScript and CSS.
- **Routing Configuration**: Supports routing requests to different backend services based on the request path.
- **YAML Configuration**: Easily configurable using a YAML file.

## Getting Started

### Prerequisites

- Rust (1.56 or later)
- Cargo (Rust package manager)

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/id-obfuscating-proxy.git
   cd id-obfuscating-proxy
   ```

2. Build the project:

   ```bash
   cargo build
   ```

3. Create a configuration file named `config.yml` in the root directory with the following structure:

   ```yaml
   listen_addr: localhost:8076
   routes:
     - name: Service A
       context: /geekbang
       target: http://localhost:8801
     - name: Service B
       context: /juejin
       target: http://localhost:8802
   ```

### Running the Proxy

To run the proxy server, use the following command:

```bash
cargo run -- --config config.yml
```

This will start the proxy server on `localhost:8076`.

### Example Usage

- To access Service A, navigate to `http://localhost:8076/geekbang`.
- To access Service B, navigate to `http://localhost:8076/juejin`.

### Configuration Options

- `listen_addr`: The address and port on which the proxy will listen for incoming requests.
- `routes`: A list of routes that define how requests should be forwarded to backend services.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any improvements or bug fixes.

## License

This project is licensed under the Apache License 2.0. See the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Pingora](https://github.com/cloudflare/pingora) for the reverse proxy framework.
- [Rust](https://www.rust-lang.org/) for the programming language.
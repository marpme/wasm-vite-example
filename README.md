# Web App with WebAssembly and Vite Example

This repository provides an example project for building web applications using WebAssembly (Wasm) and Vite, a fast and flexible build tool. If you want to harness the power of WebAssembly and streamline your development process with Vite, this project can serve as a great starting point.

## Overview

WebAssembly is a binary instruction format that enables high-performance execution of code on web browsers. It allows you to write code in languages like C, C++, Rust, and others, and then compile it into Wasm, which can be executed directly in web browsers. This project demonstrates how to integrate Wasm into a web application built with Vite.

## Features

- **Vite:** Vite is used as the build tool for this project, providing fast development and optimized production builds. It offers features like hot module replacement, fast development server, and tree-shaking for efficient bundling.

- **WebAssembly:** This project includes a simple example of using WebAssembly to perform computations in a more efficient way compared to pure JavaScript.

## Getting Started

Follow these steps to get the project up and running on your local machine:

1. **Clone the repository:**

   ```
   git clone https://github.com/marpme/wasm-vite-example.git
   cd wasm-vite-example
   ```

2. **Install dependencies:**

   ```
   pnpm install
   ```

3. **Build the project:**

   To build the project for development:

   ```
   pnpm run dev
   ```

   To build the project for production:

   ```
   pnpm run build
   ```

4. **Open the web app:**

   Once the build is complete, you can open the web app in your browser at `http://localhost:5174/`.

## Project Structure

The project has the following structure:

- `app/`: Contains the source code for the web application.
  - `index.ts`: The entry point of the application.
  - `index.html`: the HTML entry point for dev & build mode
- `src/`: Contains the WebAssembly source code and files.
- `vite.config.js`: The Vite configuration file.
- `tsconfig.json`: The TSC configuration file.

## Contributing

We welcome contributions from the community! If you'd like to make this example even better or fix any issues, please feel free to open an issue or create a pull request.

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Special thanks to the Vite community for their fantastic tools and resources.
- Thanks to the WebAssembly community for enabling high-performance computing in web browsers.

Enjoy building web applications with WebAssembly and Vite! If you have any questions or run into issues, don't hesitate to ask for help in the [GitHub issues](https://github.com/marpme/wasm-vite-example/issues).

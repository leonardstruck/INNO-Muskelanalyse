# INNO-Muskelanalyse

## Requirements (for development)

To run the application you need to have the following installed:

- [Node.js](https://nodejs.org/en/)
- [Yarn](https://yarnpkg.com/en/)
- [Rust](https://www.rust-lang.org/en-US/install.html) (we recommend installing rust using [rustup](https://rustup.rs/))
- [Python](https://www.python.org/downloads/)
- [OpenCV](https://opencv.org/releases.html)

## Codebase structure

The codebase is structured as follows:

- `src/` contains the frontend code (Next.js project written in TypeScript)
- `src-tauri/` contains the backend code (Tauri project written in Rust)
- `src-tauri/vendor/` contains the algorithms that analyze the images (written in Python and C++)

## Development

Before you can run the application, you need to install the dependencies:

```bash
yarn install
```

To run the application in development mode, run the following commands:

```bash
yarn tauri dev
```

## Building

To build the application, run the following command:

```bash
yarn tauri build
```
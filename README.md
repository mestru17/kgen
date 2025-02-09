# kgen

Generate Kotlin projects with an opinionated standard setup.

## Features

* Kickstart your Kotlin projects with a streamlined and opinionated setup
* Automatically generate essential project files:
    * `src/main/kotlin/<package>/Main.kt` for your main application code
    * `src/test/kotlin/<package>/MainTest.kt` for a unit test example
    * `pom.xml` for comprehensive Maven build management, including compilation, testing, documentation, local execution, and packaging as a runnable JAR
    * `Dockerfile` to containerize your application effortlessly
    * `docker-compose.yml` for smooth local development and testing
    * `application.yml` for easy configuration management
* Seamlessly integrate custom Maven repositories with `settings.xml` and `.env` templates
* Include Maven Wrapper to ensure consistent builds and to eliminate the need for manual Maven installation

## Usage

```bash
kgen --group-id com.example --artifact-id myproject --package com.example.myproject
```

## Installation

### Linux and macOS

1. Ensure you have Rust installed. If not, install it using [rustup](https://rustup.rs/):

    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

2. Follow the on-screen instructions to complete the Rust installation. This will include adding Cargo to your PATH.

3. Clone the repository:

    ```bash
    git clone https://github.com/mestru17/kgen.git
    cd kgen
    ```

4. Install the project:

    ```bash
    cargo install --path .
    ```

5. Ensure that the Cargo bin directory is in your PATH:

    ```bash
    export PATH="$HOME/.cargo/bin:$PATH"
    ```

## Contributing

At this time, I am not looking for contributions as the project is still in early development. Feel free to fork the repository and experiment on your own.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.


# {{ title }}

**TODO: Add description**

## Prerequisites

* Docker and Docker Compose (recommended).
* OR Java 8+ with Maven 3.6.3+ (or use included Maven Wrapper).

## Quick Start

1. Set up Maven repository credentials:

    ```bash
    cp .env.template .env
    # Edit .env with your credentials
    ```

2. Build and run:

    ```bash
    # Using Docker (recommended)
    docker compose up --build

    # OR using Maven
    mvn clean compile exec:java
    ```

## Configuration

### Maven Repository Access

You have two optiosn for providign Maven repository credentials:

1. **Using Environment Variables** (Recommended)

    * Copy `.env.template` to `.env`.
    * Edit `.env` with your credentials.
    * Never commit `.env` to version control.

2. **Using Maven Settings**

    * Copy `settings.xml.template` to `~/.m2/settings.xml`.
    * Edit with your credentials or use environment variables.

### Application Configuration

The application can be configured through `application.yml`. Example:

```yaml
# TODO: Add example configuration here
```

## Development

### Building and Running

Choose your preferred method:

#### Docker (Recommended)

```bash
# Build and run
docker compose up --build
```

#### Maven

```bash
# Build and run
mvn clean compile exec:java # OR ./mvnw for wrapper

# Run tests
mvn test                    # OR ./mvnw test

# Build deployment JAR
mvn clean package           # Creates JARs in target/
```

## Project Structure

```
.
├-- src/
|   ├-- main/
|   |   └-- kotlin/       # Kotlin source files
|   └-- test/
|       └-- kotlin/       # Kotlin test files
├-- .mvn/
|   └-- wrapper/          # Maven Wrapper files
├-- application.yml       # Application configuration
├-- docker-compose.yml    # Docker Compose configuration
├-- Dockerfile            # Multi-stage Dockerfile
├-- mvnw                  # Maven Wrapper script (Unix)
├-- mvnw.cmd              # Maven Wrapper script (Windows)
├-- settings.xml.template # Maven settings template
├-- .env.template         # Environment variables template
└-- pom.xml               # Maven configuration
```

## Documentation

Generate with:

```bash
mvn dokka:dokka
```

Output available in `target/dokka`.

---


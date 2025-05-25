# raspi_client
 Raspberry Pi Client for my dissertation

# Installation Guide
> [!WARNING]
>  Installing, building, and running requires Linux

## Install requirements
```bash
cargo install --path .
```

## Build
```bash
cargo build --release --target aarch64-unknown-linux-gnu
```

## Change directories
```bash
cd target
cd aarch64-unknown-linux-gnu
cd release
```
## Configure messaging layer host

```bash
touch .env
```

```bash
nano .env
```

```aiignore
# .env
# replace with your ip
HOST=192.168.0.101:8000

# debug mode sends logs extra messages to std output during execution
DEBUG=false
```

## Run
```bash
./raspi_client
```


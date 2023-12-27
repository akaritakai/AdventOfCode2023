FROM rust:latest

RUN apt-get update && \
    apt-get install -y \
      libclang-dev \
      libz3-dev && \
    rm -rf /var/lib/apt/lists/*

WORKDIR "/opt/aoc"
COPY . .

RUN cargo build --release
ENTRYPOINT ["target/release/aoc2023"]

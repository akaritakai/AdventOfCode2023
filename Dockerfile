FROM rust:latest

WORKDIR "/opt/aoc"
COPY . .

RUN cargo build --release
ENTRYPOINT ["target/release/aoc2023"]

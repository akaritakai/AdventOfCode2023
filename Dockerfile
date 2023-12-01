FROM rust:1.73.0
WORKDIR "/opt/aoc"
COPY . .
RUN cargo build --release
ENTRYPOINT ["target/release/aoc2023"]

FROM rust:latest AS build
WORKDIR /app
COPY kanjidic_server/src ./src
COPY kanjidic_server/Cargo.toml ./
RUN cargo build --release

FROM debian:latest
WORKDIR /app
COPY --from=build /app/target/release/kanjidic_server ./
COPY assets/kanjidic2.json ./
EXPOSE 8000
CMD ["./kanjidic_server", "--kanji-path", "./kanjidic2.json"]
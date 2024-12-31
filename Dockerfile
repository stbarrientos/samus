FROM rust:latest as base
WORKDIR /usr/samus/src

FROM base as dev

FROM base as release

COPY ./samus/ /usr/samus

# Build the Rust client
RUN cargo build --release

CMD [ "cargo", "run" ]
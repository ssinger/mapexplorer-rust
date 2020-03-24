FROM rust_devel as build
RUN mkdir /build
COPY Cargo.toml /build
COPY src /build/src
COPY cargo-cache/registry/ /usr/local/cargo/registry
RUN cd /build; cargo build

FROM debian:stable
RUN apt-get update
RUN apt-get install -y libssl1.1 ca-certificates
EXPOSE 8080/tcp
COPY --from=build /build/target/debug/mapexplorer /mapexplorer
#COPY target/debug/mapexplorer /mapexplorer
COPY static /static
COPY templates /templates
ENTRYPOINT ["/mapexplorer"]

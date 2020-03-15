FROM rust_devel as build
RUN mkdir /build
COPY Cargo.toml /build
COPY src /build/src
RUN cd /build; cargo build

FROM debian:stable
EXPOSE 8080/tcp
COPY --from=build /build/target/debug/mapexplorer /mapexplorer
COPY static /static
COPY templates /templates
ENTRYPOINT ["/mapexplorer"]

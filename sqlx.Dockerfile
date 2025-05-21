FROM --platform=$BUILDPLATFORM rust:latest AS build

ARG TARGETPLATFORM
ARG BUILDPLATFORM

RUN apt-get update
RUN apt-get install -y musl-tools
RUN apt-get clean

RUN if [ "$TARGETPLATFORM" = "linux/arm64" ]; then \
        wget https://musl.cc/aarch64-linux-musl-cross.tgz && \
        tar -xzf aarch64-linux-musl-cross.tgz -C /usr/local && \
        export PATH="/usr/local/aarch64-linux-musl-cross/bin:$PATH" && \
        export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER=aarch64-linux-musl-gcc && \
        export TARGET=aarch64-unknown-linux-musl; \
    else \
        export TARGET=x86_64-unknown-linux-musl; \
    fi && \
    rustup target add $TARGET && \
    cargo install --target $TARGET --git https://github.com/Tortoaster/sqlx.git --rev 0c9b586 sqlx-cli --no-default-features --features rustls,sqlite

FROM alpine:latest

COPY --from=build /usr/local/cargo/bin/sqlx /usr/local/bin

ENTRYPOINT [ "/usr/local/bin/sqlx" ]

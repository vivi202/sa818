FROM debian

RUN mkdir /cargo
COPY cargo-config /cargo/config

ENV CARGO_HOME=/cargo \
    PATH=/cargo/bin:$PATH \
    USER=root

RUN dpkg --add-architecture armhf && \
    apt-get update && \
    apt-get install -y curl git && \
    apt-get install -y build-essential && \
    apt-get install -y libc6-armhf-cross libc6-dev-armhf-cross gcc-arm-linux-gnueabihf && \
    apt-get install -y libdbus-1-dev libdbus-1-dev:armhf  && \
    apt-get clean
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > /usr/local/bin/rustup.sh

RUN bash /usr/local/bin/rustup.sh -y && \
    rustup default stable && \
    rustup target add armv7-unknown-linux-gnueabihf && \
    rustup target add x86_64-unknown-linux-gnu

RUN apt-get install -y libudev-dev libudev-dev:armhf

ENV PKG_CONFIG_PATH="/usr/lib/arm-linux-gnueabihf/pkgconfig"
ENV PKG_CONFIG_ALLOW_CROSS="true"

WORKDIR /code
ENTRYPOINT [ "sh","-c","cargo build --all-targets --release" ]

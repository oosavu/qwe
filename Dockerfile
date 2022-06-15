# base pre-built cross image
#FROM ghcr.io/cross-rs/arm-unknown-linux-gnueabihf:latest
FROM ghcr.io/cross-rs/armv7-unknown-linux-gnueabihf:main
#FROM rustembedded/cross/armv7-unknown-linux-gnueabihf
# add our foreign architecture and install our dependencies
#RUN apt-get update && apt-get install -y --no-install-recommends apt-utils
#RUN dpkg --add-architecture armhf
#RUN apt-get update && apt-get -y install libasound2-dev:armhf
#
## add our linker search paths and link arguments
#ENV CARGO_TARGET_ARM_UNKNOWN_LINUX_GNUEABIHF_RUSTFLAGS="-L /usr/lib/arm-linux-gnueabihf -C link-args=-Wl,-rpath-link,/usr/lib/arm-linux-gnueabihf $CARGO_TARGET_ARM_UNKNOWN_LINUX_GNUEABIHF_RUSTFLAGS"
#
#
#FROM rustembedded/cross:arm-unknown-linux-gnueabihf-0.2.1

RUN apt-get update && apt-get install -y --no-install-recommends apt-utils
RUN dpkg --add-architecture armhf
RUN apt-get update && apt-get -y install libasound2-dev:armhf

ENV CARGO_TARGET_ARM_UNKNOWN_LINUX_GNUEABIHF_RUSTFLAGS="-L /usr/lib/arm-linux-gnueabihf $CARGO_TARGET_ARM_UNKNOWN_LINUX_GNUEABIHF_RUSTFLAGS"
ENV CARGO_TARGET_ARM_UNKNOWN_LINUX_GNUEABIHF_RUSTFLAGS="-C link-args=-Wl,-rpath-link,/usr/lib/arm-linux-gnueabihf $CARGO_TARGET_ARM_UNKNOWN_LINUX_GNUEABIHF_RUSTFLAGS"
FROM rust:1.38-buster AS builder

RUN echo "deb http://deb.debian.org/debian buster-backports main" >> /etc/apt/sources.list \
 && cat /etc/apt/sources.list \
 && apt-get update \
 && apt-get install -y --no-install-recommends \
      -t buster-backports \
      "clang-8" \
      "build-essential" \
      "libsodium-dev" \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./
# Create a fake `src`, build deps and remove the fake `src`.
# See https://github.com/rust-lang/cargo/issues/2644.
RUN mkdir src \
 && touch src/lib.rs \
 && cargo build --locked --release \
 && rm -rf src

COPY src src
RUN ls -la \
 && cargo build --frozen --release \
 && ldd target/release/musicbot

FROM debian:buster

RUN echo "deb http://deb.debian.org/debian buster-backports main" >> /etc/apt/sources.list \
 && cat /etc/apt/sources.list \
 && apt-get update \
 && apt-get install -y --no-install-recommends \
      -t buster-backports \
      "libsodium23" \
      "libssl1.1" \
      "libcrypto++6" \
      "ca-certificates" \
      "ffmpeg" \
      "youtube-dl" \
      "aria2" \
      "curl" \
      "wget" \
      "rtmpdump" \
      "phantomjs" \
      "python3-pyxattr" \
 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/musicbot /usr/local/bin/musicbot

RUN ["ldd", "/usr/local/bin/musicbot"]

CMD ["musicbot"]

FROM rust:1.65.0 as builder
WORKDIR /usr/src/ffmpeg_converter
COPY . .
RUN apt-get update \
&& apt-get install -y ca-certificates tzdata libclang-dev \
    && rm -rf /var/lib/apt/lists/*

RUN cargo install --path .

FROM debian:bullseye-slim
#RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/ffmpeg_converter /usr/local/bin/ffmpeg_converter
CMD ["ffmpeg_converter"]

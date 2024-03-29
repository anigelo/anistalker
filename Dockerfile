﻿FROM rust:1.54 as builder
ENV PKG_CONFIG_ALLOW_CROSS=1
WORKDIR /usr/src/anistalker
COPY . .
RUN cargo install --path .

FROM gcr.io/distroless/cc-debian10
EXPOSE 8080
VOLUME /media
COPY --from=builder /usr/local/cargo/bin/apigelo /usr/local/bin/apigelo
CMD ["anistalker"]
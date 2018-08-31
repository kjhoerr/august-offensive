FROM rust:latest

WORKDIR /usr/src/august

COPY . .

RUN cargo install
EXPOSE 8080

CMD ["august-offensive"]

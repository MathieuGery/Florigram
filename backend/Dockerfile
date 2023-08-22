FROM rust:1.67 as builder

WORKDIR /usr/src/myapp
COPY . .

RUN cargo build -r


FROM alpine:3.17.0 AS runtime 

COPY --from=builder /usr/src/myapp/target/release/Florigram /usr/local/bin/Florigram

EXPOSE 8080

CMD ["/usr/local/bin/Florigram"]

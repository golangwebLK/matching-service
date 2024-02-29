FROM debian

WORKDIR /app

COPY ./target/release/matching-service /usr/bin/

RUN chmod +x /usr/bin/matching-service

CMD ["/usr/bin/matching-service"]
FROM debian

WORKDIR /app

COPY matching-service /usr/bin/

RUN chmod +x /usr/bin/matching-service

EXPOSE 3000

CMD ["/usr/bin/matching-service"]
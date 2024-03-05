FROM rust as builder

WORKDIR /app

COPY . .

RUN  echo "[source.crates-io]\n\
replace-with = 'rsproxy-sparse'\n\
[source.rsproxy]\n\
registry = \"https://rsproxy.cn/crates.io-index\"\n\
[source.rsproxy-sparse]\n\
registry = \"sparse+https://rsproxy.cn/index/\"\n\
[registries.rsproxy]\n\
index = \"https://rsproxy.cn/crates.io-index\"\n\
[net]\n\
git-fetch-with-cli = true\n" >> $CARGO_HOME/config

RUN cargo build --release


FROM debian

WORKDIR /app

COPY --from=builder /app/target/release/matching-service /usr/bin/

RUN chmod +x /usr/bin/matching-service

EXPOSE 3000

CMD ["/usr/bin/matching-service"]

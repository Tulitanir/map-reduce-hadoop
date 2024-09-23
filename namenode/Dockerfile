FROM ekidd/rust-musl-builder as build
WORKDIR /tmp/src
COPY ../map_reduce/src /tmp/src
USER root
RUN cargo build --release --workspace

FROM bde2020/hadoop-base:2.0.0-hadoop3.2.1-java8

HEALTHCHECK CMD curl -f http://localhost:9870/ || exit 1

ENV HDFS_CONF_dfs_namenode_name_dir=file:///hadoop/dfs/name
RUN mkdir -p /hadoop/dfs/name /data

COPY ../map_reduce/input.csv /data/input.csv
COPY --from=build /tmp/src/target/x86_64-unknown-linux-musl/release/mapper /map_reduce/mapper
COPY --from=build /tmp/src/target/x86_64-unknown-linux-musl/release/reducer /map_reduce/reducer

COPY ../namenode/run.sh /run.sh
RUN chmod a+x /run.sh

EXPOSE 9870

CMD ["/run.sh"]
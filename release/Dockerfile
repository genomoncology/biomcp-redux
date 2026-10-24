FROM debian:bookworm-slim
COPY --chmod=0755 biomcp /usr/local/bin/biomcp
USER 65532:65532
ENV HOME=/tmp/home XDG_CONFIG_HOME=/tmp/config XDG_CACHE_HOME=/tmp/cache
ENTRYPOINT ["/usr/local/bin/biomcp"]

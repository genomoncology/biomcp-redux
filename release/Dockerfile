ARG runtime_image
FROM ${runtime_image}
ARG runtime_packages
# The transport initializes platform trust for every mode, including replay.
RUN apt-get update && apt-get install -y --no-install-recommends ${runtime_packages} \
    && rm -rf /var/lib/apt/lists/*
COPY --chmod=0755 biomcp /usr/local/bin/biomcp
COPY LICENSE THIRD-PARTY-NOTICES.txt /usr/share/doc/biomcp/
USER 65532:65532
ENV HOME=/tmp/home XDG_CONFIG_HOME=/tmp/config XDG_CACHE_HOME=/tmp/cache
ENTRYPOINT ["/usr/local/bin/biomcp"]

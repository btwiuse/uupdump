FROM btwiuse/arch:uupdump
LABEL maintainer=btwiuse
LABEL repo=github.com/btwiuse/uupdump
COPY ./entrypoint.sh /
ENTRYPOINT ["/entrypoint.sh"]

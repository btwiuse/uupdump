FROM btwiuse/arch:uupdump
LABEL maintainer=btwiuse
LABEL repo=github.com/btwiuse/uupdump
#COPY ./target/release/uupdump /usr/bin/
COPY ./uupdump /usr/bin/
COPY ./entrypoint.sh /
# ENTRYPOINT ["/bin/bash"]
ENTRYPOINT ["/entrypoint.sh"]

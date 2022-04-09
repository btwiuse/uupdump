FROM ubuntu
LABEL maintainer=btwiuse
LABEL repo=github.com/btwiuse/uupdump
RUN apt-get update && apt-get install -y curl aria2 cabextract wimtools chntpw genisoimage busybox
#COPY ./target/release/uupdump /usr/bin/
COPY ./uupdump /usr/bin/
COPY ./entrypoint.sh /
# ENTRYPOINT ["/bin/bash"]
ENTRYPOINT ["/entrypoint.sh"]

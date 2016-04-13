FROM ubuntu:15.10

RUN apt-get update && apt-get upgrade -y
RUN apt-get install -y \
        python \
        software-properties-common

RUN apt-add-repository ppa:snappy-dev/tools -y
RUN apt-get update
RUN apt-get install -y \
        snappy-tools \
        snapcraft

CMD mkdir /root/snappy-project
WORKDIR /root/snappy-project
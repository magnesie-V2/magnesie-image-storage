FROM ubuntu:20.04

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y \
  build-essential \
  curl ;\
  apt-get autoclean && apt-get clean

# Install rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh  -s -- -y
ENV PATH $PATH:/root/.cargo/bin

# Build webservice
RUN mkdir /webservice
COPY ./webservice /webservice
RUN cd /webservice && rustup override set nightly && cargo build

ENTRYPOINT cd /webservice && cargo run
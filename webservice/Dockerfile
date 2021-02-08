FROM debian AS builder

RUN apt-get update && apt-get install -y curl default-libmysqlclient-dev libpq-dev build-essential

# Install rust
RUN curl https://sh.rustup.rs/ -sSf | \
  sh -s -- -y --default-toolchain nightly

ENV PATH="/root/.cargo/bin:${PATH}"

COPY . ./

RUN cargo build --release

FROM debian

RUN apt-get update && apt-get install -y default-libmysqlclient-dev libpq-dev netcat

COPY --from=builder \
  /target/release/webservice \
  /usr/local/bin/

COPY wait-for.sh /bin/

RUN chmod +x /bin/wait-for.sh


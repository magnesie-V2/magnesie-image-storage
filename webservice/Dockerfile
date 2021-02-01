FROM debian AS builder

# ARG DATABASE_URL

RUN apt-get update && apt-get install -y curl default-libmysqlclient-dev libpq-dev build-essential

# Install rust
RUN curl https://sh.rustup.rs/ -sSf | \
  sh -s -- -y --default-toolchain nightly

RUN /bin/bash -c "source $HOME/.cargo/env"

ENV PATH="/root/.cargo/bin:${PATH}"

# RUN /bin/bash -c "cargo install diesel_cli --no-default-features --features mysql"

COPY . ./

# RUN echo $DATABASE_URL

# RUN diesel migration run --database-url=$DATABASE_URL

RUN cargo build --release

FROM debian

RUN apt-get update && apt-get install -y default-libmysqlclient-dev libpq-dev netcat

COPY --from=builder \
  /target/release/webservice \
  /usr/local/bin/

COPY wait-for.sh /bin/

RUN chmod +x /bin/wait-for.sh


FROM rustlang/rust:nightly

COPY . .

RUN cargo clean
RUN cargo update
RUN cargo install --path .

CMD ["webservice"]

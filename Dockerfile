FROM rust AS builder
WORKDIR /src
COPY . .
RUN cargo install --path . --locked
RUN cp $(which medieval_sql) /medieval_sql

FROM debian
COPY --from=builder /medieval_sql /bin/medieval_sql
COPY ./static /app/static
COPY ./templates /app/templates
COPY ./Rocket.toml /app/Rocket.toml
WORKDIR /app
CMD /bin/medieval_sql
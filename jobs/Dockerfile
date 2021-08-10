FROM rust

WORKDIR /var/www

RUN cargo install --force cargo-make

RUN curl -L https://github.com/golang-migrate/migrate/releases/download/v4.14.1/migrate.linux-amd64.tar.gz | tar xvz -C /usr/local/bin
RUN mv /usr/local/bin/migrate.linux-amd64 /usr/local/bin/migrate

COPY . .

RUN cargo build

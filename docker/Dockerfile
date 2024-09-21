### build backend image ###

FROM rust:1.80.1 as backend

WORKDIR /app

COPY backend/Cargo.toml backend/Cargo.lock ./
COPY backend/src/ src/

RUN cargo build --release

### base image ###

FROM ubuntu:22.04

SHELL [ "/bin/sh", "-c" ]

# install global dependencies

RUN apt-get update && \
  apt-get install -y \
  curl \
  unzip \
  xz-utils \
  && rm -rf /var/lib/apt/lists/*

# install node and npm

WORKDIR /tmp

RUN curl -o node-v18.20.4-linux-x64.tar.xz https://nodejs.org/dist/v18.20.4/node-v18.20.4-linux-x64.tar.xz \
  && tar -xf node-v18.20.4-linux-x64.tar.xz \
  && mv node-v18.20.4-linux-x64 /usr/local/node \
  && ln -s /usr/local/node/bin/node /usr/bin/node \
  && ln -s /usr/local/node/bin/npm /usr/bin/npm \
  && rm node-v18.20.4-linux-x64.tar.xz

# install bun and pm2

WORKDIR /data

RUN sh -c "curl -fsSL https://bun.sh/install | bash -s \"bun-v1.1.29\"" \
  && ln -s /root/.bun/bin/bun /usr/bin/bun \
  && bun install -g pm2@5.4.2 \
  && ln -s /root/.bun/bin/pm2 /usr/bin/pm2 \
  && mkdir /pm2

# config database

WORKDIR /app/database

RUN sh -c "curl -sSf https://install.surrealdb.com | sh -s -- --version v2.0.1"

COPY database/init_server ./init_server
RUN chmod +x ./init_server

## database env variables
ENV S_USER=root
ENV S_PASS=root
ENV S_PORT=4700
ENV S_HOST=0.0.0.0
ENV S_METHOD=
ENV S_DBPATH=/var/lib/surrealdb
ENV S_LOGLEVEL=debug

# config frontend

WORKDIR /app/frontend

COPY frontend/ .

RUN bun install

# config backend

WORKDIR /app/backend

COPY --from=backend /app/target/release/backend ./backend
RUN chmod +x ./backend

## backend env variables
ENV SECRET_KEY=ZOREN4u8u4e8u4u8u4e8u4u8
ENV PASSWORD_ENCRYPTION_KEY=4WOLu6iFbO4XIXLTPwst1cFYEwGQ7vRUGXoOSxBWFuM=
ENV PORT=3000
ENV HOST=0.0.0.0
ENV LOG_LEVEL=debug

ENV DB_HOST=127.0.0.1:4700
ENV DB_USER=root
ENV DB_PASS=root
ENV DB_NAMESPACE=alphadate
ENV DB_DATABSAE=resources

ENV CORS_ORIGINS=http://127.0.0.1:4200

# config pm2 for process management

WORKDIR /data

COPY docker/ecosystem.config.js .

COPY docker/entry.sh docker/entry.backend.sh docker/entry.frontend.sh ./

RUN chmod +x ./entry.sh \
  && chmod +x ./entry.backend.sh \
  && chmod +x ./entry.frontend.sh

EXPOSE 4200
EXPOSE 3000

ENTRYPOINT ["./entry.sh"]


### build backend image ###

FROM rust:1.80.1 as backend

WORKDIR /app

COPY backend/Cargo.toml ./
COPY backend/src/ src/

RUN cargo build --release

### build frontend dist ###

FROM node:18.20.1 as frontend

WORKDIR /app/frontend

COPY frontend/ .

RUN npm install && npm run build

### base image ###

FROM nginx:latest

# install global dependencies

RUN apt-get update \
  && apt-get install -y \
  curl \
  unzip \
  xz-utils

# install node and pm2

WORKDIR /tmp

RUN curl -o node-v18.20.4-linux-x64.tar.xz https://nodejs.org/dist/v18.20.4/node-v18.20.4-linux-x64.tar.xz \
  && tar -xf node-v18.20.4-linux-x64.tar.xz \
  && mv node-v18.20.4-linux-x64 /usr/local/node \
  && ln -s /usr/local/node/bin/node /usr/bin/node \
  && ln -s /usr/local/node/bin/npm /usr/bin/npm \
  && rm node-v18.20.4-linux-x64.tar.xz \
  && npm install -g pm2@5.4.2 \
  && ln -s /usr/local/node/bin/pm2 /usr/bin/pm2 \
  && mkdir -p /data/pm2

# config hosting

WORKDIR /app/hosting

COPY hosting/nginx.conf /etc/nginx/nginx.conf

RUN rm -rf /var/log/nginx/ \
  && mkdir /var/log/nginx/ \
  && touch /var/log/nginx/error.log \
  && chown -R www-data:www-data /var/log/nginx/

# config database

WORKDIR /app/database

RUN sh -c "curl -sSf https://install.surrealdb.com | sh -s -- --version v2.0.1"

COPY database/init_server ./init_server
RUN chmod +x ./init_server

## database env variables
ENV DATABASE_USER=root
ENV DATABASE_PASS=root
ENV DATABASE_PORT=4700
ENV DATABASE_HOST=0.0.0.0
ENV DATABASE_METHOD=
ENV DATABASE_DBPATH=/var/lib/surrealdb
ENV DATABASE_LOGLEVEL=debug

# config frontend

WORKDIR /app/frontend

COPY --from=frontend /app/frontend/dist/frontend .

# config backend

WORKDIR /app/backend

COPY --from=backend /app/target/release/backend ./backend
RUN chmod +x ./backend

## backend env variables
ENV BACKEND_SECRET_KEY=ZOREN4u8u4e8u4u8u4e8u4u8
ENV BACKEND_PASSWORD_ENCRYPTION_KEY=4WOLu6iFbO4XIXLTPwst1cFYEwGQ7vRUGXoOSxBWFuM=
ENV BACKEND_PORT=3000
ENV BACKEND_HOST=0.0.0.0
ENV BACKEND_LOG_LEVEL=debug
ENV BACKEND_EXPIRATION_TOKEN_TIME_IN_MS=86400000

ENV BACKEND_DB_HOST=127.0.0.1:4700
ENV BACKEND_DB_USER=root
ENV BACKEND_DB_PASS=root
ENV BACKEND_DB_NAMESPACE=alphadate
ENV BACKEND_DB_DATABSAE=resources

ENV BACKEND_CORS_ORIGINS=http://127.0.0.1:4200

# config pm2 for process management

WORKDIR /data

COPY docker/ecosystem.config.js .

COPY docker/entry.sh docker/entry.backend.sh docker/entry.frontend.sh ./

RUN chmod +x ./entry.sh \
  && chmod +x ./entry.backend.sh \
  && chmod +x ./entry.frontend.sh

EXPOSE 80

ENTRYPOINT ["./entry.sh"]



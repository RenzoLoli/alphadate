FROM rust:1.80.1 as backend

WORKDIR /usr/src/alphadate-backend

COPY backend/Cargo.toml backend/Cargo.lock ./
COPY backend/src/ src/

RUN cargo build --release

FROM ubuntu:22.04

SHELL [ "/bin/sh", "-c" ]

# install dependencies

RUN apt-get update \
  && apt-get install -y curl \
  && apt-get install -y nodejs \
  && apt-get install -y npm \ 
  && apt-get install -y unzip

RUN sh -c "curl -fsSL https://bun.sh/install | bash" \
  && bun install -g pm2

# install database

WORKDIR /usr/bin/alphadate-database

RUN sh -c "curl -sSf https://install.surrealdb.com | sh"

COPY database/init_server ./init_server
RUN chmod +x ./init_server


# install frontend

WORKDIR /usr/src/alphadate-frontend

COPY frontend/ .

RUN bun install

# get backend

WORKDIR /usr/bin/alphadate-backend

COPY --from=backend /usr/src/alphadate-backend/target/release/backend .

# init pm2

WORKDIR /data/pm2

COPY docker/ecosystem.config.js .

CMD [ "pm2", "start", "ecosystem.config.js" ]

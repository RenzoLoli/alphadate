FROM ubuntu:20.04

WORKDIR /data
SHELL [ "/bin/sh", "-c" ]

RUN apt-get update && apt-get install -y curl

COPY database/init_server ./init_server
RUN chmod +x ./init_server

#fix next line
RUN curl -sSf https://install.surrealdb.com | sh

ENV S_USER=root
ENV S_PASS=root
ENV S_PORT=4700
ENV S_HOST=0.0.0.0
ENV S_METHOD=
ENV S_DBPATH=/data/database
ENV S_LOGLEVEL=debug

# expose dinamically port with variable
EXPOSE $S_PORT

CMD ["./init_server"]

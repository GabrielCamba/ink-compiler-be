FROM rust:1.67 as builder


RUN USER=root cargo new --bin ink-compiler
WORKDIR ./ink-compiler
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN rm -r ./target/release/*
RUN cargo build --release

FROM debian:buster-slim
ARG APP=/usr/src/app

RUN apt-get update 
RUN apt install libc6

EXPOSE 8000

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /ink-compiler/target/release/compiler-be ${APP}/compiler-be

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./compiler-be"]

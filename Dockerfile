FROM ryusei1068/rust_ubuntu:latest as build

# build for sever side 
RUN USER=root cargo new --bin server
WORKDIR /server
COPY ./server/Cargo* ./
COPY ./server/src ./src
RUN cargo build --release

FROM ubuntu:22.04
COPY --from=build /server/target/release/server ./

RUN apt update && apt install -y \ 
    nodejs \ 
    npm
COPY client/package.json ./
COPY client/*.js ./
RUN npm install

COPY ./local-rpc.sh ./
RUN chmod 755 local-rpc.sh

ENTRYPOINT [ ./local-rpc.sh ]
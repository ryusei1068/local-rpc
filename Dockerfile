FROM ryusei1068/rust_ubuntu:latest as build

# build for sever side 
RUN USER=root cargo new --bin server
WORKDIR /server
COPY ./server/Cargo* ./
COPY ./server/src ./src
RUN cargo build --release

FROM ryusei1068/node_ubuntu:latest
COPY --from=build /server/target/release/server ./
COPY client/package.json ./
COPY client/*.js ./
RUN npm install

COPY ./local-rpc.sh ./
RUN chmod 755 local-rpc.sh

ENTRYPOINT ./local-rpc.sh
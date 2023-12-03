FROM ryusei1068/rust-build:1.74.0 as build

# build for sever side 
RUN USER=root cargo new --bin server
WORKDIR /server
COPY ./server/Cargo* ./
COPY ./server/src ./src
RUN cargo build --release

FROM ryusei1068/node:20.5.1
COPY --from=build /server/target/release/server ./
COPY client/package.json ./
COPY client/*.js ./
RUN npm install

COPY ./rpc.sh ./
RUN chmod +x rpc.sh

ENTRYPOINT ./rpc.sh

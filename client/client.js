const net = require('net');

const socket_path = "/socket_file";
const client = net.createConnection(socket_path);

client.on('connect', () => {
    console.log('connected.');
});

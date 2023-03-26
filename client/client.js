const net = require('net');

const socket_path = "/socket_file";

const client = net.createConnection(socket_path, () => {
        console.log('connected to server!');
        client.write('world!\r\n');
    });
    client.on('data', (data) => {
        console.log(data.toString());
        client.end();
    });
    client.on('end', () => {
        console.log('disconnected from server');
    });
    client.on('timeout', () => {
        console.log('socket timeout');
        client.end();
    });
client.setTimeout(3000);

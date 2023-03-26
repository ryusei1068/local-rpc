const net = require('net');

const socket_path = "/socket_file";

const client = net.createConnection(socket_path, () => {
        console.log('connected to server!');
        client.write(
            `{
                "method": "floor",
                "params": [3.0],
                "params_types": ["int"],
                "id": 1
            }`
        );
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

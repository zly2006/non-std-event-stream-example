const timeStart = Date.now();

import https from 'https';

const httpsAgent = new https.Agent({
    rejectUnauthorized: false,
});

process.env.NODE_TLS_REJECT_UNAUTHORIZED = '0';

// Note: I am using nginx to reverse proxy 3000 to 3001 with a self-signed ssl certificate
const res = await fetch('https://localhost:3001/api/sse', {
    method: 'POST',
    headers: {
        "Content-Type": "application/json",
        'Apikey': 'xxxxxxx',
    },
    body: '{"Query":"Hi"}',
});

console.log('fetch resolved at', Date.now() - timeStart, 'ms')

const text = await res.text();

console.log('res:', JSON.stringify(text))
const timeStart1 = Date.now();

await fetch('http://localhost:3000/api/sse', {
    method: 'POST',
    headers: {
        "Content-Type": "application/json",
        'Apikey': 'xxxxxxx',
    },
    body: '{"Query":"Hi"}',
});

console.log('http fetch resolved at', Date.now() - timeStart1, 'ms')

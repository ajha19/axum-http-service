const consoleEl = document.getElementById('console');
const latencyEl = document.getElementById('latency');

function log(status, msg, meta = '') {
    const entry = document.createElement('div');
    entry.className = 'log-entry';
    const colorClass = status >= 200 && status < 300 ? 'status-200' : 'status-429';
    entry.innerHTML = `
        <span class="${colorClass}">[${status}]</span> ${msg}
        <br><span class="meta">${meta}</span>
    `;
    consoleEl.prepend(entry);
}

async function ping() {
    const start = performance.now();
    try {
        const res = await fetch('/health');
        const end = performance.now();
        const duration = (end - start).toFixed(1);

        latencyEl.innerText = `${duration}ms`;

        const reqId = res.headers.get('x-request-id') || 'unknown';

        if (res.ok) {
            log(res.status, 'Health Check OK', `req-id: ${reqId} | time: ${duration}ms`);
        } else {
            log(res.status, 'Request Failed', `req-id: ${reqId}`);
        }
    } catch (e) {
        log(0, 'Network Error', e.message);
    }
}

async function pingChain(count) {
    for (let i = 0; i < count; i++) {
        ping();
        // meaningful delay is not added here to simulate burst,
        // but browser might stagger them slightly.
    }
}

async function updateMetrics() {
    try {
        const res = await fetch('/metrics');
        const data = await res.json();

        // Update counters
        document.getElementById('total-req').innerText = data.total_requests;
        document.getElementById('allowed-req').innerText = data.allowed_requests;
        document.getElementById('blocked-req').innerText = data.rate_limited_requests;
    } catch (e) {
        console.error("Failed to fetch metrics", e);
    }
}

// Poll metrics every 1s
setInterval(updateMetrics, 1000);

async function stressTest() {
    log(0, 'Starting Stress Test...', 'Sending 20 requests in parallel');
    const requests = [];
    for (let i = 0; i < 20; i++) {
        requests.push(ping());
    }
    await Promise.all(requests);
    updateMetrics(); // Immediate update after stress test
}

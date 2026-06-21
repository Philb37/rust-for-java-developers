import { textSummary } from 'https://jslib.k6.io/k6-summary/0.0.1/index.js';
import { check } from 'k6';
import exec from 'k6/execution';
import http from 'k6/http';
import { Trend } from 'k6/metrics';

// ---------------------------------------------------------------------------
// Benchmark one app at a time:
//   k6 run -e URL=http://localhost:8081 -e LABEL=spring k6/load_testing.js
//   k6 run -e URL=http://localhost:8080 -e LABEL=rust   k6/load_testing.js
//
// Tunables (all optional):
//   URL       base url of the app under test      (default http://localhost:8081)
//   LABEL     name used in the summary filename    (default app)
//   VUS       concurrent virtual users             (default 50)
//   DURATION  measured phase duration              (default 2m)
//   WARMUP    warm-up phase duration (JVM JIT)     (default 30s)
// ---------------------------------------------------------------------------

const BASE = __ENV.URL || 'http://localhost:8081';
const LABEL = __ENV.LABEL || 'app';
const VUS = Number(__ENV.VUS || 50);
const DURATION = __ENV.DURATION || '2m';
const WARMUP = __ENV.WARMUP || '30s';

// Per-endpoint latency, so each step in the journey gets its own p95/p99 row.
// Recorded only during the 'benchmark' phase (warm-up must not pollute the stats).
const epList = new Trend('ep_list', true);
const epCreate = new Trend('ep_create', true);
const epGet = new Trend('ep_get', true);
const epUpdate = new Trend('ep_update', true);
const epGetAfter = new Trend('ep_get_after_update', true);
const epStats = new Trend('ep_stats', true);

export const options = {
    // Two phases on the same VUs: warm up (ignored), then measure. The benchmark
    // phase is tagged so thresholds and per-endpoint trends exclude the warm-up.
    systemTags: ['method', 'status', 'scenario', 'expected_response', 'error', 'error_code', 'check'],
    scenarios: {
        warmup: {
            executor: 'ramping-vus',
            startVUs: 0,
            stages: [
                { duration: WARMUP, target: VUS }
            ],
            gracefulStop: '5s',
        },
        benchmark: {
            executor: 'ramping-vus',
            startVUs: 0,
            stages: [
                { duration: '5s', target: VUS },
                { duration: DURATION, target: VUS }
            ],
            startTime: WARMUP,
            gracefulStop: '5s',
        },
    },
    // Gates only — the real output is the per-endpoint comparison below. A failing
    // threshold just flags that something is wrong (errors, or wildly slow).
    thresholds: {
        'http_req_failed{scenario:benchmark}': ['rate<0.01'],
        'checks{scenario:benchmark}': ['rate>0.99'],
        'http_req_duration{scenario:benchmark}': ['p(95)<1000'],
    },
};

const PRIORITIES = ['LOW', 'MEDIUM', 'HIGH', 'CRITICAL'];
const STATUSES = ['OPEN', 'IN_PROGRESS', 'RESOLVED', 'CLOSED'];
const NEXT_STATUS = ['IN_PROGRESS', 'RESOLVED', 'CLOSED'];

function jsonParams(endpoint) {
    return { headers: { 'Content-Type': 'application/json' }, tags: { endpoint } };
}

// Deterministic filter for GET /tickets, varied by the iteration sequence so the
// request differs across iterations yet is IDENTICAL for rust vs spring (it depends
// only on `seq`): 0 = no params, 1 = status only, 2 = priority only, 3 = both.
function listQuery(seq) {
    const status = STATUSES[seq % STATUSES.length];
    const priority = PRIORITIES[seq % PRIORITIES.length];
    switch (seq % 4) {
        case 1:  return `?status=${status}`;
        case 2:  return `?priority=${priority}`;
        case 3:  return `?status=${status}&priority=${priority}`;
        default: return '';
    }
}

// User journey: list -> create -> get -> update status -> get again -> stats
export default function () {
    const measuring = exec.scenario.name === 'benchmark';
    const seq = exec.scenario.iterationInTest;

    // 1. list all tickets
    // 1. list tickets — filter varies by sequence (none / status / priority / both)
    const listRes = http.get(`${BASE}/tickets${listQuery(seq)}`, jsonParams('list'));
    let ok = check(listRes, { 'list 200': (r) => r.status === 200 });
    if (measuring && ok) epList.add(listRes.timings.duration);

    // 2. create a new ticket
    const payload = JSON.stringify({
        title: `bench ticket ${exec.vu.idInTest}-${seq}`,
        priority: PRIORITIES[seq % PRIORITIES.length],
        description: null,
        assignee: null,
    });
    const createRes = http.post(`${BASE}/tickets`, payload, jsonParams('create'));
    const created = check(createRes, {
        'create 201': (r) => r.status === 201,
        'create returns id': (r) => r.status === 201 && r.json('id') !== undefined,
    });
    if (measuring && created) epCreate.add(createRes.timings.duration);
    if (!created) return; // can't continue the chain without an id
    const id = createRes.json('id');

    // 3. get it back
    const getRes = http.get(`${BASE}/tickets/${id}`, jsonParams('get'));
    ok = check(getRes, {
        'get 200': (r) => r.status === 200,
        'get matches id': (r) => r.status === 200 && r.json('id') === id,
    });
    if (measuring && ok) epGet.add(getRes.timings.duration);

    // 4. update its status
    const newStatus = NEXT_STATUS[seq % NEXT_STATUS.length];
    const patchRes = http.patch(
        `${BASE}/tickets/${id}/status`,
        JSON.stringify({ status: newStatus }),
        jsonParams('update'),
    );
    ok = check(patchRes, { 'update 200': (r) => r.status === 200 });
    if (measuring && ok) epUpdate.add(patchRes.timings.duration);

    // 5. get it again — confirm the new status stuck ('status' field is the same in both apps)
    const getAfterRes = http.get(`${BASE}/tickets/${id}`, jsonParams('get_after_update'));
    ok = check(getAfterRes, {
        'get-after 200': (r) => r.status === 200,
        'status updated': (r) => r.status === 200 && r.json('status') === newStatus,
    });
    if (measuring && ok) epGetAfter.add(getAfterRes.timings.duration);

    // 6. stats — naming-agnostic check (Spring: byStatus / Rust: by_status)
    const statsRes = http.get(`${BASE}/tickets/stats`, jsonParams('stats'));
    ok = check(statsRes, {
        'stats 200': (r) => r.status === 200,
        'stats has body': (r) => r.status === 200 && r.body && r.body.length > 2,
    });
    if (measuring && ok) epStats.add(statsRes.timings.duration);
}

// Write a per-target summary so you can diff rust vs spring afterwards.
export function handleSummary(data) {
    return {
        stdout: textSummary(data, { indent: ' ', enableColors: true }),
        [`summary-${VUS}-${LABEL}.json`]: JSON.stringify(data, null, 2),
    };
}

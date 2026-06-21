// Generate grouped-bar SVG charts from the k6 summaries — no dependencies.
//
//   node k6/make-charts.mjs
//
// Reads k6/summaries/summary-<vus>-<target>.json and writes:
//   docs/throughput.svg   docs/latency-p95.svg   docs/latency-avg.svg
//
// Re-run after a fresh benchmark to refresh the charts the README points at.

import { readFileSync, writeFileSync, mkdirSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const here = dirname(fileURLToPath(import.meta.url));
const summariesDir = join(here, 'summaries');
const outDir = join(here, '..', 'docs');
mkdirSync(outDir, { recursive: true });

const TARGETS = [
  { file: 'rust', label: 'rust' },
  { file: 'springboot-mvc', label: 'mvc-jvm' },
  { file: 'springboot-webflux', label: 'webflux-jvm' },
  { file: 'springboot-mvc-native', label: 'mvc-native' },
  { file: 'springboot-webflux-native', label: 'webflux-native' },
];
const LOADS = [50, 100, 300];
const COLORS = ['#9ecae1', '#4292c6', '#08519c']; // 50 / 100 / 300 VUs (light → dark)

// Peak container memory (MB), load-independent — comes from Telegraf/Prometheus,
// NOT the k6 summaries. Refresh with:
//   max_over_time(docker_container_mem_usage{container_name=~".*-app"}[6h]) / 1e6
const PEAK_MEM_MB = {
  rust: 24,
  'springboot-mvc': 527,
  'springboot-webflux': 412,
  'springboot-mvc-native': 300,
  'springboot-webflux-native': 303,
};

// Peak CPU (cores; container cap = 4), from the same Prometheus query as memory.
const PEAK_CPU = {
  rust: 1.4,
  'springboot-mvc': 4.1,
  'springboot-webflux': 4.0,
  'springboot-mvc-native': 3.8,
  'springboot-webflux-native': 3.2,
};

// Startup in seconds (process → ready), from each app's "ready" log line.
// Huge range (0.5 ms … 5.2 s) → charted on a log scale.
const STARTUP_S = {
  rust: 0.0005,
  'springboot-mvc': 5.2,
  'springboot-webflux': 3.7,
  'springboot-mvc-native': 0.95,
  'springboot-webflux-native': 0.27,
};

const read = (vus, file) =>
  JSON.parse(readFileSync(join(summariesDir, `summary-${vus}-${file}.json`), 'utf8'));
const benchmark = (m) => m.metrics['http_req_duration{scenario:benchmark}'].values;

// data[metric][vus] = [value per target, in TARGETS order]
function collect() {
  const reqs = {}, p95 = {}, avg = {};
  for (const vus of LOADS) {
    reqs[vus] = []; p95[vus] = []; avg[vus] = [];
    for (const t of TARGETS) {
      const m = read(vus, t.file);
      reqs[vus].push(m.metrics.http_reqs.values.rate);
      p95[vus].push(benchmark(m)['p(95)']);
      avg[vus].push(benchmark(m).avg);
    }
  }
  return { reqs, p95, avg };
}

const NICE = [1, 1.2, 1.5, 2, 2.5, 3, 4, 5, 6, 8, 10];
function niceMax(v) {
  if (v <= 0) return 1;
  const pow = 10 ** Math.floor(Math.log10(v));
  const n = v / pow;
  return (NICE.find((x) => x >= n - 1e-9) ?? 10) * pow;
}
const esc = (s) => String(s).replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
const kFmt = (v) => (v >= 1000 ? `${v / 1000}k` : String(Math.round(v)));

function chart({ title, yLabel, series, fmt, colors = COLORS, legend = true, log = false, axisFmt = kFmt }) {
  const cats = TARGETS.map((t) => t.label);
  const W = 860, H = 450, m = { top: 70, right: 22, bottom: 64, left: 72 };
  const pw = W - m.left - m.right, ph = H - m.top - m.bottom;
  const gW = pw / cats.length, groupW = gW * 0.72, bw = groupW / series.length;
  const vals = series.flatMap((ser) => ser.values).filter((v) => v > 0);
  let yPix, ticks;
  if (log) {
    const lo = Math.floor(Math.log10(Math.min(...vals)));
    const hi = Math.ceil(Math.log10(Math.max(...vals)));
    yPix = (v) => m.top + ph * (1 - (Math.log10(v) - lo) / (hi - lo));
    ticks = Array.from({ length: hi - lo + 1 }, (_, i) => 10 ** (lo + i));
  } else {
    const yMax = niceMax(Math.max(...vals) * 1.08);
    yPix = (v) => m.top + ph * (1 - v / yMax);
    ticks = Array.from({ length: 6 }, (_, k) => (yMax * k) / 5);
  }
  const s = [];

  s.push(`<svg xmlns="http://www.w3.org/2000/svg" width="${W}" height="${H}" viewBox="0 0 ${W} ${H}" font-family="-apple-system, Segoe UI, Roboto, Helvetica, Arial, sans-serif">`);
  s.push(`<rect width="${W}" height="${H}" rx="10" fill="#ffffff" stroke="#e5e9f0"/>`);
  s.push(`<text x="${m.left}" y="32" font-size="16" font-weight="700" fill="#1f2933">${esc(title)}</text>`);

  // legend (top-right)
  if (legend) {
    const itemW = 92, legX = W - m.right - series.length * itemW;
    series.forEach((ser, i) => {
      const x = legX + i * itemW;
      s.push(`<rect x="${x}" y="19" width="12" height="12" rx="2" fill="${colors[i]}"/>`);
      s.push(`<text x="${x + 17}" y="29" font-size="12" fill="#3e4c59">${esc(ser.label)}</text>`);
    });
  }

  // y gridlines + labels
  ticks.forEach((v, i) => {
    const y = yPix(v);
    s.push(`<line x1="${m.left}" y1="${y.toFixed(1)}" x2="${m.left + pw}" y2="${y.toFixed(1)}" stroke="${i === 0 ? '#9aa5b1' : '#eceff4'}"/>`);
    s.push(`<text x="${m.left - 8}" y="${(y + 4).toFixed(1)}" font-size="11" fill="#7b8794" text-anchor="end">${axisFmt(v)}</text>`);
  });
  s.push(`<text x="18" y="${m.top + ph / 2}" font-size="12" fill="#3e4c59" text-anchor="middle" transform="rotate(-90 18 ${m.top + ph / 2})">${esc(yLabel)}</text>`);

  // bars, value labels, category labels
  cats.forEach((cat, ci) => {
    const gx = m.left + ci * gW + (gW - groupW) / 2;
    series.forEach((ser, si) => {
      const v = ser.values[ci], bx = gx + si * bw, by = yPix(v), bh = m.top + ph - by;
      s.push(`<rect x="${bx.toFixed(1)}" y="${by.toFixed(1)}" width="${(bw - 3).toFixed(1)}" height="${Math.max(0, bh).toFixed(1)}" rx="2" fill="${colors[si]}"/>`);
      s.push(`<text x="${(bx + (bw - 3) / 2).toFixed(1)}" y="${(by - 4).toFixed(1)}" font-size="9" fill="#52606d" text-anchor="middle">${fmt(v)}</text>`);
    });
    s.push(`<text x="${(m.left + ci * gW + gW / 2).toFixed(1)}" y="${m.top + ph + 22}" font-size="11.5" fill="#1f2933" text-anchor="middle">${esc(cat)}</text>`);
  });

  s.push('</svg>');
  return s.join('\n');
}

const { reqs, p95, avg } = collect();
const seriesOf = (metric) => LOADS.map((vus) => ({ label: `${vus} VUs`, values: metric[vus] }));

const oneSeries = (data) => [{ label: '', values: TARGETS.map((t) => data[t.file]) }];
// seconds → "0.5 ms" / "270 ms" / "3.7 s"
const timeLabel = (s) => (s >= 1 ? `${+s.toFixed(2)} s` : `${+(s * 1000).toFixed(s < 0.01 ? 1 : 0)} ms`);

const charts = {
  'throughput.svg': chart({ title: 'Throughput (req/s) — higher is better', yLabel: 'req/s', series: seriesOf(reqs), fmt: (v) => Math.round(v) }),
  'latency-p95.svg': chart({ title: 'Latency p95 (ms) — lower is better', yLabel: 'ms', series: seriesOf(p95), fmt: (v) => Math.round(v) }),
  'latency-avg.svg': chart({ title: 'Latency avg (ms) — lower is better', yLabel: 'ms', series: seriesOf(avg), fmt: (v) => v.toFixed(1) }),
  'memory.svg': chart({ title: 'Peak memory under load (MB) — lower is better', yLabel: 'MB', series: oneSeries(PEAK_MEM_MB), fmt: (v) => Math.round(v), colors: ['#4292c6'], legend: false }),
  'cpu.svg': chart({ title: 'Peak CPU under load (cores, cap = 4) — lower is leaner', yLabel: 'CPU cores', series: oneSeries(PEAK_CPU), fmt: (v) => v.toFixed(1), colors: ['#4292c6'], legend: false }),
  'startup.svg': chart({ title: 'Startup: process → ready — lower is better (log scale)', yLabel: '', series: oneSeries(STARTUP_S), fmt: timeLabel, axisFmt: timeLabel, colors: ['#4292c6'], legend: false, log: true }),
};

for (const [name, svg] of Object.entries(charts)) {
  writeFileSync(join(outDir, name), svg);
  console.log(`wrote docs/${name}`);
}

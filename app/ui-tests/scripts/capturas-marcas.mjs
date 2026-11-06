// Capturas de la tarea 8 (D-31): V1 «solo nombre» y V2 «gesto por grupo», claro/oscuro, es/en,
// pantalla principal sin portada y con portada (880×860, ×2).
// Uso: node ui-tests/scripts/capturas-marcas.mjs <carpeta de salida>
import { mkdirSync } from 'node:fs';
import { resolve } from 'node:path';
import { serve, loadPlaywright } from './serve.mjs';

const out = resolve(process.argv[2] || 't8-captures');
mkdirSync(out, { recursive: true });
const { chromium } = await loadPlaywright();
const { server, base } = await serve();
const browser = await chromium.launch();
let bad = 0;
for (const [v, chips] of [['v1', 'nombre'], ['v2', 'gesto']]) {
  for (const theme of ['light', 'dark']) {
    for (const lang of ['es', 'en']) {
      for (const [cov, q] of [['sin-portada', ''], ['con-portada', '&cover=1&own=canvas']]) {
        const page = await browser.newPage({ viewport: { width: 880, height: 860 }, deviceScaleFactor: 2 });
        const errs = [];
        page.on('pageerror', e => errs.push(e.message));
        page.on('console', m => m.type() === 'error' && errs.push(m.text()));
        await page.goto(`${base}/index.html?backend=mock&lang=${lang}&theme=${theme}&state=normal&still=1&chips=${chips}${q}`);
        await page.waitForSelector('html[data-ready]');
        await page.waitForTimeout(500);
        const name = `${v}-${theme === 'light' ? 'claro' : 'oscuro'}-${lang}-${cov}.png`;
        await page.screenshot({ path: `${out}/${name}` });
        if (errs.length) { bad++; console.error(name, errs); } else console.log(name);
        await page.close();
      }
    }
  }
}
await browser.close(); server.close();
process.exit(bad ? 1 : 0);

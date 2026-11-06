// Capturas de la marca ferboix. studio, opción C «Instrumento» (D-37): pantalla principal y «Acerca de»,
// claro/oscuro, es/en (880×860, ×2), más el estado vacío (símbolo grande) y «listo» (el potenciómetro gira).
// Uso: node ui-tests/scripts/capturas-marca.mjs <carpeta de salida>
import { mkdirSync } from 'node:fs';
import { resolve } from 'node:path';
import { serve, loadPlaywright } from './serve.mjs';

const out = resolve(process.argv[2] || 'brand-c-captures');
mkdirSync(out, { recursive: true });
const { chromium } = await loadPlaywright();
const { server, base } = await serve();
const browser = await chromium.launch();
let bad = 0;
for (const theme of ['light', 'dark']) {
  for (const lang of ['es', 'en']) {
    for (const [name, q, about] of [['principal', 'state=normal&cover=1', false], ['acerca-de', 'state=normal&cover=1', true], ['vacio', 'state=empty', false], ['listo', 'state=done&cover=rect', false]]) {
      const page = await browser.newPage({ viewport: { width: 880, height: 860 }, deviceScaleFactor: 2 });
      const errs = [];
      page.on('pageerror', e => errs.push(e.message));
      page.on('console', m => m.type() === 'error' && errs.push(m.text()));
      await page.goto(`${base}/index.html?backend=mock&lang=${lang}&theme=${theme}&still=1&${q}`);
      await page.waitForSelector('html[data-ready]');
      if (about) { await page.click('#aboutBtn'); await page.waitForSelector('#pop-about.open'); }
      await page.waitForTimeout(600);
      const file = `${name}-${theme === 'light' ? 'claro' : 'oscuro'}-${lang}.png`;
      await page.screenshot({ path: `${out}/${file}` });
      if (errs.length) { bad++; console.error(file, errs); } else console.log(file);
      await page.close();
    }
  }
}
await browser.close(); server.close();
process.exit(bad ? 1 : 0);

// Capturas con el mock (880×860, ×2): 01 vacío, 02 normal claro, 03 normal oscuro, 06 creando, 07 listo, 08 material propio.
// Uso: node ui-tests/scripts/capturas.mjs [es|en]
import { fileURLToPath } from 'node:url';
import { serve, loadPlaywright } from './serve.mjs';

const lang = process.argv[2] === 'en' ? 'en' : 'es';
const OUT = fileURLToPath(new URL('../capturas-mock/', import.meta.url));
const { chromium } = await loadPlaywright();
const { server, base } = await serve();
const browser = await chromium.launch();

const SHOTS = [
  ['01_vacio.png', 'theme=light&state=empty'],
  ['02_normal_claro.png', 'theme=light&state=normal'],
  ['03_normal_oscuro.png', 'theme=dark&state=normal'],
  ['06_creando.png', 'theme=light&state=creating'],
  ['07_listo.png', 'theme=light&state=done&cover=rect'],
  ['08_material_propio.png', 'theme=light&state=normal&cover=1&own=canvas,ytv']
];
let bad = 0;
for (const [name, q] of SHOTS) {
  const page = await browser.newPage({ viewport: { width: 880, height: 860 }, deviceScaleFactor: 2 });
  const errs = [];
  page.on('pageerror', e => errs.push(e.message));
  page.on('console', m => m.type() === 'error' && errs.push(m.text()));
  await page.goto(`${base}/index.html?backend=mock&lang=${lang}&still=1&${q}`);
  await page.waitForSelector('html[data-ready]');
  await page.waitForTimeout(500);
  await page.screenshot({ path: OUT + name, omitBackground: true });
  const overflow = await page.evaluate(() => [document.scrollingElement.scrollHeight > innerHeight, document.scrollingElement.scrollWidth > innerWidth]);
  console.log(name, 'overflow', overflow, errs);
  if (errs.length) bad++;
  await page.close();
}
await browser.close(); server.close();
process.exit(bad ? 1 : 0);

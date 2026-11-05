// Recorre vacío → soltar → crear → listo con el mock, en es y en en, sin errores de página.
// Uso: node ui-tests/scripts/ui-check.mjs   (necesita Playwright + Chromium)
import assert from 'node:assert/strict';
import { serve, loadPlaywright } from './serve.mjs';

const { chromium } = await loadPlaywright();
const { server, base } = await serve();
const browser = await chromium.launch();

const T = {
  es: { drop: 'Suelta tu canción', create: 'Crear carpeta de entrega', creating: 'Creando carpeta…', open: 'Abrir carpeta', again: 'Nueva canción', lufs: /^−9,1 LUFS · pasa de 0 dB$/, done: /^Listo: \d+ archivos en Vera Alcón - Luz de Noviembre$/, notWav: /no es un WAV válido/, lame: /falta lame/i, dist: 'Distribuidora' },
  en: { drop: 'Drop your song', create: 'Create delivery folder', creating: 'Creating folder…', open: 'Open folder', again: 'New song', lufs: /^-9\.1 LUFS · over 0 dB$/, done: /^Done: \d+ files in Vera Alcón - Luz de Noviembre$/, notWav: /not a valid WAV/, lame: /lame is missing/i, dist: 'Distributor' }
};

let failures = 0;
async function run(lang) {
  const tx = T[lang];
  const page = await browser.newPage({ viewport: { width: 880, height: 860 } });
  const errors = [];
  page.on('pageerror', e => errors.push('pageerror: ' + e.message));
  page.on('console', m => { if (m.type() === 'error') errors.push('console: ' + m.text()); });
  const q = (extra = '') => `${base}/index.html?backend=mock&lang=${lang}&theme=light&state=empty&fast=1${extra}`;
  await page.goto(q());
  await page.waitForSelector('html[data-ready]');
  assert.equal(await page.getAttribute('html', 'data-ready'), '1', `${lang}: arranque`);
  assert.equal(await page.getAttribute('html', 'lang'), lang);

  // vacío
  assert.equal((await page.textContent('.dzone b')).trim(), tx.drop);

  // soltar algo que no es WAV: mensaje claro en el idioma, y sigue vacío
  await page.setInputFiles('#fsong', { name: 'cancion.mp3', mimeType: 'audio/mpeg', buffer: Buffer.from('x') });
  await page.waitForSelector('.dzone small.er');
  assert.match(await page.textContent('.dzone small.er'), tx.notWav, `${lang}: error no-WAV`);

  // una imagen sin canción cargada se ignora con una pista breve
  await page.setInputFiles('#fcov', []).catch(() => {});
  await page.evaluate(() => {
    const dt = new DataTransfer();
    dt.items.add(new File([new Uint8Array(8)], 'portada.png', { type: 'image/png' }));
    document.querySelector('#hero').dispatchEvent(new DragEvent('drop', { dataTransfer: dt, bubbles: true, cancelable: true }));
  });
  await page.waitForSelector('.dzone small.er');
  assert.match(await page.textContent('.dzone small.er'), lang === 'es' ? /Suelta primero la canción/ : /Drop the song first/);

  // soltar un WAV (HTML5 drop sobre la zona)
  await page.waitForSelector('.dzone b');
  await page.evaluate(() => {
    const dt = new DataTransfer();
    dt.items.add(new File([new Uint8Array(8)], 'Vera Alcón - Luz de Noviembre (master v3).wav', { type: 'audio/wav' }));
    document.querySelector('#hero').dispatchEvent(new DragEvent('drop', { dataTransfer: dt, bubbles: true, cancelable: true }));
  });
  await page.waitForSelector('#wave svg rect');
  assert.ok((await page.locator('#wave svg rect').count()) > 24, `${lang}: onda dibujada`);
  assert.equal(await page.inputValue('input[data-f=artist]'), 'Vera Alcón');
  assert.equal(await page.inputValue('input[data-f=title]'), 'Luz de Noviembre');
  assert.match((await page.textContent('.lpill')).trim(), tx.lufs, `${lang}: pastilla de loudness`);

  // el motor (mock) deshabilita Portada y Canvas: tooltip con el motivo
  for (const id of ['portada', 'canvas']) {
    const chip = page.locator(`.chip[data-id=${id}]`);
    assert.equal(await chip.getAttribute('aria-disabled'), 'true');
    assert.equal(await chip.getAttribute('aria-checked'), 'false');
    assert.ok((await chip.getAttribute('title') || '').length > 5, `${lang}: tooltip de ${id}`);
  }
  assert.match(await page.locator('.chip[data-id=canvas]').getAttribute('title'), lang === 'es' ? /Aún no disponible/ : /Not yet available/);
  const before = await page.locator('.chip.on').count();
  await page.click('.chip[data-id=canvas]', { force: true });
  assert.equal(await page.locator('.chip.on').count(), before, `${lang}: la ficha deshabilitada no se activa`);
  assert.equal((await page.textContent('.row2 .d1 .gh')).trim(), tx.dist);

  // alternar una ficha cambia el resumen
  const sum0 = (await page.textContent('[data-sum]')).trim();
  await page.click('.chip[data-id=amazon]');
  assert.notEqual((await page.textContent('[data-sum]')).trim(), sum0);
  await page.click('.chip[data-id=amazon]');
  assert.equal((await page.textContent('[data-sum]')).trim(), sum0);

  // 16 bit: el motor desactiva Apple (probado con otro archivo)
  // crear → listo
  await page.click('.go[data-act=create]');
  await page.waitForSelector('.go.busy');
  assert.match((await page.textContent('.go.busy')).trim(), new RegExp(tx.creating));
  await page.waitForSelector('.donebox');
  assert.match((await page.textContent('.donebox h1')).trim(), tx.done, `${lang}: título de listo`);
  assert.equal((await page.textContent('[data-act=open]')).trim(), tx.open);
  await page.click('[data-act=open]');
  await page.click('[data-act=again]');
  await page.waitForSelector('.dzone b');
  assert.equal((await page.textContent('.dzone b')).trim(), tx.drop);

  // mp3 sin lame: ficha deshabilitada con aviso
  await page.goto(q('&lame=0&state=normal'));
  await page.waitForSelector('html[data-ready]');
  assert.match(await page.locator('.chip[data-id=mp3]').getAttribute('title'), tx.lame);

  // un WAV de 16 bit desactiva Apple Music con el motivo del motor
  await page.goto(q('&state=empty'));
  await page.waitForSelector('html[data-ready]');
  await page.setInputFiles('#fsong', { name: 'Ana - Canción 16bit.wav', mimeType: 'audio/wav', buffer: Buffer.from('x') });
  await page.waitForSelector('#wave svg rect');
  assert.match(await page.locator('.chip[data-id=apple]').getAttribute('title'), lang === 'es' ? /24 bit/ : /24-bit/);

  assert.deepEqual(errors, [], `${lang}: errores de página`);
  await page.close();
}

for (const lang of ['es', 'en']) {
  try { await run(lang); console.log(`ok   ${lang}: vacío → soltar → crear → listo`); }
  catch (e) { failures++; console.error(`FAIL ${lang}:`, e.message); }
}
await browser.close();
server.close();
process.exit(failures ? 1 : 0);

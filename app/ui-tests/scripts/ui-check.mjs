// Recorre vacío → soltar → crear → listo con el mock, en es y en en, sin errores de página.
// Uso: node ui-tests/scripts/ui-check.mjs   (necesita Playwright + Chromium)
import assert from 'node:assert/strict';
import { serve, loadPlaywright } from './serve.mjs';

const { chromium } = await loadPlaywright();
const { server, base } = await serve();
const browser = await chromium.launch();

const T = {
  es: { drop: 'Suelta tu canción', create: 'Crear carpeta de entrega', creating: 'Creando carpeta…', preparing: 'Preparando…', open: 'Abrir carpeta', again: 'Nueva canción', lufs: /^−9,1 LUFS · pasa de 0 dB$/, done: /^Listo: \d+ archivos en Vera Alcón - Luz de Noviembre$/, notWav: /no es un WAV válido/, sidecar: /falta ffmpeg/i, noComponent: /no disponible/i, dist: 'Distribuidora', dropHint: 'Arrastra la portada', adm: /Apple Music: pasa de 0 dB al codificar/, openFail: /No se pudo abrir la carpeta/, legalFail: /No se pudo abrir el documento de licencias/, trademarks: /Deezer, YouTube y TuneCore son marcas de sus titulares\. master check no está afiliado ni avalado por ellos\./, chips: ['Spotify', 'Apple Music', 'Amazon Music', 'TIDAL', 'Deezer', 'YouTube Music', 'Estándar', 'MP3', 'Portada', 'Canvas', 'YouTube (vídeo)', 'Portada animada'] },
  en: { drop: 'Drop your song', create: 'Create delivery folder', creating: 'Creating folder…', preparing: 'Preparing…', open: 'Open folder', again: 'New song', lufs: /^-9\.1 LUFS · over 0 dB$/, done: /^Done: \d+ files in Vera Alcón - Luz de Noviembre$/, notWav: /not a valid WAV/, sidecar: /ffmpeg is missing/i, noComponent: /not available/i, dist: 'Distributor', dropHint: 'Drop the cover', adm: /Apple Music: clips when encoded/, openFail: /folder could not be opened/, legalFail: /licenses document could not be opened/, trademarks: /Deezer, YouTube and TuneCore are trademarks of their respective owners\. master check is not affiliated with or endorsed by them\./, chips: ['Spotify', 'Apple Music', 'Amazon Music', 'TIDAL', 'Deezer', 'YouTube Music', 'Standard', 'MP3', 'Cover', 'Canvas', 'YouTube (video)', 'Motion artwork'] }
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

  // el motor (mock) deshabilita las 4 fichas de material sin portada: tooltip con el motivo
  for (const id of ['portada', 'canvas', 'ytv', 'motion']) {
    const chip = page.locator(`.chip[data-id=${id}]`);
    assert.equal(await chip.getAttribute('aria-disabled'), 'true');
    assert.equal(await chip.getAttribute('aria-checked'), 'false');
    assert.ok((await chip.getAttribute('title') || '').length > 5, `${lang}: tooltip de ${id}`);
  }
  assert.match(await page.locator('.chip[data-id=canvas]').getAttribute('title'), lang === 'es' ? /Falta la portada/ : /The cover is missing/);
  // y lo insinúan: borde discontinuo y una pista corta («Arrastra la portada»), no solo el tooltip
  assert.equal(await page.locator('.chip.acc').count(), 4, `${lang}: 4 fichas esperan material`);
  assert.equal((await page.locator('.chip[data-id=canvas] .dh').textContent()).trim(), tx.dropHint, `${lang}: pista visible`);
  assert.equal(await page.locator('.chip[data-id=spotify] .dh').count(), 0);
  const before = await page.locator('.chip.on').count();
  await page.click('.chip[data-id=canvas]', { force: true });
  assert.equal(await page.locator('.chip.on').count(), before, `${lang}: la ficha deshabilitada no se activa`);
  assert.equal((await page.textContent('.row2 .d1 .gh')).trim(), tx.dist);

  // soltar un vídeo propio sobre Canvas: la ficha se habilita, muestra miniatura y × para quitarlo
  const dropOn = (sel, name, type) => page.evaluate(([s, n, ty]) => {
    const dt = new DataTransfer();
    dt.items.add(new File([new Uint8Array(8)], n, { type: ty }));
    document.querySelector(s).dispatchEvent(new DragEvent('drop', { dataTransfer: dt, bubbles: true, cancelable: true }));
  }, [sel, name, type]);
  await dropOn('.chipw[data-drop=canvas]', 'bucle.mp4', 'video/mp4');
  await page.waitForSelector('.chip[data-id=canvas].has-media');
  assert.equal(await page.locator('.chip[data-id=canvas]').getAttribute('aria-disabled'), null, `${lang}: Canvas habilitado con material propio`);
  assert.equal(await page.locator('.chip[data-id=canvas] .mthumb').count(), 1);
  // el punto de estado sigue visible junto a la miniatura y distingue encendida de apagada
  assert.equal(await page.locator('.chip[data-id=canvas] .sdot').count(), 1, `${lang}: punto de estado junto a la miniatura`);
  const dotLook = () => page.evaluate(() => {
    const c = document.querySelector('.chip[data-id=canvas]'), d = c.querySelector('.sdot'), cs = getComputedStyle(d), r = d.getBoundingClientRect();
    return { on: c.getAttribute('aria-checked'), bg: cs.backgroundColor, vis: cs.display !== 'none' && cs.opacity !== '0' && r.width >= 8 && r.height >= 8, op: getComputedStyle(c.querySelector('.mono')).opacity };
  });
  const dot1 = await dotLook();
  await page.click('.chip[data-id=canvas]');
  await page.waitForTimeout(450); // la transición del punto dura 250 ms
  const dot2 = await dotLook();
  assert.notEqual(dot1.on, dot2.on, `${lang}: la ficha cambia de estado`);
  assert.ok(dot1.vis && dot2.vis && dot1.op === '1' && dot2.op === '1', `${lang}: el punto de estado se ve en las dos posiciones ${JSON.stringify([dot1, dot2])}`);
  assert.notEqual(dot1.bg, dot2.bg, `${lang}: el punto distingue encendida de apagada ${JSON.stringify([dot1, dot2])}`);
  assert.match(await page.locator('.chip[data-id=canvas]').getAttribute('title'), /bucle\.mp4/);
  assert.equal(await page.locator('.chip[data-id=ytv]').getAttribute('aria-disabled'), 'true', `${lang}: el resto sigue sin portada`);
  assert.equal(await page.locator('.chip[data-id=canvas] .dh').count(), 0, `${lang}: con material propio ya no hay pista`);
  await page.click('[data-act=media-clear][data-mid=canvas]');
  await page.waitForSelector('.chip[data-id=canvas]:not(.has-media)');
  assert.equal(await page.locator('.chip[data-id=canvas]').getAttribute('aria-disabled'), 'true');

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
  assert.match((await page.textContent('.go.busy')).trim(), new RegExp(`${tx.creating}|${tx.preparing}`), `${lang}: crea o prepara`);
  await page.waitForSelector('.donebox');
  assert.match((await page.textContent('.donebox h1')).trim(), tx.done, `${lang}: título de listo`);
  assert.equal((await page.textContent('[data-act=open]')).trim(), tx.open);
  await page.click('[data-act=open]');
  await page.click('[data-act=again]');
  await page.waitForSelector('.dzone b');
  assert.equal((await page.textContent('.dzone b')).trim(), tx.drop);

  // default_on:false del motor: la ficha arranca apagada y «Lo necesario» la apaga de nuevo
  await page.goto(q('&off=tidal,apple_motion&state=normal'));
  await page.waitForSelector('html[data-ready]');
  assert.equal(await page.locator('.chip[data-id=tidal]').getAttribute('aria-checked'), 'false', `${lang}: tidal empieza apagada`);
  assert.equal(await page.locator('.chip[data-id=amazon]').getAttribute('aria-checked'), 'true');
  assert.equal(await page.locator('.chip[data-id=motion]').getAttribute('aria-checked'), 'false', `${lang}: apple motion apagada por defecto`);
  await page.click('.chip[data-id=tidal]');
  assert.equal(await page.locator('.chip[data-id=tidal]').getAttribute('aria-checked'), 'true');
  await page.click('[data-act=preset]');
  assert.equal(await page.locator('.chip[data-id=tidal]').getAttribute('aria-checked'), 'false', `${lang}: el preset apaga las fichas default_on:false`);

  // el AAC de la comprobación de Apple pasa de 0 dB: aviso de estado al terminar (no error)
  await page.goto(`${base}/index.html?backend=mock&lang=${lang}&theme=light&fast=1&state=done&adm=clip`);
  await page.waitForSelector('.donebox');
  assert.match((await page.textContent('.statusline')).trim(), tx.adm, `${lang}: aviso de ADM`);
  assert.equal(await page.locator('.warnline').count(), 0, `${lang}: no es un error`);

  // «Abrir carpeta» rechazado por el backend: aviso visible, no solo consola
  await page.goto(`${base}/index.html?backend=mock&lang=${lang}&theme=light&fast=1&state=done&fail=open`);
  await page.waitForSelector('.donebox');
  await page.click('[data-act=open]');
  await page.waitForSelector('.donebox .warnline');
  assert.match((await page.textContent('.donebox .warnline')).trim(), tx.openFail, `${lang}: aviso al fallar Abrir carpeta`);

  // «Licencias» rechazado: aviso en Acerca de
  await page.goto(`${base}/index.html?backend=mock&lang=${lang}&theme=light&fast=1&state=normal&fail=legal`);
  await page.waitForSelector('html[data-ready]');
  await page.click('#aboutBtn');
  await page.click('#pop-about [data-act=legal]');
  await page.waitForSelector('#pop-about .warnline');
  assert.match((await page.textContent('#pop-about .warnline')).trim(), tx.legalFail, `${lang}: aviso al fallar Licencias`);

  // Cancelar justo al crear (antes del primer evento): la exportación no termina en «Listo»
  await page.goto(`${base}/index.html?backend=mock&lang=${lang}&theme=light&fast=1&state=normal&fail=cancel_early`);
  await page.waitForSelector('html[data-ready]');
  /* el render de «creando» es síncrono: el botón Cancelar existe en el mismo turno, antes de cualquier evento del backend */
  await page.evaluate(() => {
    document.querySelector('.go[data-act=create]').click();
    document.querySelector('[data-act=cancel]').click();
  });
  await page.waitForSelector('.go[data-act=create]', { timeout: 15000 });
  assert.equal(await page.locator('.donebox').count(), 0, `${lang}: cancelar antes del primer evento cancela`);

  // Acerca de: versión, licencias y enlace al documento de terceros; Esc lo cierra
  await page.click('#aboutBtn');
  await page.waitForSelector('#pop-about.open');
  assert.equal(await page.getAttribute('#aboutBtn', 'aria-expanded'), 'true');
  const about = await page.textContent('#pop-about');
  assert.match(about, /0\.1\.0-mock/, `${lang}: versión`);
  assert.match(about, /FFmpeg/); assert.match(about, /LGPL/); assert.match(about, /LAME/);
  assert.match(about, /ffmpeg-sidecar-v7\.1\.5-3a21763/, `${lang}: release del componente`);
  await page.click('#pop-about [data-act=legal]');
  assert.equal(await page.evaluate(async () => (await import('./backend.js')).backend.legalOpened), 1, `${lang}: abre licencias`);
  await page.keyboard.press('Escape');
  assert.equal(await page.locator('#pop-about.open').count(), 0, `${lang}: Esc cierra Acerca de`);

  // sin ffmpeg: ficha deshabilitada con aviso
  await page.goto(q('&sidecar=0&state=normal'));
  await page.waitForSelector('html[data-ready]');
  assert.match(await page.locator('.chip[data-id=mp3]').getAttribute('title'), tx.sidecar);
  await page.click('#aboutBtn');
  await page.waitForSelector('#pop-about.open');
  assert.match(await page.textContent('#pop-about'), tx.noComponent, `${lang}: Acerca de sin componente`);
  await page.keyboard.press('Escape');
  assert.equal(await page.locator('.chip[data-id=ytv]').getAttribute('aria-disabled'), 'true');

  // un WAV de 16 bit desactiva Apple Music con el motivo del motor
  await page.goto(q('&state=empty'));
  await page.waitForSelector('html[data-ready]');
  await page.setInputFiles('#fsong', { name: 'Ana - Canción 16bit.wav', mimeType: 'audio/wav', buffer: Buffer.from('x') });
  await page.waitForSelector('#wave svg rect');
  assert.match(await page.locator('.chip[data-id=apple]').getAttribute('title'), lang === 'es' ? /24 bit/ : /24-bit/);

  // marcas de terceros (D-31): por defecto V1 «solo nombre» (punto de estado, sin glifos); V2 «gesto por grupo» con ?chips=gesto
  await page.goto(q('&state=normal'));
  await page.waitForSelector('html[data-ready]');
  assert.equal(await page.getAttribute('html', 'data-chip-style'), 'nombre', `${lang}: V1 por defecto`);
  const names = await page.$$eval('.chip .cn', els => els.map(e => e.textContent.trim()));
  assert.deepEqual(names, tx.chips, `${lang}: nombres oficiales de las fichas`);
  assert.equal(await page.locator('.chip .mono svg').count(), 0, `${lang}: V1 no lleva glifos`);
  assert.equal(await page.locator('.chip[data-id=spotify] .mono i.dot').count(), 1, `${lang}: V1 lleva el punto de estado`);
  assert.equal(await page.locator('.chip svg text').count(), 0, `${lang}: ningún glifo con texto de marca`);
  // punto de estado V1: anillo sólido y sin atenuar al estar apagado (contraste ≥ 3:1); deshabilitado sin outline cuadrado
  const p2 = await browser.newPage({ viewport: { width: 880, height: 860 } });
  await p2.goto(`${base}/index.html?backend=mock&lang=${lang}&theme=light&fast=1&state=normal`);
  await p2.waitForSelector('html[data-ready]');
  const off = await p2.evaluate(() => {
    const dot = document.querySelector('.chip[aria-checked=false]:not(.dis) .dot') || document.querySelector('.chip:not(.on):not(.dis) .dot');
    const dis = document.querySelector('.chip.dis:not(.has-media) .mono');
    const eff = el => { let o = 1; for (let e = el; e && e !== document.body; e = e.parentElement) o *= +getComputedStyle(e).opacity; return o; };
    return { dot: dot ? { style: getComputedStyle(dot).borderTopStyle, w: parseFloat(getComputedStyle(dot).borderTopWidth), op: eff(dot) } : null, disOutline: dis ? getComputedStyle(dis).outlineStyle : null, disDot: document.querySelector('.chip.dis .dot') ? { style: getComputedStyle(document.querySelector('.chip.dis .dot')).borderTopStyle, op: eff(document.querySelector('.chip.dis .dot')) } : null };
  });
  assert.ok(off.disDot, `${lang}: hay un punto en una ficha deshabilitada ${JSON.stringify(off)}`);
  assert.equal(off.disDot.style, 'solid', `${lang}: anillo deshabilitado sólido`);
  assert.ok(off.disDot.op >= 0.99, `${lang}: anillo deshabilitado sin atenuar (${off.disDot.op})`);
  assert.equal(off.disOutline, 'none', `${lang}: sin outline cuadrado en deshabilitado`);
  await p2.click('.chip[data-id=amazon]');
  const offDot = await p2.evaluate(() => { const d = document.querySelector('.chip[data-id=amazon] .dot'); let o = 1; for (let e = d; e && e !== document.body; e = e.parentElement) o *= +getComputedStyle(e).opacity; return { style: getComputedStyle(d).borderTopStyle, w: parseFloat(getComputedStyle(d).borderTopWidth), op: o }; });
  assert.equal(offDot.style, 'solid', `${lang}: anillo apagado sólido`);
  assert.ok(offDot.w <= 1.6 && offDot.op >= 0.99, `${lang}: anillo apagado fino y sin atenuar (${JSON.stringify(offDot)})`);
  await p2.close();
  await page.goto(q('&state=normal&chips=gesto'));
  await page.waitForSelector('html[data-ready]');
  assert.equal(await page.getAttribute('html', 'data-chip-style'), 'gesto');
  assert.equal(await page.locator('.chip[data-id=spotify] .mono svg').count(), 1, `${lang}: V2 lleva glifo`);
  assert.equal(await page.locator('.chip svg text').count(), 0);
  const kinds = await page.$$eval('.chip .mono svg', els => els.map(e => e.dataset.g));
  assert.deepEqual([...new Set(kinds)].sort(), ['distribuidora', 'extras', 'streaming'], `${lang}: tres gestos por grupo`);
  const box = await page.locator('.chip[data-id=spotify] .mono svg').boundingBox();
  assert.ok(Math.abs(box.width - 24) < 1.5 && Math.abs(box.height - 24) < 1.5, `${lang}: glifo de 24 px (${box.width}x${box.height})`);

  // «Acerca de» lleva el aviso de marcas
  await page.goto(q('&state=normal'));
  await page.waitForSelector('html[data-ready]');
  await page.click('#aboutBtn');
  await page.waitForSelector('#pop-about.open');
  assert.match(await page.textContent('#pop-about'), tx.trademarks, `${lang}: aviso de marcas`);

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

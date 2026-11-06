// Marcas de terceros (D-31, opción B): sin textos que impliquen aval, grafía oficial, avisos de marcas
// y conjuntos de claves idénticos en es/en (interfaz y motor).
import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync, readdirSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { join } from 'node:path';

const UI = fileURLToPath(new URL('../../ui/', import.meta.url));
const ENGINE = fileURLToPath(new URL('../../../engine/masterout-core/', import.meta.url));
const json = p => JSON.parse(readFileSync(p, 'utf8'));
const ui = { es: json(join(UI, 'i18n/es.json')), en: json(join(UI, 'i18n/en.json')) };
const eng = { es: json(join(ENGINE, 'i18n/es.json')), en: json(join(ENGINE, 'i18n/en.json')) };
const presets = json(join(ENGINE, 'presets.json'));

/* cadenas que implican aval, certificación o un programa ajeno (es/en) */
const FORBIDDEN = [
  /Digital Masters/i, /certificad[oa]/i, /certified/i, /aprobad[oa] por/i, /approved by/i, /\bReady\b/,
  /optimizad[oa] para/i, /optimized for/i, /Mastered for/i
];
const offenders = (label, text) => FORBIDDEN.filter(re => re.test(text)).map(re => `${label}: ${re}`);

test('i18n_ui_y_motor_tienen_claves_identicas_en_es_y_en', () => {
  assert.deepEqual(Object.keys(ui.es).sort(), Object.keys(ui.en).sort());
  assert.deepEqual(Object.keys(eng.es).sort(), Object.keys(eng.en).sort());
});

test('sin_literales_de_aval_en_ui_ni_motor', () => {
  const bad = [];
  for (const f of readdirSync(UI).filter(f => /\.(js|html|css)$/.test(f))) bad.push(...offenders(`ui/${f}`, readFileSync(join(UI, f), 'utf8')));
  for (const l of ['es', 'en']) {
    bad.push(...offenders(`ui/i18n/${l}`, Object.values(ui[l]).join('\n')));
    bad.push(...offenders(`motor/i18n/${l}`, Object.values(eng[l]).join('\n')));
  }
  bad.push(...offenders('presets.json', presets.map(p => `${p.name}\n${p.tag}`).join('\n')));
  assert.deepEqual(bad, []);
});

test('grafia_oficial_de_las_plataformas', () => {
  for (const l of ['es', 'en']) {
    assert.equal(ui[l]['chip.amazon'], 'Amazon Music');
    assert.equal(ui[l]['chip.tidal'], 'TIDAL');
    assert.equal(ui[l]['chip.long.tidal'], 'TIDAL');
    assert.equal(ui[l]['chip.ytm'], 'YouTube Music');
    assert.equal(eng[l]['preset.tidal'], 'TIDAL');
    assert.equal(eng[l]['preset.apple_adm'], 'Apple Music');
  }
  assert.equal(ui.es['chip.ytv'], 'YouTube (vídeo)');
  assert.equal(ui.en['chip.ytv'], 'YouTube (video)');
  assert.equal(presets.find(x => x.id === 'tidal').tag, 'TIDAL');
  assert.equal(presets.find(x => x.id === 'apple_motion').tag, 'Portada animada');
  assert.equal(eng.es['folder.tidal'], '04 TIDAL');
  assert.equal(eng.en['folder.tidal'], '04 TIDAL');
  assert.equal(eng.es['tag.motion'], 'Portada animada');
  assert.equal(eng.en['tag.motion'], 'Motion artwork');
  assert.equal(ui.es['chip.motion'], 'Portada animada');
  assert.equal(ui.en['chip.motion'], 'Motion artwork');
  assert.equal(ui.es['chip.long.motion'], 'Portada animada (Apple Music)');
  assert.equal(ui.en['chip.long.motion'], 'Motion artwork (Apple Music)');
  const p = Object.fromEntries(presets.map(x => [x.id, x]));
  assert.equal(p.apple_adm.name, 'Apple Music');
  assert.equal(p.apple_adm.tag, 'Apple Music');
  assert.equal(p.tidal.name, 'TIDAL');
});

test('aviso_de_marcas_en_acerca_de_y_en_el_motor', () => {
  assert.equal(ui.es['about.trademarks'], 'Spotify, Apple Music, Amazon Music, TIDAL, Deezer, YouTube y TuneCore son marcas de sus titulares. master check no está afiliado ni avalado por ellos.');
  assert.equal(ui.en['about.trademarks'], 'Spotify, Apple Music, Amazon Music, TIDAL, Deezer, YouTube and TuneCore are trademarks of their respective owners. master check is not affiliated with or endorsed by them.');
  assert.equal(eng.es['readme.trademarks'], ui.es['about.trademarks']);
  assert.equal(eng.en['readme.trademarks'], ui.en['about.trademarks']);
});

test('el_mock_no_habla_de_apple_digital_masters', () => {
  const src = readFileSync(join(UI, 'backend.js'), 'utf8');
  assert.match(src, /Apple Music pide un máster de 24 bit o más/);
  assert.match(src, /Apple Music asks for a 24-bit master or higher/);
  assert.match(src, /tag: 'Apple Music'/);
});

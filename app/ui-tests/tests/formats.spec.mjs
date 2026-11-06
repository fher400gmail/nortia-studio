// Los textos de «tipo no admitido» listan los formatos que la interfaz y el motor aceptan de verdad.
import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';

const read = rel => readFileSync(fileURLToPath(new URL(`../../${rel}`, import.meta.url)), 'utf8');
const js = read('ui/app.js');
const rust = read('../engine/masterout-core/src/media/mod.rs');

/* extensiones de MEDIA_RE (app.js): jpe?g → jpg, jpeg */
const mediaExts = () => {
  const m = js.match(/const MEDIA_RE = \/\\\.\(([^)]+)\)\$\/i;/);
  assert.ok(m, 'MEDIA_RE no encontrada en app.js');
  return m[1].split('|').flatMap(e => (e === 'jpe?g' ? ['jpg', 'jpeg'] : [e]));
};
const rustList = name => {
  const m = rust.match(new RegExp(`pub const ${name}: &\\[&str\\] = &\\[([^\\]]+)\\]`));
  assert.ok(m, `${name} no encontrada en media/mod.rs`);
  return [...m[1].matchAll(/"([a-z0-9]+)"/g)].map(x => x[1]);
};

test('la interfaz acepta exactamente las extensiones del motor para material propio', () => {
  assert.deepEqual([...mediaExts()].sort(), [...rustList('IMAGE_EXTS'), ...rustList('VIDEO_EXTS')].sort());
});

for (const lang of ['es', 'en']) {
  test(`media.bad_type (${lang}) nombra todos los formatos admitidos`, () => {
    const text = JSON.parse(read(`ui/i18n/${lang}.json`))['media.bad_type'].toLowerCase();
    for (const e of mediaExts()) {
      if (e === 'jpeg') continue; // «JPG» cubre las dos grafías
      assert.ok(text.includes(e), `${lang}: falta .${e} en «${text}»`);
    }
  });
  test(`cover.bad_type (${lang}) solo habla de JPG y PNG (la portada no admite WebP)`, () => {
    const text = JSON.parse(read(`ui/i18n/${lang}.json`))['cover.bad_type'].toLowerCase();
    assert.ok(text.includes('jpg') && text.includes('png') && !text.includes('webp'));
  });
}

test('la portada rechaza WebP: el crate image se compila sin esa feature', () => {
  assert.match(js, /if \(!\/\\\.\(png\|jpe\?g\)\$\/i\.test\(src\)\) return hint\(t\('cover\.bad_type'\)\)/);
  const cargo = read('../engine/masterout-core/Cargo.toml');
  assert.match(cargo, /image = \{[^}]*features = \["jpeg", "png"\]/);
});

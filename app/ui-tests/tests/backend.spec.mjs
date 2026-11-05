import test from 'node:test';
import assert from 'node:assert/strict';
import { createMock, detect, suggestFromName } from '../../ui/backend.js';
import { fmtLufs, fmtDbtp, fmtDuration, fmtRate, fmtSpec, fmtBytes, fmtNumber, shortPath } from '../../ui/format.js';
import { setDict, t, tn } from '../../ui/i18n.js';
import { isCancelled } from '../../ui/flow.js';
import { readFileSync } from 'node:fs';

const SAMPLE = '/m/Vera Alcón - Luz de Noviembre (master v3).wav';
const ALL = ['spotify', 'apple_adm', 'amazon', 'tidal', 'deezer', 'youtube_music', 'distributor_std', 'mp3_promo', 'cover', 'canvas'];
const meta = { artist: 'Vera Alcón', title: 'Luz de Noviembre' };

test('mock_analyze_returns_bins_and_master', async () => {
  const be = createMock({ fast: true });
  const { master, waveform } = await be.analyze(SAMPLE, 600);
  assert.equal(waveform.bins.length, 600);
  assert.ok(waveform.bins.every(v => v >= 0 && v <= 1));
  assert.equal(master.suggested_artist, 'Vera Alcón');
  assert.equal(master.suggested_title, 'Luz de Noviembre');
  assert.equal(master.integrated_lufs, -9.1);
  assert.equal(master.sample_rate, 96000);
});

test('mock_analyze_rejects_non_wav_with_localized_error', async () => {
  const es = createMock({ lang: 'es' });
  await assert.rejects(es.analyze('/m/cancion.mp3', 10), e => e.key === 'err.not_wav' && /WAV/.test(e.message) && /válido/.test(e.message));
  const en = createMock({ lang: 'en' });
  await assert.rejects(en.analyze('/m/song.mp3', 10), e => e.key === 'err.not_wav' && /valid WAV/.test(e.message));
  await assert.rejects(en.analyze('/m/song 192k.wav', 10), e => e.key === 'err.rate_over_96k');
});

test('mock_plan_disables_apple_for_16bit_and_unfinished_presets_with_reason', async () => {
  const be = createMock({ fast: true });
  const { master } = await be.analyze('/m/x 16bit.wav', 10);
  const plan = await be.plan(master, meta, ALL);
  const byId = Object.fromEntries(plan.jobs.map(j => [j.preset_id, j]));
  assert.equal(byId.apple_adm.enabled, false);
  assert.match(byId.apple_adm.disabled_reason, /24 bit/);
  assert.equal(byId.cover.enabled, false);
  assert.ok(byId.cover.disabled_reason);
  assert.equal(byId.spotify.enabled, true);
  assert.equal(byId.spotify.disabled_reason, null);
  assert.equal(plan.total_files, plan.jobs.filter(j => j.enabled).length + 4);
});

test('mock_plan_english_uses_english_contract', async () => {
  const be = createMock({ lang: 'en' });
  const { master } = await be.analyze(SAMPLE, 10);
  const plan = await be.plan(master, { ...meta, version: 'Radio Edit' }, ['distributor_std']);
  assert.equal(plan.delivery_folder, 'Vera Alcón - Luz de Noviembre (Radio Edit)');
  assert.equal(plan.jobs[0].subfolder, '07 Distributor');
  assert.match(plan.jobs[0].file_name, /\[Distributor\]/);
});

test('mock_export_emits_progress_then_finished', async () => {
  const be = createMock({ fast: true });
  const { master } = await be.analyze(SAMPLE, 10);
  const ids = ['spotify', 'deezer', 'distributor_std'];
  const events = [];
  const d = await be.export({ master_path: master.path, out_root: '/out', meta, active_ids: ids }, e => events.push(e));
  const kinds = events.map(e => e.kind);
  assert.equal(kinds[0], 'analyzed');
  assert.equal(kinds[1], 'planned');
  assert.equal(kinds.at(-1), 'finished');
  assert.deepEqual(events.filter(e => e.kind === 'job_started').map(e => e.preset_id), ids);
  assert.equal(events.filter(e => e.kind === 'job_done').length, 3);
  assert.ok(events.some(e => e.kind === 'job_progress' && e.fraction > 0 && e.fraction < 1));
  assert.equal(d.files.length, 3 + 4);
  assert.deepEqual(d.failed, []);
  assert.deepEqual(events.at(-1).delivery, d);
});

test('mock_cancel_stops_export', async () => {
  const be = createMock({ fast: true });
  const events = [];
  const p = be.export({ master_path: SAMPLE, out_root: '/out', meta, active_ids: ['spotify', 'amazon', 'tidal'] }, e => { events.push(e.kind); if (e.kind === 'job_done') be.cancelExport(); });
  const d = await p;
  assert.ok(events.includes('cancelled'));
  assert.ok(!events.includes('finished'));
  assert.equal(d.files.length, 1);
});

test('detect_picks_mock_without_tauri_and_honours_query', () => {
  assert.equal(detect({ location: { search: '' } }).kind, 'mock');
  assert.equal(detect({ location: { search: '?backend=mock' }, __TAURI_INTERNALS__: {} }).kind, 'mock');
  assert.equal(detect({ location: { search: '' }, __TAURI_INTERNALS__: {} }).kind, 'tauri');
  assert.equal(detect({ location: { search: '?backend=tauri' } }).kind, 'tauri');
});

test('tauri_backend_calls_commands_with_exact_names', async () => {
  const calls = [];
  const listeners = {};
  const g = {
    location: { search: '' }, __TAURI_INTERNALS__: {},
    __TAURI__: {
      core: { invoke: async (c, a) => { calls.push([c, a]); if (c === 'export') listeners['mastercheck://progress']({ payload: { kind: 'finished' } }); return null; }, convertFileSrc: p => `asset://${p}` },
      event: { listen: async (n, cb) => { listeners[n] = cb; return () => { delete listeners[n]; }; } }
    }
  };
  const be = detect(g);
  await be.analyze('/a.wav', 600);
  await be.plan({ x: 1 }, { artist: 'a' }, ['spotify']);
  const seen = [];
  await be.export({ master_path: '/a.wav' }, e => seen.push(e.kind));
  await be.cancelExport(); await be.pickFolder(); await be.openPath('/x'); await be.systemLang(); await be.appInfo();
  assert.deepEqual(calls.map(c => c[0]), ['analyze', 'plan', 'export', 'cancel_export', 'pick_folder', 'open_path', 'system_lang', 'app_info']);
  assert.deepEqual(calls[0][1], { path: '/a.wav', bins: 600 });
  assert.deepEqual(calls[1][1], { master: { x: 1 }, meta: { artist: 'a' }, activeIds: ['spotify'] });
  assert.deepEqual(calls[2][1], { req: { master_path: '/a.wav' } });
  assert.deepEqual(seen, ['finished']);
  assert.equal(Object.keys(listeners).length, 0, 'se desuscribe al terminar');
});

test('format_lufs_es_uses_comma_and_minus', () => {
  assert.equal(fmtLufs(-9.1, 'es'), '\u22129,1 LUFS');
});
test('format_lufs_en_uses_dot', () => {
  assert.equal(fmtLufs(-9.1, 'en'), '-9.1 LUFS');
});
test('format_other_numbers', () => {
  assert.equal(fmtDbtp(1.2, 'es'), '+1,2 dBTP');
  assert.equal(fmtDbtp(-0.04, 'en'), '0.0 dBTP');
  assert.equal(fmtDuration(232), '3:52');
  assert.equal(fmtRate(44100, 'es'), '44,1 kHz');
  assert.equal(fmtRate(96000, 'en'), '96 kHz');
  assert.equal(fmtBytes(74_100_000, 'es'), '74,1 MB');
  assert.equal(fmtBytes(500, 'en'), '<1 KB');
  assert.equal(fmtNumber(1234.5, 0, 'en'), '1235');
  assert.equal(fmtSpec({ container: 'flac', bits: 24, sample_rate: 96000 }, 'es'), 'FLAC · 24 bit · 96 kHz');
  assert.equal(fmtSpec({ container: 'mp3', bits: 0, sample_rate: 44100 }, 'es'), 'MP3 · 320 kbps');
  assert.equal(shortPath('/Users/vera/Desktop/Entregas'), 'Desktop/Entregas');
  assert.equal(shortPath('C:\\Users\\vera\\Desktop\\Entregas'), 'Desktop/Entregas');
});

test('suggest_from_name', () => {
  assert.deepEqual(suggestFromName('/x/Solo Titulo (final mix).wav'), { artist: null, title: 'Solo Titulo' });
});

test('i18n_files_have_same_keys_and_plurals', () => {
  const es = JSON.parse(readFileSync(new URL('../../ui/i18n/es.json', import.meta.url)));
  const en = JSON.parse(readFileSync(new URL('../../ui/i18n/en.json', import.meta.url)));
  assert.deepEqual(Object.keys(es).sort(), Object.keys(en).sort());
  setDict('es', es);
  assert.equal(tn('loud.over', 1), '1 vez');
  assert.equal(tn('loud.over', 3), '3 veces');
  assert.equal(t('sum', { n: 8, size: '1 MB' }), '8 archivos · 1 MB');
});

test('cancel_race_delivery_flag_wins_when_event_never_arrives', async () => {
  // la promesa se resuelve sin que la interfaz haya visto el evento `cancelled`
  const be = createMock({ fast: true, dropCancelEvent: true });
  let flag = false;
  const kinds = [];
  const d = await be.export({ master_path: SAMPLE, out_root: '/out', meta, active_ids: ['spotify', 'amazon'] }, e => { kinds.push(e.kind); if (e.kind === 'cancelled') flag = true; if (e.kind === 'job_done') be.cancelExport(); });
  assert.ok(!kinds.includes('cancelled'));
  assert.equal(flag, false);
  assert.equal(d.cancelled, true);
  assert.equal(isCancelled(flag, d), true, 'nunca se dibuja "Listo" para una entrega cancelada');
  assert.equal(isCancelled(true, { cancelled: false }), true);
  assert.equal(isCancelled(false, { cancelled: false }), false);
  assert.equal(isCancelled(false, undefined), false);
});

test('mock_delivery_has_full_dto_shape', async () => {
  const be = createMock({ fast: true });
  const d = await be.export({ master_path: SAMPLE, out_root: '/out', meta, active_ids: ['spotify'] }, () => {});
  assert.equal(d.cancelled, false);
  assert.deepEqual(d.failed_keys, []);
  assert.deepEqual(d.failed_details, []);
});

test('tauri_allow_asset_command', async () => {
  const calls = [];
  const be = detect({ location: { search: '' }, __TAURI_INTERNALS__: {}, __TAURI__: { core: { invoke: async (c, a) => { calls.push([c, a]); } } } });
  await be.allowAsset('/x/cover.png');
  assert.deepEqual(calls, [['allow_asset', { path: '/x/cover.png' }]]);
});

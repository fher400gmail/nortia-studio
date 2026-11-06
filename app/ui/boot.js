/* antes del primer pintado: tema, sistema y entorno desde querystring / sistema (script clásico) */
(function () {
  var q = new URLSearchParams(location.search), d = document.documentElement;
  d.dataset.theme = q.get('theme') || (matchMedia('(prefers-color-scheme:dark)').matches ? 'dark' : 'light');
  d.dataset.os = q.get('os') || (/Win/.test(navigator.platform) ? 'win' : 'mac');
  /* fichas: «nombre» (V1, por defecto) o «gesto» (V2: un gesto propio por grupo), solo por querystring */
  d.dataset.chipStyle = q.get('chips') === 'gesto' ? 'gesto' : 'nombre';
  d.dataset.native = String(!!(window.__TAURI_INTERNALS__ || window.__TAURI__) && q.get('backend') !== 'mock');
})();

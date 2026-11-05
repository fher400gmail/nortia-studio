/* ===== núcleo compartido: datos, formato, árbol en vivo, exportación simulada ===== */
const MO={subs:[],listeners:[]};
const Q=new URLSearchParams(location.search);
const STILL=Q.has('still');
if(STILL)document.body.classList.add('still');
const nf=(n,d=1)=>(n<0?'−':'')+Math.abs(n).toFixed(d).replace('.',',');
const esc=s=>String(s).replace(/&/g,'&amp;').replace(/</g,'&lt;');
const ART='Vera Alcón',TIT='Luz de Noviembre',PRE=ART+' - '+TIT+' ';
const M={file:'Luz de Noviembre - master v7.wav',fmt:'WAV',bits:24,khz:96,ch:'Estéreo',dur:'3:52',size:'133,6 MB',lufs:-9.1,lra:5.8,tp:1.2,clips:3};
const IC={
 folder:'<svg class="ic" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7" stroke-linejoin="round"><path d="M3 7a2 2 0 0 1 2-2h4l2 2h8a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/></svg>',
 file:'<svg class="ic f" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linejoin="round"><path d="M6 3h8l5 5v13H6z"/><path d="M14 3v5h5"/></svg>',
 check:'<svg class="ic" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.6" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12.5l4.5 4.5L19 7.5"/></svg>',
 play:'<svg class="ic" viewBox="0 0 24 24" fill="currentColor"><path d="M7 4.5v15l12-7.5z"/></svg>',
 chev:'<svg class="ic" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M9 6l6 6-6 6"/></svg>',
 down:'<svg class="ic" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M6 9l6 6 6-6"/></svg>',
 left:'<svg class="ic" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M15 6l-6 6 6 6"/></svg>',
 moon:'<svg class="ic" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linejoin="round"><path d="M20 14.5A8 8 0 0 1 9.5 4 8 8 0 1 0 20 14.5z"/></svg>',
 sun:'<svg class="ic" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round"><circle cx="12" cy="12" r="4"/><path d="M12 2.5v2.2M12 19.3v2.2M2.5 12h2.2M19.3 12h2.2M5.3 5.3l1.6 1.6M17.1 17.1l1.6 1.6M5.3 18.7l1.6-1.6M17.1 6.9l1.6-1.6"/></svg>',
 pdir:'<svg class="ic" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"><path d="M12 5v14M5 12l7 7 7-7" transform="rotate(-90 12 12)"/></svg>',
 ok:'<svg viewBox="0 0 16 16"><circle cx="8" cy="8" r="7" fill="currentColor"/><path d="M4.6 8.3l2.3 2.3 4.5-4.7" fill="none" stroke="var(--bg)" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"/></svg>',
 wn:'<svg viewBox="0 0 16 16"><path d="M8 1.6L15 14H1z" fill="currentColor" stroke="currentColor" stroke-width="1.4" stroke-linejoin="round"/><path d="M8 6v4M8 11.6v.4" stroke="var(--bg)" stroke-width="1.7" stroke-linecap="round"/></svg>',
 er:'<svg viewBox="0 0 16 16"><circle cx="8" cy="8" r="7" fill="currentColor"/><path d="M5.4 5.4l5.2 5.2M10.6 5.4l-5.2 5.2" stroke="var(--bg)" stroke-width="1.8" stroke-linecap="round"/></svg>',
 plus:'<svg class="ic" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"><path d="M12 5v14M5 12h14"/></svg>',
 reveal:'<svg class="ic" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"><path d="M14 4h6v6M20 4l-9 9M18 14v5a1 1 0 0 1-1 1H5a1 1 0 0 1-1-1V7a1 1 0 0 1 1-1h5"/></svg>'
};
const SEVL={ok:'Cumple',wn:'Aviso',er:'No cumple'};
const sev=s=>`<span class="sev ${s}">${IC[s]}${SEVL[s]}</span>`;
/* norte marcado: rombo vertical 20×60 en viewBox 64, hueco de 2 u en la cintura */
const sym=(h=22)=>`<svg width="${h*20/64}" height="${h}" viewBox="22 2 20 60" aria-hidden="true" style="flex:none"><path d="M32 2L42 31H22Z" fill="#BF7F12"/><path d="M22 33H42L32 62Z" fill="var(--sym-s)"/></svg>`;
const wordmark=(sz=15)=>`<span class="wm" style="font-size:${sz}px">${sym(sz*1.5)}<span style="display:inline-flex;gap:.28em"><b>nortia.</b><span>studio</span></span></span>`;
function titlebar(extra=''){return `<div class="tb"><div class="lights"><i></i><i></i><i></i></div>${wordmark(15)}<span class="appn">Master Out</span><div class="sp"></div>${extra}<button class="ibtn" id="themeBtn" aria-label="Cambiar tema" title="Claro / oscuro"></button></div>`}
function themeInit(){const b=document.getElementById('themeBtn');const set=()=>{b.innerHTML=document.documentElement.dataset.theme==='dark'?IC.sun:IC.moon};set();
 b.onclick=()=>{const d=document.documentElement;d.dataset.theme=d.dataset.theme==='dark'?'light':'dark';set();const u=new URL(location);u.searchParams.set('theme',d.dataset.theme);history.replaceState(0,'',u)}}
/* ---------- destinos (datos de ejemplo) ---------- */
const Fs=(tag,mb)=>({tag,mb});
const D=[
 {id:'spotify',n:'Spotify',m:'Sp',g:'Streaming',spec:'FLAC · 24 bit · 96 kHz',sev:'er',why:'TP +1,2 dBTP; Spotify recomienda ≤ −1',adv:'El máster llega a +1,2 dBTP. Spotify recomienda ≤ −1 dBTP para que la recodificación no distorsione. Master Out no cambia el audio: si quieres corregirlo, hazlo en tu DAW.',fo:'01 Spotify',f:[Fs('[Spotify] 24bit-96kHz.flac',74.1)]},
 {id:'apple',n:'Apple Music',m:'Ap',g:'Streaming',spec:'WAV · 24 bit · 96 kHz · Digital Masters',sev:'er',why:'afclip: 1.284 muestras con clipping',adv:'La validación oficial (afconvert + afclip) encuentra 1.284 muestras con clipping en la versión AAC. Apple Digital Masters las rechazaría.',fo:'02 Apple Music',f:[Fs('[Apple Digital Masters] 24bit-96kHz.wav',133.6)]},
 {id:'amazon',n:'Amazon Music',m:'Am',g:'Streaming',spec:'FLAC · 24 bit · 96 kHz',sev:'wn',why:'TP +1,2 dBTP; recomendación ≤ −2',adv:'Amazon recomienda dejar margen de true peak. Es una recomendación, no un rechazo.',fo:'03 Amazon Music',f:[Fs('[Amazon Music] 24bit-96kHz.flac',74.1)]},
 {id:'tidal',n:'Tidal',m:'Ti',g:'Streaming',spec:'FLAC · 24 bit · 96 kHz',sev:'ok',why:'Cumple · resolución nativa',adv:'Se entrega en la resolución nativa del máster, sin conversión. Tidal solo ajusta el volumen de reproducción.',fo:'04 Tidal',f:[Fs('[Tidal] 24bit-96kHz.flac',74.1)]},
 {id:'deezer',n:'Deezer',m:'De',g:'Streaming',spec:'WAV · 16 bit · 44,1 kHz',sev:'wn',why:'TP +1,2 dBTP: puede recortar a 16 bit',adv:'Al pasar de 24 a 16 bit se aplica dither TPDF. Con picos sobre 0 dBTP el remuestreo puede recortar.',fo:'05 Deezer',f:[Fs('[Deezer] 16bit-44kHz.wav',40.9)]},
 {id:'ytm',n:'YouTube Music',m:'YM',g:'Streaming',spec:'WAV · 24 bit · 48 kHz',sev:'ok',why:'Cumple · YouTube solo baja el volumen',adv:'YouTube nunca sube el volumen; con −9,1 LUFS lo bajará al normalizar.',fo:'06 YouTube',f:[Fs('[YouTube Music] 24bit-48kHz.wav',66.8)]},
 {id:'dist',n:'Distribuidora estándar',m:'Di',g:'Distribuidoras',spec:'WAV · 16 bit · 44,1 kHz',sev:'wn',why:'TP > 0 dBTP: la distribuidora recodifica',adv:'La mayoría de distribuidoras recodifican a MP3/AAC para algunas tiendas; un pico sobre 0 dBTP puede recortar.',fo:'07 Distribuidora',f:[Fs('[Distribuidora] 16bit-44kHz.wav',40.9)]},
 {id:'disthr',n:'Distribuidora Hi-Res',m:'DH',g:'Distribuidoras',spec:'WAV · 24 bit · 96 kHz',sev:'ok',why:'Cumple · resolución nativa',adv:'Entrega 24 bit en la resolución nativa del máster.',fo:'07 Distribuidora',f:[Fs('[Distribuidora Hi-Res] 24bit-96kHz.wav',133.6)]},
 {id:'tunecore',n:'TuneCore',m:'Tc',g:'Distribuidoras',spec:'WAV · 24 bit · 192 kHz · sin metadatos',sev:'wn',why:'Upsampling 96 → 192 kHz, sin ganancia',adv:'TuneCore pide 24/192. Subir de 96 a 192 kHz se permite, pero no mejora la calidad: el README lo marca.',fo:'07 Distribuidora',f:[Fs('[TuneCore] 24bit-192kHz.wav',267.2)]},
 {id:'mp3',n:'MP3 320',m:'MP',g:'Promo',spec:'MP3 · 320 kbps CBR',sev:'wn',why:'TP +1,2 dBTP: el MP3 puede recortar',adv:'El codificador MP3 añade picos entre muestras; con +1,2 dBTP de partida es probable que recorte.',fo:'08 Promo',f:[Fs('[MP3 320].mp3',9.3)]},
 {id:'ytv',n:'YouTube vídeo',m:'YV',g:'Vídeo e imagen',spec:'MP4 · 1080p · imagen fija',sev:'ok',why:'Cumple · portada sobre fondo desenfocado',adv:'Vídeo H.264 con la portada centrada y audio AAC a 48 kHz.',fo:'06 YouTube',f:[Fs('[YouTube] 1080p.mp4',6.8)]},
 {id:'canvas',n:'Spotify Canvas',m:'Ca',g:'Vídeo e imagen',spec:'MP4 · 1080×1920 · 6 s · sin audio',sev:'ok',why:'Cumple · recorte 9:16 de la portada',adv:'Recorte 9:16 de la portada con zoom lento de 6 s.',fo:'09 Imagen',f:[Fs('[Canvas] 1080x1920.mp4',3.2)]},
 {id:'portada',n:'Portada',m:'Po',g:'Vídeo e imagen',spec:'JPG + PNG · 3000×3000',sev:'ok',why:'Cumple · 3000×3000 · sRGB',adv:'La portada cumple tamaño, formato cuadrado y perfil sRGB.',fo:'09 Imagen',f:[Fs('[Portada] 3000x3000.jpg',1.9),Fs('[Portada] 3000x3000.png',6.4)]}
];
const GROUPS=['Streaming','Distribuidoras','Promo','Vídeo e imagen'];
const DOCS=[['README.txt',.003],['informe-loudness.txt',.004],['ficha-lanzamiento.csv',.001],['ficha-lanzamiento.txt',.002]];
MO.on=new Set(['spotify','apple','tidal','deezer','dist','mp3','portada']);
MO.byId=Object.fromEntries(D.map(d=>[d.id,d]));
MO.exp={mode:'idle',pct:{}};            // idle | run | done
MO.t0=0;MO.dur=8200;
const sizeStr=mb=>mb<0.1?'<1 KB':mb<1?nf(mb*1000,0)+' KB':nf(mb,1)+' MB';
MO.files=function(){ // lista plana ordenada por carpeta
  const out=[];D.filter(d=>MO.on.has(d.id)).forEach(d=>d.f.forEach((f,i)=>out.push({k:d.id+':'+i,fo:d.fo,tag:f.tag,mb:f.mb,d})));
  out.sort((a,b)=>a.fo.localeCompare(b.fo)||D.indexOf(a.d)-D.indexOf(b.d));
  DOCS.forEach((x,i)=>out.push({k:'doc:'+i,fo:'',doc:x[0],mb:x[1]}));
  return out};
MO.stats=function(){const f=MO.files();const au=f.filter(x=>!x.doc);const act=D.filter(d=>MO.on.has(d.id));
  return{n:au.length,docs:DOCS.length,mb:f.reduce((s,x)=>s+x.mb,0),dest:act.length,er:act.filter(d=>d.sev==='er').length,wn:act.filter(d=>d.sev==='wn').length}};
const fileRow=(x,ind)=>{
  const e=MO.exp,p=e.pct[x.k]??0;let rs;
  if(e.mode==='idle')rs=`<span class="sz">${sizeStr(x.mb)}</span>`;
  else if(p>=100)rs=`<span class="dn">${IC.check}</span><span class="sz">${sizeStr(x.mb)}</span>`;
  else if(p>0)rs=`<span class="pc" data-pc="${x.k}">${Math.round(p)} %</span>`;
  else rs=`<span class="q">En cola</span>`;
  const nm=x.doc?`<span class="rest" style="color:var(--tx2)">${x.doc}</span>`:`<span class="pre">${esc(PRE)}</span><span class="tag">${esc(x.tag.slice(0,x.tag.indexOf(']')+1))}</span><span class="rest">${esc(x.tag.slice(x.tag.indexOf(']')+1))}</span>`;
  return `<div class="tr ${x.doc?'doc':''} ${p>0&&p<100&&e.mode!=='idle'?'rp':''}" data-k="${x.k}" style="padding-left:${ind}px;--p:${p}%">${IC.file}<div class="nm">${nm}</div>${rs}</div>`};
MO.treeHTML=function(){
  const f=MO.files();let h=`<div class="tr dir" data-k="root">${IC.folder}<div class="nm">${esc(ART+' - '+TIT)}/</div></div>`;
  let last=null;
  f.filter(x=>!x.doc).forEach(x=>{if(x.fo!==last){last=x.fo;h+=`<div class="tr dir" data-k="fo:${x.fo}" style="padding-left:16px">${IC.folder}<div class="nm">${x.fo}/</div></div>`}h+=fileRow(x,30)});
  f.filter(x=>x.doc).forEach(x=>h+=fileRow(x,18));
  return h};
MO.mountTree=function(el){
  let prev=new Set();
  const draw=()=>{const html=MO.treeHTML();el.innerHTML=html;
    const keys=new Set([...el.querySelectorAll('[data-k]')].map(n=>n.dataset.k));
    if(prev.size&&!STILL)el.querySelectorAll('[data-k]').forEach(n=>{if(!prev.has(n.dataset.k))n.classList.add('in')});
    prev=keys;
    el.querySelectorAll('.tr:not(.dir)').forEach(r=>r.onclick=()=>{el.querySelectorAll('.sel').forEach(s=>s.classList.remove('sel'));r.classList.add('sel')})};
  draw();MO.subs.push(draw)};
MO.refresh=()=>MO.subs.forEach(f=>f());
MO.toggle=id=>{MO.on.has(id)?MO.on.delete(id):MO.on.add(id);MO.refresh()};
/* ---------- exportación simulada ---------- */
MO.snapshot=function(){ // estado congelado "exportando"
  const pr={'spotify:0':100,'apple:0':64,'tidal:0':88,'deezer:0':100,'dist:0':31,'mp3:0':0,'portada:0':100,'portada:1':52};
  MO.files().forEach(x=>MO.exp.pct[x.k]=pr[x.k]??0);MO.exp.mode='run';MO.exp.snap=true};
MO.overall=function(){const f=MO.files();const t=f.reduce((s,x)=>s+x.mb,0);return f.reduce((s,x)=>s+x.mb*(MO.exp.pct[x.k]??0)/100,0)/t*100};
MO.start=function(){
  const f=MO.files();MO.exp={mode:'run',pct:{}};MO.t0=performance.now();
  const sp={};f.forEach((x,i)=>sp[x.k]={d:x.doc?.25:.35+Math.min(x.mb,140)/140*.55,s:x.doc?.7:(i%3)*.05});
  MO.refresh();
  const tick=()=>{const t=(performance.now()-MO.t0)/MO.dur;
    f.forEach(x=>{const a=sp[x.k];const u=x.doc?Math.max(0,(t-.75)/a.d):Math.max(0,(t-a.s)/a.d);MO.exp.pct[x.k]=Math.min(100,u*100)});
    MO.tickUI();
    if(f.every(x=>MO.exp.pct[x.k]>=100)){MO.exp.mode='done';MO.exp.secs=(performance.now()-MO.t0)/1000;MO.refresh();MO.listeners.forEach(l=>l('done'));return}
    requestAnimationFrame(tick)};
  requestAnimationFrame(tick);MO.listeners.forEach(l=>l('run'))};
let _lastFull=0;
MO.tickUI=function(){
  document.querySelectorAll('.tr[data-k]').forEach(n=>{const v=MO.exp.pct[n.dataset.k];if(v!=null)n.style.setProperty('--p',v+'%')});
  document.querySelectorAll('[data-pc]').forEach(n=>n.textContent=Math.round(MO.exp.pct[n.dataset.pc]||0)+' %');
  const now=performance.now();if(now-_lastFull>260){_lastFull=now;MO.refresh()}
  const o=MO.overall();document.querySelectorAll('[data-total]').forEach(n=>n.style.width=o+'%');
  document.querySelectorAll('[data-totalpc]').forEach(n=>n.textContent=Math.round(o)+' %');
  document.querySelectorAll('[data-eta]').forEach(n=>n.textContent=MO.eta())};
MO.eta=()=>{const o=MO.overall()/100;const s=Math.max(1,Math.round(MO.dur/1000*(1-o)));return 'Faltan '+s+' s'};
MO.confetti=function(host,n=22){if(STILL||!host)return;const c=document.createElement('div');c.className='conf';
  const cols=['#BF7F12','var(--sym-s)','var(--st)'];
  for(let i=0;i<n;i++){const e=document.createElement('i');e.style.left=(6+Math.random()*88)+'%';e.style.background=cols[i%3];e.style.animationDelay=(Math.random()*.7)+'s';e.style.opacity=.9;e.style.transform='scale('+(.6+Math.random()*.7)+')';c.appendChild(e)}
  host.appendChild(c);setTimeout(()=>c.remove(),4200)};
MO.doneMark=(sz=64)=>`<svg class="markdone" width="${sz*20/64*1.0}" height="${sz}" viewBox="22 2 20 60"><path class="t" d="M32 2L42 31H22Z" fill="#BF7F12"/><path class="b" d="M22 33H42L32 62Z" fill="var(--sym-s)"/></svg>`;
/* ---------- forma de onda ---------- */
function rng(a){return function(){a|=0;a=a+0x6D2B79F5|0;let t=Math.imul(a^a>>>15,1|a);t=t+Math.imul(t^t>>>7,61|t)^t;return((t^t>>>14)>>>0)/4294967296}}
const ENV=[[0,.1],[.05,.18],[.08,.5],[.28,.56],[.30,.88],[.5,.86],[.52,.5],[.63,.55],[.65,.94],[.85,.92],[.87,.5],[1,.08]];
const env=t=>{for(let i=0;i<ENV.length-1;i++){const[a,x]=ENV[i],[b,y]=ENV[i+1];if(t>=a&&t<=b)return x+(y-x)*(t-a)/(b-a)}return .1};
const CLIPS=[.312,.668,.745];
function waveSVG(w,h,n,opt={}){
  const r=rng(opt.seed||7),bw=w/n,g=bw*(opt.gap??.32);let o='';
  for(let i=0;i<n;i++){const t=(i+.5)/n;let a=Math.min(1,env(t)*(.62+.38*r()));
    const cl=opt.clips&&CLIPS.some(c=>Math.abs(t-c)<.55/n*1.2);if(cl)a=1;
    const bh=Math.max(3,a*h),cls=cl?'c':(opt.prog!=null&&t<=opt.prog?'p':'');
    o+=`<rect class="${cls}" x="${(i*bw).toFixed(1)}" y="${((h-bh)/2).toFixed(1)}" width="${(bw-g).toFixed(1)}" height="${bh.toFixed(1)}" rx="${((bw-g)/2).toFixed(1)}"/>`}
  return `<svg class="wv" width="${w}" height="${h}" viewBox="0 0 ${w} ${h}" aria-hidden="true">${o}</svg>`}
/* medidores */
function lufsMeter(w){const pos=v=>(v+24)/20*100;
  return `<div class="meter"><div class="mh"><span>Loudness integrado</span><b>${nf(M.lufs)} LUFS</b></div>
  <div class="mtrack"><i class="fill" style="width:${pos(M.lufs)}%"></i><i class="ref" style="left:${pos(-14)}%"></i></div>
  <div class="mscale"><span style="left:0">−24</span><span style="left:${pos(-14)}%">−14 streaming</span><span style="left:${pos(-9.1)}%;display:none"></span><span style="left:100%">−4</span></div></div>`}
function tpMeter(){const pos=v=>(v+6)/8*100;
  return `<div class="meter"><div class="mh"><span>True peak máximo</span><b class="er">+${nf(M.tp)} dBTP</b></div>
  <div class="mtrack"><i class="zone" style="width:${100-pos(0)}%"></i><i class="fill er" style="width:${pos(M.tp)}%"></i><i class="ref" style="left:${pos(-1)}%"></i></div>
  <div class="mscale"><span style="left:0">−6</span><span style="left:${pos(-1)}%">−1</span><span style="left:${pos(0)}%">0</span><span style="left:100%">+2</span></div></div>`}
function cover(s){return `<svg width="${s}" height="${s}" viewBox="0 0 100 100" role="img" aria-label="Portada" style="border-radius:${Math.max(6,s/16)}px;display:block;flex:none"><rect width="100" height="100" fill="#1B3A4D"/><rect y="60" width="100" height="40" fill="#12293A"/><circle cx="62" cy="42" r="19" fill="#EBDDC3"/><rect y="58" width="100" height="2.4" fill="#EBDDC3" opacity=".6"/><rect x="12" y="68" width="52" height="2.2" fill="#EBDDC3" opacity=".35"/><rect x="24" y="75" width="38" height="2.2" fill="#EBDDC3" opacity=".25"/><rect x="34" y="82" width="22" height="2.2" fill="#EBDDC3" opacity=".18"/></svg>`}
function sw(on,id){return `<button class="sw" role="switch" aria-checked="${on}" aria-label="Activar ${id}"></button>`}

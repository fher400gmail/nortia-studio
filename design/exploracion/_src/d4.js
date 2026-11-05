/* ===== D4: estado y comportamiento comunes ===== */
const ALL4=['spotify','apple','amazon','tidal','deezer','ytm','dist','mp3','portada','canvas'];
MO.on=new Set(ALL4);
const NM={spotify:'Spotify',apple:'Apple Music',amazon:'Amazon',tidal:'Tidal',deezer:'Deezer',ytm:'YouTube',dist:'Estándar',mp3:'MP3',portada:'Portada',canvas:'Canvas'};
const SN={spotify:'Spotify',apple:'Apple Music',amazon:'Amazon Music',tidal:'Tidal',deezer:'Deezer',ytm:'YouTube',dist:'Distribuidora',mp3:'Promo',portada:'Portada',canvas:'Canvas'};
/* marcas dibujadas a mano, evocativas; las definitivas requieren los activos oficiales de cada marca */
const BR={spotify:'#1DB954',apple:'#FA243C',amazon:'#232F3E',tidal:'#000000',deezer:'#A238FF',ytm:'#FF0000'};
const g=(b,x='')=>`<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.9" stroke-linecap="round" stroke-linejoin="round" ${x}>${b}</svg>`;
const GLY={
 spotify:g('<path d="M5 9c4.8-1.5 9.6-1.1 14 1.4"/><path d="M6.2 12.8c4-1.1 7.6-.8 10.8 1"/><path d="M7.4 16.2c3-.8 5.6-.5 8 .9"/>','stroke-width="2.1"'),
 apple:g('<path d="M10 17.2V6.6l8-1.9v10.6" /><circle cx="7.9" cy="17.4" r="2.2" fill="currentColor"/><circle cx="15.9" cy="15.5" r="2.2" fill="currentColor"/>'),
 amazon:g('<text x="12" y="13" font-size="12.5" font-weight="700" text-anchor="middle" fill="currentColor" stroke="none" font-family="Inter,sans-serif">a</text><path class="ac" d="M5 15.6c4.4 3 9.6 3 14 0" stroke="#FF9900" stroke-width="1.8"/><path class="ac" d="M16.6 15.1l2.6.4-.7 2.5" stroke="#FF9900" stroke-width="1.8"/>'),
 tidal:g('<path d="M12 4.5l3 3-3 3-3-3z M6 10.5l3 3-3 3-3-3z M18 10.5l3 3-3 3-3-3z M12 13.5l3 3-3 3-3-3z" fill="currentColor" stroke="none" transform="translate(0 .5) scale(1)"/>'),
 deezer:g('<path d="M4 18v-3M8 18v-6M12 18v-9M16 18v-6M20 18V6" stroke-width="3" stroke-linecap="butt"/>'),
 ytm:'<svg viewBox="0 0 24 24"><rect x="3" y="6" width="18" height="12" rx="3.6" fill="currentColor"/><path d="M10.2 9.3v5.4l4.6-2.7z" fill="var(--tc)"/></svg>',
 dist:g('<path d="M4 12l16-8-6 16-3-7z"/>'),
 mp3:g('<path d="M9 18V6l10-2v12"/><circle cx="6.5" cy="18" r="2.5"/><circle cx="16.5" cy="16" r="2.5"/>'),
 portada:g('<rect x="3.5" y="4.5" width="17" height="15" rx="2"/><circle cx="9" cy="10" r="1.6"/><path d="M4 17l5-4.5 4 3.5 3-2.5 4 3.5"/>'),
 canvas:g('<rect x="3.5" y="3.5" width="17" height="17" rx="2"/><path d="M10 8.5v7l5.5-3.5z"/>')
};
const ICO={
 pause:'<svg class="ic" viewBox="0 0 24 24" fill="currentColor"><rect x="6" y="5" width="4" height="14" rx="1"/><rect x="14" y="5" width="4" height="14" rx="1"/></svg>',
 reset:g('<path d="M4 12a8 8 0 1 0 2.6-5.9"/><path d="M4 4v5h5"/>','class="ic"'),
 img:g('<rect x="3.5" y="4.5" width="17" height="15" rx="2"/><circle cx="9" cy="10" r="1.6"/><path d="M4 17l5-4.5 4 3.5 3-2.5 4 3.5"/>','class="ic"'),
 folderPlus:g('<path d="M3 7a2 2 0 0 1 2-2h4l2 2h8a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/><path d="M12 10.5v5M9.5 13h5"/>','class="ic"')
};
const G4=[{n:'Plataformas',ids:['spotify','apple','amazon','tidal','deezer','ytm']},{n:'Distribuidora',ids:['dist']},{n:'Extras',ids:['mp3','portada','canvas']}];
const dest=id=>{const on=MO.on.has(id);return `<button class="chip ${on?'on':''}" role="switch" aria-checked="${on}" data-act="toggle" data-id="${id}" style="--br:${BR[id]||'var(--st)'}"><span class="mono">${GLY[id]}</span><span class="cn">${NM[id]}</span><i class="ck">${IC.check}</i></button>`};
const S={mode:'idle',loaded:true,artist:ART,title:TIT,fname:M.file,folder:0,cover:'sample',err:'',pp:0,pt:0,fresh:true,flash:0};
{const s=Q.get('state');if(s==='empty'){S.loaded=false;S.cover=null}else if(s==='creating'||s==='done')S.mode=s}
const FOLDERS=['Escritorio/Entregas','Documentos/Lanzamientos','Descargas'];
const waveH=(w,h,n,prog)=>waveSVG(w,h,n,{seed:7,prog}).replace('<svg class="wv"','<svg class="wv" preserveAspectRatio="none" style="width:100%;height:'+h+'px"');
const waveBox=(w,h,n,prog)=>`<div class="wvw ${S.fresh?'fresh':''}" data-wave="${w},${h},${n}">${waveH(w,h,n,prog!=null?prog:(S.pp>0?S.pp:null))}</div>`;
const timeTxt=()=>S.pp>0?(()=>{const s=Math.round(S.pp*232);return Math.floor(s/60)+':'+String(s%60).padStart(2,'0')+' / 3:52'})():'3:52';
const lpill=()=>`<span class="loudw"><button class="lpill" data-act="pop" data-p="detail"><i></i>−9,1 LUFS · pasa de 0 dB</button>
 <span class="pop" id="pop-detail" style="top:calc(100% + 8px);right:0;width:300px"><p>Las plataformas le bajarán el volumen. Tu archivo no se toca.</p>
 <span class="kv"><span>Volumen</span><b>−9,1 LUFS</b></span><span class="kv"><span>Pico más alto</span><b>+1,2 dBTP</b></span><span class="kv"><span>Pasa de 0 dB</span><b>3 veces</b></span><span class="kv"><span>Formato</span><b>WAV 24 bit · 96 kHz</b></span></span></span>`;
const presetHTML=()=>`<button class="preset" data-act="preset" data-preset title="Restablecer"><i>${IC.check}</i><span>Lo necesario</span></button>`;
const sumHTML=()=>`<span class="sumw"><button class="sum" data-act="pop" data-p="tree"><span data-sum></span>${IC.down}</button><div class="pop" id="pop-tree"><div class="tree" id="tree"></div></div></span>`;
const fieldsMore=()=>{const f=(l,k,v='',p='')=>`<div class="fld"><label>${l}</label><input data-m="${k}" value="${v}" placeholder="${p}"></div>`;
 return `<div class="pop" id="pop-more" style="top:calc(100% + 6px);left:0">${f('Con (feat.)','feat')}${f('Versión','ver','','Radio Edit')}${f('Año','year','2026')}${f('ISRC','isrc','','CC-XXX-YY-NNNNN')}${f('Compositor/a','comp')}${f('Sello','label')}</div>`};
const folderLine=()=>`<div class="fline">${IC.folder}<b data-folder>${FOLDERS[S.folder]}</b><button class="lnk" data-act="folder">cambiar</button></div>`;
const goBtn=()=>`<button class="go" data-act="create">${ICO.folderPlus}Crear carpeta de entrega</button>`;
const busyBtn=()=>`<div class="go busy"><i class="spin"></i>Creando carpeta…</div>`;
const inputs=()=>`<div class="two"><input class="in4" data-f="artist" placeholder="Artista" aria-label="Artista" autocomplete="off"><input class="in4" data-f="title" placeholder="Título" aria-label="Título" autocomplete="off"></div>`;
function fparts(x){if(x.doc){const i=x.doc.lastIndexOf('.');return[x.doc.slice(0,i),x.doc.slice(i)]}return[SN[x.d.id],x.tag.slice(x.tag.lastIndexOf('.'))]}
const frow=(x,done)=>{const[n,e]=fparts(x);return `<div class="fr ${done?'':'in'}">${IC.file}<span class="fn">${esc(n)}<em>${esc(e)}</em></span><span class="r ${done?'ok':''}">${done?IC.check:''}</span></div>`};
const advHTML=()=>`<div class="pop" id="pop-adv"><h4>Ajustes avanzados</h4>${ALL4.map(id=>`<div class="kv"><span>${SN[id]}</span><b>${MO.byId[id].spec.replace(/ · Digital Masters/,'')}</b></div>`).join('')}</div>`;
const coverHTML=cs=>S.cover?`<div class="cov has" data-act="cover" style="--cs:${cs}px" title="Cambiar portada">${S.cover==='sample'?cover(cs):`<img src="${S.cover}" alt="Portada">`}</div>`:`<div class="cov" data-act="cover" style="--cs:${cs}px" aria-label="Portada">${ICO.img}</div>`;
const heroEmpty=(style)=>`<div class="dz" ${style||''}><div class="dzone" data-act="song" style="height:100%"><span class="mk">${sym(40)}</span><b>Suelta tu canción</b>${S.err?`<small class="er">${S.err}</small>`:''}</div></div>`;
/* ---------- núcleo ---------- */
let V=null;const $=s=>document.querySelector(s);
function live(){const st=MO.stats(),n=st.n+st.docs,def=MO.on.size===ALL4.length;
 document.querySelectorAll('[data-sum]').forEach(e=>e.textContent=`${n} archivos · ${Math.round(st.mb)} MB`);
 document.querySelectorAll('[data-preset]').forEach(e=>{e.classList.toggle('cus',!def);e.querySelector('span').textContent=def?'Lo necesario':'Personalizado';e.querySelector('i').innerHTML=def?IC.check:ICO.reset});
 document.querySelectorAll('[data-fname]').forEach(e=>e.textContent=S.artist+' - '+S.title);
 document.querySelectorAll('[data-folder]').forEach(e=>e.textContent=FOLDERS[S.folder])}
function setDest(id,bump){document.querySelectorAll(`[data-id="${id}"]`).forEach(b=>{const on=MO.on.has(id);b.classList.toggle('on',on);b.setAttribute('aria-checked',on);
 if(bump&&on){b.classList.remove('bump');void b.offsetWidth;b.classList.add('bump')}})}
function setProg(p){document.querySelectorAll('[data-wave]').forEach(w=>{const r=w.querySelectorAll('rect');r.forEach((e,i)=>e.classList.toggle('p',(i+.5)/r.length<=p))})}
function celebrate(){if(STILL)return;document.querySelectorAll('[data-wave] rect').forEach((r,i,a)=>{r.style.animation=`rbump .75s ${Math.round(i*700/a.length)}ms cubic-bezier(.3,1.4,.5,1) both`});MO.confetti($('#win'),26)}
function render(){
 const app=$('#app');app.innerHTML=V.html();
 document.querySelectorAll('input[data-f]').forEach(i=>{i.value=S[i.dataset.f];if(S.flash&&i.value)i.classList.add('filled')});S.flash=0;
 MO.subs=[];const t=$('#tree');if(t){MO.exp={mode:'idle',pct:{}};MO.mountTree(t)}
 live();S.fresh=false;
 const p=Q.get('pop');if(p&&!D4.popDone){D4.popDone=1;const e=$('#pop-'+p);e&&e.classList.add('open')}
 if(S.mode==='creating')startCreate();
 if(S.mode==='done'){const F=MO.files();const l=$('[data-flist]');if(l)l.innerHTML=F.map(x=>frow(x,1)).join('');setProg(1);if(!D4.celeDone){D4.celeDone=1;celebrate()}}
}
function closePops(){document.querySelectorAll('.pop.open').forEach(p=>p.classList.remove('open'))}
function missing(){if(!S.loaded)return'song';if(!S.artist.trim())return'artist';if(!S.title.trim())return'title';if(!MO.on.size)return'dest';return null}
function nudge(k){let el;if(k==='song')el=$('.dz');else if(k==='dest')el=$('.dests');else{el=$(`input[data-f=${k}]`);el&&el.focus()}
 if(!el)return;el.classList.remove('nudge');void el.offsetWidth;el.classList.add('nudge');setTimeout(()=>el.classList.remove('nudge'),700)}
function parseName(n){let b=n.replace(/\.[^.]+$/,'').replace(/_+/g,' ');b=b.replace(/\s*[\(\[-]?\s*(master|mastered|final|mix|v\d+)[^\)\]]*[\)\]]?\s*$/i,'').trim();const m=b.match(/^(.+?)\s+-\s+(.+)$/);return m?{artist:m[1].trim(),title:m[2].trim()}:{title:b}}
function loadSong(f){const ext=(f?.name||'').split('.').pop().toLowerCase();
 if(f&&!['wav','aif','aiff','flac','caf','m4a'].includes(ext)){S.err=['mp3','aac','ogg'].includes(ext)?'Un MP3 no sirve como máster. Usa WAV o FLAC.':'Usa un archivo WAV, AIFF o FLAC.';render();setTimeout(()=>{S.err='';if(!S.loaded&&S.mode==='idle')render()},3200);return}
 S.err='';S.loaded=true;S.fresh=true;S.pp=0;S.flash=1;
 if(f){S.fname=f.name;const p=parseName(f.name);if(p.artist)S.artist=p.artist;S.title=p.title}else{S.artist=ART;S.title=TIT}
 render()}
function loadCover(f){if(!f||!f.type.startsWith('image/'))return;S.cover=URL.createObjectURL(f);render()}
function togglePlay(){if(S.pt){clearInterval(S.pt);S.pt=0}else S.pt=setInterval(()=>{S.pp+=.02;if(S.pp>=1){S.pp=0;clearInterval(S.pt);S.pt=0}paintPlay()},100);paintPlay()}
function paintPlay(){document.querySelectorAll('[data-wave]').forEach(e=>{const[w,h,n]=e.dataset.wave.split(',').map(Number);e.innerHTML=waveH(w,h,n,S.pp>0?S.pp:null)});
 document.querySelectorAll('[data-time]').forEach(e=>e.textContent=timeTxt());
 document.querySelectorAll('[data-act=play]').forEach(b=>b.innerHTML=S.pt?ICO.pause:IC.play)}
/* ---------- crear (simulado): la barra es la propia onda ---------- */
let RAF=0;
function startCreate(){
 const F=MO.files(),du=F.map(x=>x.doc?.18:.28+Math.min(x.mb,140)/140*.5),st=[];let c=0;du.forEach(d=>{st.push(c);c+=d});const total=c;
 const paint=t=>{const list=$('[data-flist]');if(!list)return;let shown=0;F.forEach((x,i)=>{if(t>=st[i])shown=i+1});
  while(list.children.length<shown){list.insertAdjacentHTML('beforeend',frow(F[list.children.length]));list.scrollTop=list.scrollHeight}
  F.forEach((x,i)=>{if(i>=shown)return;const p=Math.min(1,(t-st[i])/du[i]),r=list.children[i].querySelector('.r');
   if(p>=1){if(!r.classList.contains('ok')){r.classList.add('ok');r.innerHTML=IC.check}}else r.textContent=Math.round(p*100)+' %'});
  const pr=Math.min(1,t/total);setProg(pr);document.querySelectorAll('[data-pc]').forEach(e=>e.textContent=Math.round(pr*100)+' %')};
 if(STILL){paint(st[8]+du[8]*.45);return}
 const t0=performance.now();cancelAnimationFrame(RAF);
 const tick=()=>{const t=(performance.now()-t0)/1000;paint(t);if(t>=total+.3){S.mode='done';D4.celeDone=0;render();return}RAF=requestAnimationFrame(tick)};
 RAF=requestAnimationFrame(tick)}
/* ---------- eventos ---------- */
document.addEventListener('click',e=>{
 const a=e.target.closest('[data-act]');
 if(!e.target.closest('.pop')&&!(a&&a.dataset.act==='pop'))closePops();
 if(!a)return;const act=a.dataset.act;
 if(act==='pop'){const el=$('#pop-'+a.dataset.p),was=el.classList.contains('open');closePops();if(!was)el.classList.add('open');return}
 if(act==='toggle'){if(S.mode!=='idle')return;MO.toggle(a.dataset.id);setDest(a.dataset.id,1);live()}
 else if(act==='preset'){ALL4.forEach(id=>MO.on.add(id));MO.refresh();ALL4.forEach(id=>setDest(id,1));live()}
 else if(act==='song'){$('#fsong').click()}
 else if(act==='cover'){$('#fcov').click()}
 else if(act==='play'){togglePlay()}
 else if(act==='more'){const el=$('#pop-more');const was=el.classList.contains('open');closePops();if(!was)el.classList.add('open')}
 else if(act==='folder'){S.folder=(S.folder+1)%FOLDERS.length;live()}
 else if(act==='create'){const m=missing();if(m){nudge(m);return}S.mode='creating';render()}
 else if(act==='again'){S.mode='idle';S.loaded=false;S.cover=null;S.title='';S.pp=0;ALL4.forEach(id=>MO.on.add(id));render()}
 else if(act==='open'){const s=a.querySelector('span');s.textContent='Abriendo…';setTimeout(()=>s.textContent='Abrir carpeta',1200)}
});
document.addEventListener('input',e=>{const i=e.target.closest('input[data-f]');if(i){S[i.dataset.f]=i.value;live()}});
['dragenter','dragover'].forEach(n=>document.addEventListener(n,e=>{e.preventDefault();const z=e.target.closest('.dz,.cov');if(z)z.classList.add('over')}));
['dragleave','drop'].forEach(n=>document.addEventListener(n,e=>{e.preventDefault();document.querySelectorAll('.over').forEach(z=>z.classList.remove('over'))}));
document.addEventListener('drop',e=>{e.preventDefault();if(S.mode!=='idle')return;const f=e.dataTransfer.files[0];if(!f)return;
 if(e.target.closest('.cov')){loadCover(f)}else if(e.target.closest('.dz')){if(f.type.startsWith('image/'))loadCover(f);else loadSong(f)}});
document.addEventListener('change',e=>{if(e.target.id==='fsong'){loadSong(e.target.files[0]||null);e.target.value=''}if(e.target.id==='fcov'){loadCover(e.target.files[0]);e.target.value=''}});
document.addEventListener('keydown',e=>{if(e.key==='Escape')closePops()});
const D4={boot(v){V=v;$('#tbw').innerHTML=titlebar(`<button class="adv" data-act="pop" data-p="adv">Ajustes avanzados</button>`);themeInit();$('#win').insertAdjacentHTML('beforeend',advHTML());render()}};

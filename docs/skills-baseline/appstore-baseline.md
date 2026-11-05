# Baseline Mac App Store (solo conocimiento propio, sin verificar)

Todo lo marcado NO SEGURO debe verificarse antes de actuar.

## 1. Sandbox y bookmarks
Entitlements mínimos:
- `com.apple.security.app-sandbox` = true
- `com.apple.security.files.user-selected.read-write` = true (cubre el archivo arrastrado y la carpeta elegida)
- `com.apple.security.files.bookmarks.app-scope` = true (para persistir el acceso)
- Sin `network.client`.

Bookmark: tras `NSOpenPanel` (`canChooseDirectories`, `canCreateDirectories`):
1. `url.bookmarkData(options: .withSecurityScope, includingResourceValuesForKeys: nil, relativeTo: nil)`; guardar el `Data`.
2. Al arrancar: `URL(resolvingBookmarkData:options:.withSecurityScope, relativeTo:nil, bookmarkDataIsStale:&stale)`; si `stale`, regenerar y guardar.
3. `startAccessingSecurityScopedResource()` antes de escribir y `stop...` después (equilibrados; el acceso a la carpeta cubre los hijos).

Errores típicos: olvidar el entitlement bookmarks; guardar la ruta en vez del `Data`; no llamar a start (error Cocoa 513/257, "no permission"); ignorar `stale`; llamar a stop antes de terminar la escritura async.

## 2. afconvert / afclip con Process
Una app sandboxed puede lanzar `Process` sobre `/usr/bin/afconvert`; el hijo hereda el sandbox y los permisos de las URLs abiertas por el usuario. Pero: NO SEGURO que funcione sin errores en todas las versiones, NO SEGURO que `afclip` exista/sea estable en todas las macOS, y la revisión puede cuestionarlo (dependencia de herramientas del sistema no garantizadas, guía 2.4.5/2.5.2). Alternativa recomendada, en proceso: `AVAudioFile`/`AVAudioConverter`, `ExtAudioFile` (AudioToolbox, es lo que usa afconvert) o `AVAssetExportSession`. Si hace falta un binario propio: en el bundle, firmado, con `com.apple.security.inherit`.

## 3. LAME (LGPL)
Técnicamente se puede enlazar. La LGPL exige: enlace dinámico (dylib/framework en `Contents/Frameworks`), que el usuario pueda sustituir la biblioteca (relink), incluir texto de licencia y aviso de uso (pantalla Acerca de), y ofrecer el código fuente de LAME y de tus modificaciones. Problema: la firma de Apple y el DRM de la tienda impiden de facto sustituir la biblioteca; riesgo jurídico (NO SEGURO). Apple no audita licencias en revisión. Mitigación: dinámico, sin modificar, avisos y URL del código. Alternativa: no ofrecer MP3, solo AAC/ALAC/WAV/FLAC con APIs de Apple (NO SEGURO si macOS codifica MP3).

## 4. Firma y subida
- Apple Developer Program (99 USD/año), Account Holder.
- Certificados: Apple Distribution (antes "Mac App Distribution") y Mac Installer Distribution (para el .pkg). Xcode con firma automática los crea.
- App ID con bundle ID, perfil de provisioning tipo Mac App Store (se incrusta como `embedded.provisionprofile`).
- Notarización: NO aplica a la Mac App Store (la revisión sustituye). Solo Developer ID fuera de tienda.
- Subida: Xcode, Product > Archive > Organizer > Distribute App > App Store Connect > Upload. Alternativas: app Transporter o `xcrun altool --upload-app -f App.pkg -t macos` (NO SEGURO de la sintaxis actual).
- TestFlight para Mac existe: la build subida aparece en TestFlight, testers internos (hasta 100) sin revisión, externos con revisión beta; instalan con la app TestFlight de macOS.

## 5. App Store Connect
- Precio: punto de precio 9,99 € como precio base en España; Apple calcula el resto (verificar que aparece).
- Categoría: Utilidades (principal), Música (secundaria).
- Capturas Mac: 16:10, 1280x800, 1440x900, 2560x1600 o 2880x1800; de 1 a 10. Icono 1024x1024.
- Ficha: nombre (30), subtítulo (30), texto promocional (170), descripción (4000), palabras clave (100), URL de soporte (obligatoria), URL de privacidad (obligatoria aunque no haya red; basta una página que diga "no recopilamos datos").
- Privacidad: etiqueta "Datos no recopilados".
- Export compliance: `ITSAppUsesNonExemptEncryption` = NO en Info.plist (sin cifrado propio).
- Estatus de "trader" DSA de la UE: obligatorio: nombre, dirección y teléfono públicos.
- Firmar Paid Apps Agreement, banca y fiscalidad antes de vender.
- Notas de revisión: flujo y audio de prueba.

## 6. Ocho rechazos frecuentes
1. Cuelgues o fallos (2.1): probar en macOS mínima declarada, con y sin carpeta elegida.
2. Funcionalidad mínima (4.2): valor claro, varios formatos.
3. Fuera del sandbox / entitlements sobrantes (2.4.5): solo los tres anteriores.
4. Metadatos o capturas que no coinciden con la app (2.3).
5. Falta URL de privacidad o soporte (5.1.1, 1.5).
6. Reviewer sin archivo de prueba o sin explicación: dar notas y ejemplo.
7. Descargar o instalar código/herramientas externas (2.4.5): todo en el bundle.
8. Mencionar otras plataformas, "beta" o precios en la ficha (2.3).

## 7. Vendedor "nortia."
Cuenta individual: el vendedor es tu nombre legal; no puede mostrarse un nombre comercial (NO SEGURO si autónomo con nombre comercial lo admite). Opciones: (a) constituir sociedad (SL), obtener D-U-N-S (gratuito, 1-2 semanas) e inscribirse como Organización; (b) publicar ya como individuo y luego transferir la app a la cuenta de organización; (c) mientras tanto, "nortia." en copyright, web y soporte.

## 8. Small Business Program (15 %)
App Store Connect > Acuerdos, fiscalidad y banca > Small Business Program: solicitud por el Account Holder, aceptando condiciones. Requisito: ingresos de la cuenta inferiores a 1 M USD en el año natural anterior. Se aplica desde el siguiente mes fiscal tras la aprobación y se pierde al superar 1 M USD.

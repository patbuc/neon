# Neon logo

The mark is "Ne" in Inter ExtraBold (800), outlined to paths, inside a pink
(`#ec597c`) rounded frame. Use dark ink (`#1f1a1c`) on light backgrounds and
light ink (`#f3eef0`) on dark ones. App icons put the mark on a `#141016`
tile. Sizes of 32 px and below use a heavier frame and larger letters.

## Files

| Folder | Contents |
|---|---|
| `svg/` | Masters. `neon-logo-light.svg` (for light backgrounds), `neon-logo-dark.svg`, `neon-logo-auto.svg` (follows `prefers-color-scheme`). App icon tiles: `neon-icon.svg`, `neon-icon-small.svg` (16–32 px), `neon-icon-full-bleed.svg` (for platforms that apply their own mask). |
| `logo/` | Logo PNGs at 256, 512 and 1024 px, light and dark. |
| `macos/` | `neon.icns`, 16–1024 px on Apple's 1024 grid. |
| `windows/` | `neon.ico` with 16, 20, 24, 32, 40, 48, 64 and 256 px, and `neon.rc`, which `build.rs` compiles into the Windows `neon.exe` as its icon. |
| `linux/` | freedesktop `hicolor/` theme: 16–512 px PNGs and `scalable/apps/neon.svg`. Install under `/usr/share/icons/`. |
| `ios/` | `AppIcon-1024.png` and `Contents.json` for an `AppIcon.appiconset`. |
| `android/` | Adaptive icon (`mipmap-anydpi-v26/ic_launcher.xml`, foreground per density, background colour in `values/`), legacy `ic_launcher.png` per density, `play-store-512.png`. |
| `web/` | `favicon.ico`, `favicon.svg`, `apple-touch-icon.png`, PWA icons (192, 512, maskable 512) and `site.webmanifest`. |

For a web page:

```html
<link rel="icon" href="/favicon.ico" sizes="48x48">
<link rel="icon" href="/favicon.svg" type="image/svg+xml">
<link rel="apple-touch-icon" href="/apple-touch-icon.png">
<link rel="manifest" href="/site.webmanifest">
```

## Rebuilding

`build.py` regenerates every folder above from `Inter.woff2` (a latin subset
of Inter, licensed under `Inter-OFL.txt`). It needs macOS for `iconutil`.

```bash
python3 -m venv /tmp/neon-logo
/tmp/neon-logo/bin/pip install -r assets/logo/requirements.txt
/tmp/neon-logo/bin/python assets/logo/build.py
```

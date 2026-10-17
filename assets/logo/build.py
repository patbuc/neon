"""Build the Neon "Ne" logo and app icons for every platform into this directory.

Usage: python3 assets/logo/build.py   (needs macOS for iconutil; see README.md)
"""

import json
import shutil
import subprocess
import tempfile
from io import BytesIO
from pathlib import Path

import resvg_py
from fontTools.pens.boundsPen import BoundsPen
from fontTools.pens.svgPathPen import SVGPathPen
from fontTools.pens.transformPen import TransformPen
from fontTools.ttLib import TTFont
from fontTools.varLib import instancer
from PIL import Image

PINK = "#ec597c"
INK_ON_LIGHT = "#1f1a1c"
INK_ON_DARK = "#f3eef0"
TILE = "#141016"

OUT = Path(__file__).resolve().parent
GENERATED = ["svg", "logo", "macos", "windows", "linux", "ios", "android", "web"]

font = instancer.instantiateVariableFont(TTFont(OUT / "Inter.woff2"), {"wght": 800, "opsz": 32})
UPM = font["head"].unitsPerEm
GLYPHS = font.getGlyphSet()
CMAP = font.getBestCmap()
CAP = font["OS/2"].sCapHeight
TRACKING = -1.5 / 54  # em


def ne_path(size, cx=60.0, cy=60.0):
    """Outlined "Ne" in 120-unit space, centred on its ink box horizontally and its cap height vertically."""
    s = size / UPM
    names = [CMAP[ord("N")], CMAP[ord("e")]]
    x, placed = 0.0, []
    for name in names:
        placed.append((name, x))
        x += GLYPHS[name].width + TRACKING * UPM
    bounds = BoundsPen(GLYPHS)
    for name, gx in placed:
        GLYPHS[name].draw(TransformPen(bounds, (1, 0, 0, 1, gx, 0)))
    xmin, _, xmax, _ = bounds.bounds
    tx = cx - (xmin + xmax) / 2 * s
    ty = cy + CAP / 2 * s
    pen = SVGPathPen(GLYPHS, ntos=lambda v: f"{v:.2f}".rstrip("0").rstrip("."))
    for name, gx in placed:
        GLYPHS[name].draw(TransformPen(pen, (s, 0, 0, -s, tx + gx * s, ty)))
    return pen.getCommands()


# Geometry in a 120-unit square. "small" is tuned for 16-32 px: heavier frame, bigger letters.
GEOMETRY = {
    "mark": {"frame": (8, 104, 16, 5), "ne": 54},
    "tile": {"frame": (16, 88, 14, 5), "ne": 46, "radius": 27},
    "small": {"frame": (10, 100, 14, 9), "ne": 58, "radius": 22},
}


def frame(g, color=PINK):
    xy, wh, r, sw = g["frame"]
    return f'<rect x="{xy}" y="{xy}" width="{wh}" height="{wh}" rx="{r}" fill="none" stroke="{color}" stroke-width="{sw}"/>'


def ne(g, ink):
    return f'<path d="{ne_path(g["ne"])}" fill="{ink}"/>'


def svg(body, view=120):
    return f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {view} {view}">{body}</svg>\n'


def mark_svg(ink):
    g = GEOMETRY["mark"]
    return svg(frame(g) + ne(g, ink))


def auto_mark_svg():
    g = GEOMETRY["mark"]
    style = f"<style>.ink{{fill:{INK_ON_LIGHT}}}@media (prefers-color-scheme: dark){{.ink{{fill:{INK_ON_DARK}}}}}</style>"
    return svg(style + frame(g) + f'<path class="ink" d="{ne_path(g["ne"])}"/>')


def tile_body(kind):
    g = GEOMETRY[kind]
    return f'<rect width="120" height="120" rx="{g["radius"]}" fill="{TILE}"/>' + frame(g) + ne(g, INK_ON_DARK)


def tile_svg(kind="tile", margin=0.0):
    """Rounded dark tile, inset by margin (fraction of the canvas) for platforms that show the shape as is."""
    scale = 1 - 2 * margin
    return svg(f'<g transform="translate({120 * margin:.3f} {120 * margin:.3f}) scale({scale:.5f})">{tile_body(kind)}</g>')


def full_bleed_svg(content_scale=1.0, background=True):
    """Square canvas for platforms that apply their own mask; content shrinks to fit the mask's safe zone."""
    g = GEOMETRY["tile"]
    off = 60 * (1 - content_scale)
    inner = frame(g) + ne(g, INK_ON_DARK)
    bg = f'<rect width="120" height="120" fill="{TILE}"/>' if background else ""
    return svg(bg + f'<g transform="translate({off:.3f} {off:.3f}) scale({content_scale:.5f})">{inner}</g>')


def render(svg_text, px):
    return bytes(resvg_py.svg_to_bytes(svg_string=svg_text, width=px, height=px))


def write(path, data):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(data if isinstance(data, bytes) else data.encode())


def icon_png(px, margin):
    return render(tile_svg("small" if px <= 32 else "tile", margin), px)


for name in GENERATED:
    shutil.rmtree(OUT / name, ignore_errors=True)

# Masters
masters = {
    "neon-logo-light.svg": mark_svg(INK_ON_LIGHT),
    "neon-logo-dark.svg": mark_svg(INK_ON_DARK),
    "neon-logo-auto.svg": auto_mark_svg(),
    "neon-icon.svg": tile_svg("tile", 0.0625),
    "neon-icon-small.svg": tile_svg("small", 0.0),
    "neon-icon-full-bleed.svg": full_bleed_svg(),
}
for name, text in masters.items():
    write(OUT / "svg" / name, text)

# Logo PNGs for READMEs and docs
for variant in ("light", "dark"):
    for px in (256, 512, 1024):
        write(OUT / "logo" / f"neon-logo-{variant}-{px}.png", render(masters[f"neon-logo-{variant}.svg"], px))

# macOS: 1024 grid with an 824 body, packed into .icns
iconset = Path(tempfile.mkdtemp()) / "neon.iconset"
for pt in (16, 32, 128, 256, 512):
    for scale in (1, 2):
        px = pt * scale
        suffix = "" if scale == 1 else "@2x"
        # The 1024 grid's margin leaves too few pixels for the mark at 16 and 32 px
        small = px <= 32
        write(iconset / f"icon_{pt}x{pt}{suffix}.png", render(tile_svg("small" if small else "tile", 0.0625 if small else 100 / 1024), px))
(OUT / "macos").mkdir()
subprocess.run(["iconutil", "-c", "icns", str(iconset), "-o", str(OUT / "macos" / "neon.icns")], check=True)
shutil.rmtree(iconset.parent)

# Windows: one .ico with the sizes Explorer and the taskbar ask for
win_sizes = [16, 20, 24, 32, 40, 48, 64, 256]
win_images = [Image.open(BytesIO(icon_png(px, 0.0 if px <= 32 else 0.03))) for px in win_sizes]
(OUT / "windows").mkdir(parents=True)
win_images[-1].save(OUT / "windows" / "neon.ico", format="ICO",
                    sizes=[(px, px) for px in win_sizes], append_images=win_images[:-1])

# Linux: freedesktop hicolor theme
for px in (16, 22, 24, 32, 48, 64, 128, 256, 512):
    write(OUT / "linux" / "hicolor" / f"{px}x{px}" / "apps" / "neon.png", icon_png(px, 0.0 if px <= 32 else 0.0625))
write(OUT / "linux" / "hicolor" / "scalable" / "apps" / "neon.svg", masters["neon-icon.svg"])

# iOS: single 1024 square, opaque; the system rounds the corners
ios = Image.open(BytesIO(render(masters["neon-icon-full-bleed.svg"], 1024))).convert("RGB")
(OUT / "ios").mkdir(parents=True)
ios.save(OUT / "ios" / "AppIcon-1024.png")
write(OUT / "ios" / "Contents.json", json.dumps({
    "images": [{"filename": "AppIcon-1024.png", "idiom": "universal", "platform": "ios", "size": "1024x1024"}],
    "info": {"author": "xcode", "version": 1},
}, indent=2) + "\n")

# Android: adaptive icon (foreground in the 66/108 safe zone, solid background) plus legacy launcher PNGs
android = OUT / "android"
densities = {"mdpi": 1, "hdpi": 1.5, "xhdpi": 2, "xxhdpi": 3, "xxxhdpi": 4}
foreground = full_bleed_svg(content_scale=0.58, background=False)
for name, d in densities.items():
    write(android / f"mipmap-{name}" / "ic_launcher_foreground.png", render(foreground, round(108 * d)))
    write(android / f"mipmap-{name}" / "ic_launcher.png", icon_png(round(48 * d), 0.04))
write(android / "mipmap-anydpi-v26" / "ic_launcher.xml",
      '<?xml version="1.0" encoding="utf-8"?>\n'
      '<adaptive-icon xmlns:android="http://schemas.android.com/apk/res/android">\n'
      '    <background android:drawable="@color/ic_launcher_background"/>\n'
      '    <foreground android:drawable="@mipmap/ic_launcher_foreground"/>\n'
      '    <monochrome android:drawable="@mipmap/ic_launcher_foreground"/>\n'
      '</adaptive-icon>\n')
write(android / "values" / "ic_launcher_background.xml",
      '<?xml version="1.0" encoding="utf-8"?>\n'
      f'<resources>\n    <color name="ic_launcher_background">{TILE}</color>\n</resources>\n')
write(android / "play-store-512.png", render(masters["neon-icon-full-bleed.svg"], 512))

# Web: favicons, touch icon and PWA icons
web = OUT / "web"
fav_images = [Image.open(BytesIO(icon_png(px, 0.0))) for px in (16, 32, 48)]
web.mkdir(parents=True)
fav_images[-1].save(web / "favicon.ico", format="ICO", sizes=[(16, 16), (32, 32), (48, 48)], append_images=fav_images[:-1])
write(web / "favicon.svg", masters["neon-icon-small.svg"])
write(web / "apple-touch-icon.png", render(masters["neon-icon-full-bleed.svg"], 180))
write(web / "icon-192.png", icon_png(192, 0.0))
write(web / "icon-512.png", icon_png(512, 0.0))
write(web / "icon-maskable-512.png", render(full_bleed_svg(content_scale=0.72), 512))
write(web / "site.webmanifest", json.dumps({
    "name": "Neon", "short_name": "Neon",
    "icons": [
        {"src": "/icon-192.png", "sizes": "192x192", "type": "image/png"},
        {"src": "/icon-512.png", "sizes": "512x512", "type": "image/png"},
        {"src": "/icon-maskable-512.png", "sizes": "512x512", "type": "image/png", "purpose": "maskable"},
    ],
    "theme_color": TILE, "background_color": TILE, "display": "standalone",
}, indent=2) + "\n")

print("\n".join(sorted(str(p.relative_to(OUT)) for name in GENERATED for p in (OUT / name).rglob("*") if p.is_file())))

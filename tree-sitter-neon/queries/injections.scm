; Highlight interpolated expressions as Neon code
((string_interpolation
  (expression) @injection.content)
  (#set! injection.language "neon"))

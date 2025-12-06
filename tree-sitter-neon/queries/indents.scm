; Indent blocks
[
  (block)
  (function_declaration)
  (if_statement)
  (while_statement)
  (for_statement)
] @indent

; Dedent on closing braces
[
  "}"
  "]"
  ")"
] @outdent

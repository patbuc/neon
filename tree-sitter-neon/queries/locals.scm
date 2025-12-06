; Definitions
(val_declaration
  name: (identifier) @local.definition)

(var_declaration
  name: (identifier) @local.definition)

(function_declaration
  name: (identifier) @local.definition)

(parameter_list
  (identifier) @local.definition)

(struct_declaration
  name: (identifier) @local.definition)

; References
(identifier) @local.reference

; Scopes
[
  (function_declaration)
  (block)
  (for_statement)
] @local.scope

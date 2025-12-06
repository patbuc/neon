; Keywords
[
  "val"
  "var"
  "fn"
  "struct"
  "if"
  "else"
  "while"
  "for"
  "in"
  "return"
  "print"
] @keyword

; Statements (these are named nodes, not simple keywords)
(break_statement) @keyword
(continue_statement) @keyword

; Operators
[
  "+"
  "-"
  "*"
  "/"
  "//"
  "%"
  "=="
  "!="
  "<"
  "<="
  ">"
  ">="
  "&&"
  "||"
  "!"
  "++"
  "--"
  ".."
  "..="
  "="
] @operator

; Punctuation
[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket

[
  ","
  ";"
  ":"
] @punctuation.delimiter

"." @punctuation.special

; Literals
(number) @number
(string) @string
(string_content) @string
(escape_sequence) @string.escape
(boolean) @boolean
(nil) @constant.builtin

; String interpolation
(string_interpolation_start) @punctuation.special
(string_interpolation_end) @punctuation.special
(string_interpolation
  (expression) @embedded)

; Comments
(comment) @comment

; Functions
(function_declaration
  name: (identifier) @function)

(call_expression
  function: (expression
    (primary_expression
      (identifier) @function.call)))

(method_call_expression
  method: (identifier) @function.method)

; Types (struct names)
(struct_declaration
  name: (identifier) @type)

; Variables and identifiers
(val_declaration
  name: (identifier) @variable)

(var_declaration
  name: (identifier) @variable)

(parameter_list
  (identifier) @parameter)

(field_expression
  field: (identifier) @property)

(identifier) @variable

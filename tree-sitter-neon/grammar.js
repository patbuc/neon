module.exports = grammar({
  name: 'neon',

  externals: $ => [
    $.string_content,
    $.string_interpolation_start,
    $.string_interpolation_end,
  ],

  extras: $ => [
    /\s/,
    $.comment,
  ],

  word: $ => $.identifier,

  conflicts: $ => [
    [$.map_literal, $.set_literal],
    [$.field_expression, $.method_call_expression],
    [$.for_statement],
    [$.block, $.map_literal],
    [$.block, $.set_literal],
    [$.expression_statement, $.set_literal],
  ],

  rules: {
    source_file: $ => repeat($.declaration),

    comment: $ => token(prec(-1, seq('//', /[^\r\n]*/))),

    declaration: $ => choice(
      $.val_declaration,
      $.var_declaration,
      $.function_declaration,
      $.struct_declaration,
      $.statement,
    ),

    val_declaration: $ => seq(
      'val',
      field('name', $.identifier),
      optional(seq('=', field('value', $.expression))),
    ),

    var_declaration: $ => seq(
      'var',
      field('name', $.identifier),
      optional(seq('=', field('value', $.expression))),
    ),

    function_declaration: $ => seq(
      'fn',
      field('name', $.identifier),
      field('parameters', $.parameter_list),
      field('body', $.block),
    ),

    parameter_list: $ => seq(
      '(',
      optional(seq(
        $.identifier,
        repeat(seq(',', $.identifier)),
        optional(','),
      )),
      ')',
    ),

    struct_declaration: $ => seq(
      'struct',
      field('name', $.identifier),
      '{',
      optional(seq(
        $.identifier,
        repeat(seq(optional(','), $.identifier)),
        optional(','),
      )),
      '}',
    ),

    statement: $ => choice(
      $.expression_statement,
      $.print_statement,
      $.block,
      $.if_statement,
      $.while_statement,
      $.for_statement,
      $.return_statement,
      $.break_statement,
      $.continue_statement,
    ),

    expression_statement: $ => $.expression,

    print_statement: $ => seq('print', $.expression),

    block: $ => seq(
      '{',
      repeat($.declaration),
      '}',
    ),

    if_statement: $ => prec.right(seq(
      'if',
      '(',
      field('condition', $.expression),
      ')',
      field('consequence', $.statement),
      optional(seq('else', field('alternative', $.statement))),
    )),

    while_statement: $ => seq(
      'while',
      '(',
      field('condition', $.expression),
      ')',
      field('body', $.statement),
    ),

    for_statement: $ => choice(
      // For-in: for (item in collection)
      seq(
        'for',
        '(',
        field('variable', $.identifier),
        'in',
        field('collection', $.expression),
        ')',
        field('body', $.statement),
      ),
      // C-style: for (init; cond; incr)
      seq(
        'for',
        '(',
        field('initializer', choice($.val_declaration, $.var_declaration)),
        ';',
        field('condition', $.expression),
        ';',
        field('increment', $.expression),
        ')',
        field('body', $.statement),
      ),
    ),

    return_statement: $ => seq('return', $.expression),

    break_statement: $ => 'break',

    continue_statement: $ => 'continue',

    // Expression hierarchy following Neon precedence (lowest to highest):
    // Assignment < Or < And < Equality < Comparison < Range < Term < Factor < Unary < Call < Primary
    expression: $ => choice(
      $.assignment_expression,
      $.binary_expression,
      $.unary_expression,
      $.postfix_expression,
      $.call_expression,
      $.method_call_expression,
      $.field_expression,
      $.index_expression,
      $.range_expression,
      $.primary_expression,
    ),

    // Assignment: x = 5, obj.field = 5, arr[0] = 5
    assignment_expression: $ => prec.right(0, seq(
      field('left', choice(
        $.identifier,
        $.field_expression,
        $.index_expression,
      )),
      '=',
      field('right', $.expression),
    )),

    // Binary expressions with precedence matching parser.rs
    binary_expression: $ => {
      const table = [
        [prec.left, 1, '||'],          // Or
        [prec.left, 2, '&&'],          // And
        [prec.left, 3, choice('==', '!=')],  // Equality
        [prec.left, 4, choice('<', '<=', '>', '>=')],  // Comparison
        [prec.left, 6, choice('+', '-')],    // Term
        [prec.left, 7, choice('*', '/', '//', '%')],  // Factor
      ];

      return choice(...table.map(([fn, precedence, operator]) =>
        fn(precedence, seq(
          field('left', $.expression),
          field('operator', operator),
          field('right', $.expression),
        ))
      ));
    },

    // Unary: -x, !flag
    unary_expression: $ => prec(8, seq(
      field('operator', choice('-', '!')),
      field('operand', $.expression),
    )),

    // Postfix: x++, x--
    postfix_expression: $ => prec(9, seq(
      field('operand', $.expression),
      field('operator', choice('++', '--')),
    )),

    // Call: foo(), add(1, 2, 3)
    call_expression: $ => prec(10, seq(
      field('function', $.expression),
      field('arguments', $.argument_list),
    )),

    // Method call: arr.push(5), str.len()
    method_call_expression: $ => prec(10, seq(
      field('object', $.expression),
      '.',
      field('method', $.identifier),
      field('arguments', $.argument_list),
    )),

    // Field access: point.x, obj.field.nested
    field_expression: $ => prec(10, seq(
      field('object', $.expression),
      '.',
      field('field', $.identifier),
    )),

    // Index access: arr[0], map["key"]
    index_expression: $ => prec(10, seq(
      field('object', $.expression),
      '[',
      field('index', $.expression),
      ']',
    )),

    // Range: 1..10, 0..=100
    range_expression: $ => prec.left(5, seq(
      field('start', $.expression),
      field('operator', choice('..', '..=')),
      field('end', $.expression),
    )),

    // Argument list for function/method calls
    argument_list: $ => seq(
      '(',
      optional(seq(
        $.expression,
        repeat(seq(',', $.expression)),
        optional(','),
      )),
      ')',
    ),

    // Primary expressions (highest precedence)
    primary_expression: $ => choice(
      $.identifier,
      $.number,
      $.string,
      $.boolean,
      $.nil,
      $.array_literal,
      $.map_literal,
      $.set_literal,
      $.grouped_expression,
    ),

    // Array literal: [1, 2, 3], [[1, 2], [3, 4]]
    array_literal: $ => seq(
      '[',
      optional(seq(
        $.expression,
        repeat(seq(',', $.expression)),
        optional(','),
      )),
      ']',
    ),

    // Map literal: {"key": "value", "num": 42}
    map_literal: $ => seq(
      '{',
      optional(seq(
        $.map_entry,
        repeat(seq(',', $.map_entry)),
        optional(','),
      )),
      '}',
    ),

    // Map entry: key: value
    map_entry: $ => seq(
      field('key', $.expression),
      ':',
      field('value', $.expression),
    ),

    // Set literal: {1, 2, 3, 4}
    set_literal: $ => seq(
      '{',
      $.expression,
      repeat(seq(',', $.expression)),
      optional(','),
      '}',
    ),

    // Grouped expression: (expr)
    grouped_expression: $ => seq(
      '(',
      $.expression,
      ')',
    ),

    string: $ => seq(
      '"',
      repeat(choice(
        $.string_content,
        $.escape_sequence,
        $.string_interpolation,
      )),
      '"',
    ),

    string_interpolation: $ => seq(
      $.string_interpolation_start,
      $.expression,
      $.string_interpolation_end,
    ),

    escape_sequence: $ => token.immediate(seq(
      '\\',
      choice(
        /[\\'"nrt]/,        // Basic escapes
        /x[0-9a-fA-F]{2}/,  // Hex escape \xHH
        /u[0-9a-fA-F]{4}/,  // Unicode escape \uHHHH
        /U[0-9a-fA-F]{8}/,  // Unicode escape \UHHHHHHHH
      ),
    )),

    number: $ => token(choice(
      /\d+\.\d+/,
      /\d+/,
    )),

    boolean: $ => choice('true', 'false'),

    nil: $ => 'nil',

    identifier: $ => /[a-zA-Z_][a-zA-Z0-9_]*/,
  },
});

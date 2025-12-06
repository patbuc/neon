module.exports = grammar({
  name: 'neon',

  extras: $ => [
    /\s/,
    $.comment,
  ],

  word: $ => $.identifier,

  rules: {
    source_file: $ => repeat($.declaration),

    comment: $ => token(seq('//', /.*/)),

    declaration: $ => choice(
      $.val_declaration,
      $.var_declaration,
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

    statement: $ => choice(
      $.expression_statement,
      $.print_statement,
    ),

    expression_statement: $ => $.expression,

    print_statement: $ => seq('print', $.expression),

    expression: $ => choice(
      $.binary_expression,
      $.unary_expression,
      $.primary_expression,
    ),

    binary_expression: $ => choice(
      prec.left(1, seq(field('left', $.expression), '+', field('right', $.expression))),
      prec.left(1, seq(field('left', $.expression), '-', field('right', $.expression))),
      prec.left(2, seq(field('left', $.expression), '*', field('right', $.expression))),
      prec.left(2, seq(field('left', $.expression), '/', field('right', $.expression))),
    ),

    unary_expression: $ => prec(3, seq(
      choice('-', '!'),
      $.expression,
    )),

    primary_expression: $ => choice(
      $.identifier,
      $.number,
      $.string,
      $.boolean,
      $.nil,
      seq('(', $.expression, ')'),
    ),

    string: $ => seq(
      '"',
      repeat(choice(
        token.immediate(prec(1, /[^"\\$]+/)),
        /\\./,
      )),
      '"',
    ),

    number: $ => token(choice(
      /\d+\.\d+/,
      /\d+/,
    )),

    boolean: $ => choice('true', 'false'),

    nil: $ => 'nil',

    identifier: $ => /[a-zA-Z_][a-zA-Z0-9_]*/,
  },
});

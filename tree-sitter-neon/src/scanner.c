#include <tree_sitter/parser.h>
#include <wctype.h>
#include <string.h>

enum TokenType {
  STRING_CONTENT,
  STRING_INTERPOLATION_START,
  STRING_INTERPOLATION_END,
};

typedef struct {
  int brace_depth;
} Scanner;

void *tree_sitter_neon_external_scanner_create() {
  Scanner *scanner = (Scanner *)calloc(1, sizeof(Scanner));
  scanner->brace_depth = 0;
  return scanner;
}

void tree_sitter_neon_external_scanner_destroy(void *payload) {
  Scanner *scanner = (Scanner *)payload;
  free(scanner);
}

unsigned tree_sitter_neon_external_scanner_serialize(void *payload, char *buffer) {
  Scanner *scanner = (Scanner *)payload;
  if (sizeof(int) > 0) {
    memcpy(buffer, &scanner->brace_depth, sizeof(int));
  }
  return sizeof(int);
}

void tree_sitter_neon_external_scanner_deserialize(void *payload, const char *buffer, unsigned length) {
  Scanner *scanner = (Scanner *)payload;
  if (length > 0) {
    memcpy(&scanner->brace_depth, buffer, sizeof(int));
  }
}

bool tree_sitter_neon_external_scanner_scan(
  void *payload,
  TSLexer *lexer,
  const bool *valid_symbols
) {
  Scanner *scanner = (Scanner *)payload;

  // Handle interpolation start ${
  if (valid_symbols[STRING_INTERPOLATION_START]) {
    if (lexer->lookahead == '$') {
      lexer->advance(lexer, false);
      if (lexer->lookahead == '{') {
        lexer->advance(lexer, false);
        lexer->mark_end(lexer);
        lexer->result_symbol = STRING_INTERPOLATION_START;
        scanner->brace_depth = 1;
        return true;
      }
    }
  }

  // Handle interpolation end }
  if (valid_symbols[STRING_INTERPOLATION_END]) {
    if (lexer->lookahead == '}' && scanner->brace_depth > 0) {
      lexer->advance(lexer, false);
      lexer->mark_end(lexer);
      scanner->brace_depth = 0;
      lexer->result_symbol = STRING_INTERPOLATION_END;
      return true;
    }
  }

  // Handle string content (text before ${)
  if (valid_symbols[STRING_CONTENT]) {
    bool has_content = false;

    while (true) {
      // Stop at end of string
      if (lexer->lookahead == '"' || lexer->lookahead == 0) {
        break;
      }

      // Stop at escape sequence
      if (lexer->lookahead == '\\') {
        break;
      }

      // Stop at interpolation start ${
      if (lexer->lookahead == '$') {
        // Peek ahead for {
        lexer->mark_end(lexer);
        lexer->advance(lexer, false);
        if (lexer->lookahead == '{') {
          // Don't consume the ${, let it be handled by interpolation_start
          return has_content;
        }
        // Not ${, continue consuming as string content
        has_content = true;
        continue;
      }

      has_content = true;
      lexer->advance(lexer, false);
    }

    if (has_content) {
      lexer->mark_end(lexer);
      lexer->result_symbol = STRING_CONTENT;
      return true;
    }
  }

  return false;
}

#!/bin/bash
# PostToolUse hook: compile-check a .n file after it's edited or written

if ! command -v jq &> /dev/null; then
    echo "check-neon.sh: jq is required but not installed" >&2
    exit 1
fi

INPUT=$(cat)
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

if [[ "$FILE_PATH" != *.n ]] || [ ! -f "$FILE_PATH" ]; then
    exit 0
fi

PROJECT_DIR=$(git -C "$(dirname "$FILE_PATH")" rev-parse --show-toplevel 2>/dev/null) || PROJECT_DIR="$CLAUDE_PROJECT_DIR"

cd "$PROJECT_DIR" || exit 1
RELATIVE_PATH=$(realpath --relative-to="$PROJECT_DIR" "$FILE_PATH")

case "$RELATIVE_PATH" in
    tests/compile_errors/* | tests/compile_errors_must_fail/*) exit 0 ;;
esac

# A module case expecting a compile error fails --check by design, so only its
# formatting is checked.
case "$RELATIVE_PATH" in
    tests/modules/*/*)
        CASE=${RELATIVE_PATH#tests/modules/}
        EXPECTATION_FILE="tests/modules/${CASE%%/*}/main.n"
        ;;
    tests/modules_must_fail/*/*)
        CASE=${RELATIVE_PATH#tests/modules_must_fail/}
        EXPECTATION_FILE="tests/modules_must_fail/${CASE%%/*}/main.n"
        ;;
    *) EXPECTATION_FILE="" ;;
esac

if [ -z "$EXPECTATION_FILE" ] \
    || ! grep -qE '^[[:space:]]*// Expected compile error:' "$EXPECTATION_FILE" 2>/dev/null; then
    OUTPUT=$(cargo run -q -- --check "$FILE_PATH" 2>&1)
    STATUS=$?

    if [ "$STATUS" -eq 65 ]; then
        echo "$OUTPUT" >&2
        exit 2
    elif [ "$STATUS" -ne 0 ]; then
        echo "$OUTPUT" >&2
        exit 1
    fi
fi

case "$RELATIVE_PATH" in
    tests/scripts/* | tests/modules/* | tests/modules_must_fail/* | benches/* | examples/*) ;;
    *) exit 0 ;;
esac

OUTPUT=$(cargo run -q -- fmt --check "$FILE_PATH" 2>&1)
STATUS=$?

if [ "$STATUS" -eq 0 ]; then
    exit 0
elif [ "$STATUS" -eq 1 ]; then
    echo "check-neon.sh: $FILE_PATH is not formatted, run: cargo run -- fmt $FILE_PATH" >&2
    exit 2
else
    echo "$OUTPUT" >&2
    exit 1
fi

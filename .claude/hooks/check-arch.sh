#!/bin/bash
# PostToolUse hook: run the layer-boundary test after a src/*.rs file is edited

if ! command -v jq &> /dev/null; then
    echo "check-arch.sh: jq is required but not installed" >&2
    exit 1
fi

INPUT=$(cat)
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

PROJECT_DIR=$(git -C "$(dirname "$FILE_PATH")" rev-parse --show-toplevel 2>/dev/null) || PROJECT_DIR="$CLAUDE_PROJECT_DIR"

case "$(realpath --relative-to="$PROJECT_DIR" "$FILE_PATH" 2>/dev/null)" in
    src/*.rs) ;;
    *) exit 0 ;;
esac

cd "$PROJECT_DIR" || exit 1

OUTPUT=$(cargo test -q --test architecture 2>&1)
STATUS=$?

if [ "$STATUS" -eq 0 ]; then
    exit 0
elif echo "$OUTPUT" | grep -q "test result: FAILED"; then
    echo "$OUTPUT" >&2
    exit 2
else
    echo "$OUTPUT" >&2
    exit 1
fi

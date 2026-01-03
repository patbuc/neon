#!/bin/bash
# Notification hook for Claude Code
# Sends macOS notification when Claude needs user input

# Read JSON input from stdin
INPUT=$(cat)

# Parse notification type and message using jq (or fallback to grep/sed)
if command -v jq &> /dev/null; then
    NOTIFICATION_TYPE=$(echo "$INPUT" | jq -r '.notification_type // empty')
    MESSAGE=$(echo "$INPUT" | jq -r '.message // empty')
else
    # Fallback: simple grep extraction
    NOTIFICATION_TYPE=$(echo "$INPUT" | grep -o '"notification_type":"[^"]*"' | cut -d'"' -f4)
    MESSAGE=$(echo "$INPUT" | grep -o '"message":"[^"]*"' | cut -d'"' -f4)
fi

# Set title based on notification type
case "$NOTIFICATION_TYPE" in
    "permission_prompt")
        TITLE="Claude Code - Permission Required"
        MESSAGE="${MESSAGE:-Claude needs your permission}"
        ;;
    "idle_prompt")
        TITLE="Claude Code - Input Needed"
        MESSAGE="${MESSAGE:-Claude is waiting for your input}"
        ;;
    *)
        TITLE="Claude Code"
        MESSAGE="${MESSAGE:-Notification}"
        ;;
esac

# Send macOS notification with sound
osascript -e "display notification \"$MESSAGE\" with title \"$TITLE\" sound name \"Ping\"" 2>/dev/null

# Fallback: terminal bell if osascript fails
if [ $? -ne 0 ]; then
    printf '\a'
fi

exit 0

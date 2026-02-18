#!/bin/bash
set -e

HOOK_SRC=".hooks/pre-commit"
HOOK_DEST=".git/hooks/pre-commit"

if [ ! -f "$HOOK_SRC" ]; then
    echo "❌ Error: $HOOK_SRC not found!"
    exit 1
fi

echo "📦 Installing pre-commit hook..."
cp "$HOOK_SRC" "$HOOK_DEST"
chmod +x "$HOOK_DEST"

echo "✅ Pre-commit hook installed successfully!"

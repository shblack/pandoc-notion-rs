#!/bin/bash
set -e

# Create a temporary directory
TMP_DIR=$(mktemp -d)
echo "Working in directory: $TMP_DIR"

# Create markdown file with nested list
cat > "$TMP_DIR/nested_list.md" << 'EOF'
- First level item
  - Second level item
- Another first level item
EOF

echo "Created markdown file with nested list:"
cat "$TMP_DIR/nested_list.md"

# Convert to JSON using pandoc
pandoc -f markdown -t json "$TMP_DIR/nested_list.md" -o "$TMP_DIR/original.json"
echo "Converted to JSON. Original JSON saved to: $TMP_DIR/original.json"

# Display the JSON content
echo "JSON content:"
cat "$TMP_DIR/original.json"

# Copy for manual modification
cp "$TMP_DIR/original.json" "$TMP_DIR/modified.json"
echo "Created a copy for manual modification at: $TMP_DIR/modified.json"

echo ""
echo "Now you can:"
echo "1. Edit the file $TMP_DIR/modified.json to add spans"
echo "2. Compare the outputs with:"
echo "   pandoc -f json -t markdown $TMP_DIR/original.json -o $TMP_DIR/original.md"
echo "   pandoc -f json -t markdown $TMP_DIR/modified.json -o $TMP_DIR/modified.md"
echo "   diff $TMP_DIR/original.md $TMP_DIR/modified.md"
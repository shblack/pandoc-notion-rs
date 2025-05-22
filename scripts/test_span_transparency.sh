#!/bin/bash
set -e

# Create a temporary directory
TMP_DIR=$(mktemp -d)
echo "Working in temporary directory: $TMP_DIR"

# Create a Pandoc JSON file with both plain text and text wrapped in empty spans
cat > "$TMP_DIR/span_test.json" << 'EOF'
{
  "pandoc-api-version": [1, 23, 1],
  "meta": {},
  "blocks": [
    {
      "t": "Para",
      "c": [
        {
          "t": "Str",
          "c": "Plain"
        },
        {
          "t": "Space"
        },
        {
          "t": "Str",
          "c": "text"
        },
        {
          "t": "Space"
        },
        {
          "t": "Str",
          "c": "without"
        },
        {
          "t": "Space"
        },
        {
          "t": "Str",
          "c": "span."
        }
      ]
    },
    {
      "t": "Para",
      "c": [
        {
          "t": "Span",
          "c": [
            {
              "c": [],
              "t": "Emph"
            },
            {
              "c": [],
              "t": "Strong"
            },
            {
              "c": [],
              "t": "Strikeout"
            },
            {
              "c": [
                [
                  "",
                  [],
                  []
                ],
                []
              ],
              "t": "Span"
            }
          ]
        },
        {
          "t": "Span",
          "c": [
            {
              "t": "",
              "c": [],
              "classes": [],
              "attributes": []
            },
            [
              {
                "t": "Str",
                "c": "Text"
              },
              {
                "t": "Space"
              },
              {
                "t": "Str",
                "c": "with"
              },
              {
                "t": "Space"
              },
              {
                "t": "Str",
                "c": "empty"
              },
              {
                "t": "Space"
              },
              {
                "t": "Str",
                "c": "span."
              }
            ]
          ]
        }
      ]
    },
    {
      "t": "Para",
      "c": [
        {
          "t": "Span",
          "c": [
            {
              "t": "",
              "c": [],
              "classes": ["test-class"],
              "attributes": []
            },
            [
              {
                "t": "Str",
                "c": "Text"
              },
              {
                "t": "Space"
              },
              {
                "t": "Str",
                "c": "with"
              },
              {
                "t": "Space"
              },
              {
                "t": "Str",
                "c": "classed"
              },
              {
                "t": "Space"
              },
              {
                "t": "Str",
                "c": "span."
              }
            ]
          ]
        }
      ]
    }
  ]
}
EOF

echo "Created test file with plain text and text in empty spans"

# Define the formats to test
FORMATS=("markdown" "html" "latex" "docx" "rst")

# Convert to each format and compare
for FORMAT in "${FORMATS[@]}"; do
  echo "Testing conversion to $FORMAT format..."
  
  # Skip docx format if not on a system that can generate it
  if [ "$FORMAT" = "docx" ] && ! pandoc --list-output-formats | grep -q docx; then
    echo "  Skipping docx format (not available)"
    continue
  fi
  
  # Create output files
  pandoc -f json -t "$FORMAT" -o "$TMP_DIR/output.$FORMAT" "$TMP_DIR/span_test.json"
  
  # Special handling for docx (binary format)
  if [ "$FORMAT" = "docx" ]; then
    echo "  docx is a binary format - visual inspection required"
    echo "  File saved at: $TMP_DIR/output.docx"
    continue
  fi
  
  # Display the output for comparison
  echo "  Output in $FORMAT format:"
  cat "$TMP_DIR/output.$FORMAT"
  echo ""
done

echo "Test complete. Check if the empty spans affected the output in different formats."
echo "Temporary files are in: $TMP_DIR"
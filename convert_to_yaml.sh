#!/bin/bash

# Convert all TOML frontmatter posts to YAML frontmatter

for file in content/posts/*.md content/pages/*.md; do
    if [ ! -f "$file" ]; then
        continue
    fi

    echo "Processing: $file"

    # Check if file has TOML frontmatter (starts with +++)
    if head -n 1 "$file" | grep -q "^+++"; then
        # Extract TOML frontmatter
        toml_content=$(sed -n '/^+++$/,/^+++$/p' "$file" | sed '1d;$d')

        # Extract body (everything after second +++)
        body=$(sed -n '/^+++$/,/^+++$/!p;//d' "$file" | tail -n +2)

        # Convert TOML to YAML using Python
        yaml_content=$(python3 << EOF
import sys
import re

toml_text = """$toml_content"""

# Simple TOML to YAML converter for common fields
lines = toml_text.strip().split('\n')
for line in lines:
    line = line.strip()
    if not line or line.startswith('#'):
        continue

    # Match: key = "value" or key = value or key = ["val1", "val2"]
    if '=' in line:
        key, value = line.split('=', 1)
        key = key.strip()
        value = value.strip()

        # Remove inline comments (but not # inside strings)
        if '#' in value and not (value.startswith('"') and value.endswith('"')):
            # For non-string values, strip comments
            value = value.split('#')[0].strip()

        # Handle string values
        if value.startswith('"') and value.endswith('"'):
            value = value[1:-1]
            print(f"{key}: \"{value}\"")
        # Handle boolean/number values
        elif value.lower() in ['true', 'false'] or value.isdigit():
            print(f"{key}: {value}")
        # Handle arrays
        elif value.startswith('[') and value.endswith(']'):
            array_content = value[1:-1]
            items = [item.strip().strip('"') for item in array_content.split(',')]
            print(f"{key}:")
            for item in items:
                if item:
                    print(f"  - \"{item}\"")
        else:
            print(f"{key}: {value}")
EOF
)

        # Write new file with YAML frontmatter
        {
            echo "---"
            echo "$yaml_content"
            echo "---"
            echo ""
            echo "$body"
        } > "$file.tmp"

        mv "$file.tmp" "$file"
        echo "  ✓ Converted to YAML"
    else
        echo "  - Already YAML or no frontmatter"
    fi
done

echo ""
echo "Conversion complete!"

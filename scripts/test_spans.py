#!/usr/bin/env python3
import difflib
import json
import os
import subprocess
import tempfile


def run_command(cmd):
    """Run a command and return its output."""
    result = subprocess.run(cmd, shell=True, capture_output=True, text=True)
    if result.returncode != 0:
        print(f"Command failed: {cmd}")
        print(f"Error: {result.stderr}")
        exit(1)
    return result.stdout


def create_markdown_file(path):
    """Create a test markdown file with a nested list."""
    content = """
- First level item
  - Second level item
- Another first level item
"""
    with open(path, "w") as f:
        f.write(content)
    return content


def main():
    # Create temporary directory
    with tempfile.TemporaryDirectory() as temp_dir:
        # Create test files
        md_path = os.path.join(temp_dir, "test.md")
        create_markdown_file(md_path)

        # Convert to JSON
        json_path = os.path.join(temp_dir, "test.json")
        run_command(f"pandoc -f markdown -t json {md_path} -o {json_path}")

        # Load JSON
        with open(json_path, "r") as f:
            data = json.load(f)

        # Create a modified version with spans
        with_spans = modify_json_add_spans(data)
        spans_json_path = os.path.join(temp_dir, "with_spans.json")
        with open(spans_json_path, "w") as f:
            json.dump(with_spans, f, indent=2)

        # Convert both back to markdown
        original_md_path = os.path.join(temp_dir, "original_output.md")
        spans_md_path = os.path.join(temp_dir, "spans_output.md")

        # Use pandoc to convert from JSON to markdown
        run_command(f"pandoc -f json -t markdown {json_path} -o {original_md_path}")
        run_command(f"pandoc -f json -t markdown {spans_json_path} -o {spans_md_path}")

        # Compare the results
        with open(original_md_path, "r") as f:
            original_md = f.read()

        with open(spans_md_path, "r") as f:
            spans_md = f.read()

        print("Original markdown:")
        print(original_md)
        print("\nMarkdown with spans:")
        print(spans_md)

        if original_md == spans_md:
            print(
                "\nRESULT: The outputs are identical - empty spans don't affect the output."
            )
        else:
            print(
                "\nRESULT: The outputs are different - empty spans do affect the output."
            )
            diff = difflib.unified_diff(
                original_md.splitlines(),
                spans_md.splitlines(),
                fromfile="original",
                tofile="with spans",
            )
            for line in diff:
                print(line)

        # Also convert to HTML to see if spans appear there
        original_html_path = os.path.join(temp_dir, "original.html")
        spans_html_path = os.path.join(temp_dir, "with_spans.html")

        run_command(f"pandoc -f json -t html {json_path} -o {original_html_path}")
        run_command(f"pandoc -f json -t html {spans_json_path} -o {spans_html_path}")

        with open(original_html_path, "r") as f:
            original_html = f.read()

        with open(spans_html_path, "r") as f:
            spans_html = f.read()

        if original_html == spans_html:
            print("\nHTML RESULT: The HTML outputs are identical.")
        else:
            print("\nHTML RESULT: The HTML outputs are different.")
            print("\nOriginal HTML:")
            print(original_html)
            print("\nHTML with spans:")
            print(spans_html)

        # Keep these files around for inspection
        kept_dir = "test_output"
        os.makedirs(kept_dir, exist_ok=True)
        run_command(f"cp {original_md_path} {kept_dir}/original.md")
        run_command(f"cp {spans_md_path} {kept_dir}/with_spans.md")
        run_command(f"cp {original_html_path} {kept_dir}/original.html")
        run_command(f"cp {spans_html_path} {kept_dir}/with_spans.html")
        run_command(f"cp {json_path} {kept_dir}/original.json")
        run_command(f"cp {spans_json_path} {kept_dir}/with_spans.json")
        print(f"\nTest files preserved in {kept_dir}/")


def modify_json_add_spans(data):
    """Add empty spans around text elements in the JSON."""
    result = json.loads(json.dumps(data))  # Deep copy

    def process_node(node):
        if not isinstance(node, dict):
            return node

        # If this is a Plain node, modify its content to add spans
        if node.get("t") == "Plain" and "c" in node:
            # Get the content array
            content = node["c"]

            # Wrap all inline elements in a span
            node["c"] = [
                {
                    "t": "Span",
                    "c": [
                        {"t": "", "classes": [], "attributes": []},  # Empty attrs
                        content,  # Original content
                    ],
                }
            ]
        # Otherwise, recursively process all properties
        else:
            for key in node:
                if isinstance(node[key], dict):
                    node[key] = process_node(node[key])
                elif isinstance(node[key], list):
                    node[key] = [
                        process_node(item)
                        if isinstance(item, dict)
                        else [
                            process_node(subitem)
                            if isinstance(subitem, dict)
                            else subitem
                            for subitem in item
                        ]
                        if isinstance(item, list)
                        else item
                        for item in node[key]
                    ]
        return node

    # Process the entire document
    for i, block in enumerate(result["blocks"]):
        result["blocks"][i] = process_node(block)

    return result


if __name__ == "__main__":
    main()

#!/usr/bin/env python3
import difflib
import json
import subprocess
import tempfile
from pathlib import Path


def run_pandoc(args):
    """Run pandoc with the given arguments and return the output."""
    cmd = ["pandoc"] + args
    result = subprocess.run(cmd, capture_output=True, text=True)
    if result.returncode != 0:
        raise Exception(f"pandoc failed: {result.stderr}")
    return result.stdout


def create_test_file():
    """Create a test markdown file with a nested list."""
    content = """
- First level item
  - Second level item
- Another first level item
"""
    return content


def add_spans_to_json(json_data):
    """Add empty spans around all Str elements in the JSON."""
    if isinstance(json_data, dict):
        if json_data.get("t") == "Str":
            # Don't modify Str objects directly, they'll be wrapped by the parent
            return json_data

        for key, value in json_data.items():
            json_data[key] = add_spans_to_json(value)
        return json_data

    elif isinstance(json_data, list):
        # Check if this is a list of inline elements that we should wrap in a span
        if all(
            isinstance(item, dict) and item.get("t") in ["Str", "Space"]
            for item in json_data
        ):
            # Create an empty span that wraps all these elements
            span = {
                "t": "Span",
                "c": [
                    {"t": "", "classes": [], "attributes": []},  # Empty attributes
                    json_data,  # The original content
                ],
            }
            # Return a list with just the span (replacing all original elements)
            return [span]
        else:
            # Otherwise just process each element
            return [add_spans_to_json(item) for item in json_data]

    else:
        return json_data


def main():
    print("Testing how spans affect Pandoc output...")

    # Create temporary directory
    with tempfile.TemporaryDirectory() as temp_dir:
        temp_dir_path = Path(temp_dir)

        # Create test markdown file
        md_file = temp_dir_path / "test.md"
        md_file.write_text(create_test_file())
        print(f"Created test markdown file: {md_file}")

        # Convert to JSON
        json_text = run_pandoc(["-f", "markdown", "-t", "json", str(md_file)])
        json_data = json.loads(json_text)
        print("Converted to JSON")

        # Save original JSON
        original_json_file = temp_dir_path / "original.json"
        original_json_file.write_text(json.dumps(json_data, indent=2))

        # Create modified JSON with spans
        modified_json_data = add_spans_to_json(json_data)
        modified_json_file = temp_dir_path / "modified.json"
        modified_json_file.write_text(json.dumps(modified_json_data, indent=2))
        print("Created modified JSON with spans")

        # Convert both back to markdown
        original_md = run_pandoc(
            ["-f", "json", "-t", "markdown", str(original_json_file)]
        )
        modified_md = run_pandoc(
            ["-f", "json", "-t", "markdown", str(modified_json_file)]
        )

        # Save the outputs
        original_md_file = temp_dir_path / "original_output.md"
        modified_md_file = temp_dir_path / "modified_output.md"
        original_md_file.write_text(original_md)
        modified_md_file.write_text(modified_md)

        # Compare outputs
        print("\nOriginal Markdown Output:")
        print(original_md)
        print("\nModified Markdown Output (with spans):")
        print(modified_md)

        if original_md == modified_md:
            print("\nRESULT: IDENTICAL - Empty spans have no effect on markdown output")
        else:
            print("\nRESULT: DIFFERENT - Empty spans do affect markdown output")
            print("\nDifferences:")
            diff = difflib.unified_diff(
                original_md.splitlines(),
                modified_md.splitlines(),
                fromfile="without spans",
                tofile="with spans",
            )
            for line in diff:
                print(line)

        # Also try HTML output to see if spans appear there
        original_html = run_pandoc(
            ["-f", "json", "-t", "html", str(original_json_file)]
        )
        modified_html = run_pandoc(
            ["-f", "json", "-t", "html", str(modified_json_file)]
        )

        if original_html == modified_html:
            print(
                "\nHTML RESULT: IDENTICAL - Empty spans have no effect on HTML output"
            )
        else:
            print("\nHTML RESULT: DIFFERENT - Empty spans do affect HTML output")

        # Save files for inspection
        print(f"\nAll test files are saved in: {temp_dir}")
        print("You can inspect these files before the temp directory is cleaned up")
        input("Press Enter to continue...")


if __name__ == "__main__":
    main()

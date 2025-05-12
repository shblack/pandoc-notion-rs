#!/usr/bin/env python3
"""
md_to_pandoc_ast.py - Convert Markdown text to Pandoc AST (Abstract Syntax Tree)

This script converts Markdown text to Pandoc's AST format (JSON).
Input can be provided as a file, standard input, or a string argument.

Requirements:
- Python 3.6+
- pypandoc (install with: pip install pypandoc)
- pandoc (must be installed on your system)

Usage:
  python md_to_pandoc_ast.py [options] [markdown_text]
  
Options:
  -f, --file FILENAME   Read Markdown from specified file
  -o, --output FILENAME Write AST output to specified file (default: stdout)
  -p, --pretty          Pretty-print the JSON output (default)
  --compact             Compact JSON output (no pretty printing)
  -h, --help            Show this help message and exit

Examples:
  python md_to_pandoc_ast.py "# Hello World"
  echo "# Hello World" | python md_to_pandoc_ast.py
  python md_to_pandoc_ast.py -f input.md -o output.json
"""

import argparse
import json
import sys
import subprocess
import tempfile
from pathlib import Path


def convert_markdown_to_ast(markdown_text, pretty=True):
    """
    Convert Markdown text to Pandoc AST (as JSON)
    
    Args:
        markdown_text (str): The markdown text to convert
        pretty (bool): Whether to pretty-print the JSON output
    
    Returns:
        str: The Pandoc AST in JSON format
    """
    # Create a temporary file for the markdown content
    with tempfile.NamedTemporaryFile(suffix='.md', mode='w', delete=False) as temp_md:
        temp_md.write(markdown_text)
        temp_md_path = temp_md.name
    
    try:
        # Run pandoc with the AST output format (--to=json)
        cmd = ['pandoc', '--from=markdown', '--to=json', temp_md_path]
        result = subprocess.run(cmd, capture_output=True, text=True, check=True)
        
        # Parse the JSON to validate and optionally pretty-print
        ast_json = json.loads(result.stdout)
        
        if pretty:
            return json.dumps(ast_json, indent=2)
        else:
            return json.dumps(ast_json)
    
    except subprocess.CalledProcessError as e:
        raise RuntimeError(f"Pandoc conversion failed: {e.stderr}")
    except json.JSONDecodeError:
        raise ValueError("Failed to parse Pandoc output as JSON")
    finally:
        # Clean up the temporary file
        Path(temp_md_path).unlink()


def main():
    """Parse arguments and convert markdown to Pandoc AST"""
    parser = argparse.ArgumentParser(
        description="Convert Markdown text to Pandoc AST (JSON)",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  python md_to_pandoc_ast.py "# Hello World"
  echo "# Hello World" | python md_to_pandoc_ast.py
  python md_to_pandoc_ast.py -f input.md -o output.json
        """
    )
    
    parser.add_argument(
        'markdown_text', nargs='?', default=None,
        help='Markdown text to convert (if not specified, read from stdin or file)'
    )
    
    parser.add_argument(
        '-f', '--file', metavar='FILENAME', 
        help='Read Markdown from specified file'
    )
    
    parser.add_argument(
        '-o', '--output', metavar='FILENAME',
        help='Write AST output to specified file (default: stdout)'
    )
    
    formatting = parser.add_mutually_exclusive_group()
    formatting.add_argument(
        '-p', '--pretty', action='store_true', default=True,
        help='Pretty-print the JSON output (default)'
    )
    
    formatting.add_argument(
        '--compact', dest='pretty', action='store_false',
        help='Compact JSON output (no pretty printing)'
    )
    
    args = parser.parse_args()
    
    # Determine input source (priority: file, then argument, then stdin)
    markdown_text = None
    
    if args.file:
        try:
            with open(args.file, 'r', encoding='utf-8') as f:
                markdown_text = f.read()
        except IOError as e:
            sys.stderr.write(f"Error reading file '{args.file}': {e}\n")
            sys.exit(1)
    
    elif args.markdown_text:
        markdown_text = args.markdown_text
    
    else:
        # Check if there's input on stdin
        if not sys.stdin.isatty():
            markdown_text = sys.stdin.read()
        else:
            parser.print_help()
            sys.exit(1)
    
    try:
        ast_json = convert_markdown_to_ast(markdown_text, args.pretty)
        
        # Output to file or stdout
        if args.output:
            with open(args.output, 'w', encoding='utf-8') as f:
                f.write(ast_json)
        else:
            print(ast_json)
    
    except Exception as e:
        sys.stderr.write(f"Error: {e}\n")
        sys.exit(1)


if __name__ == '__main__':
    main()
#!/usr/bin/env python3
"""Create a new page with front matter."""
import sys
import re
from datetime import datetime, timezone
from pathlib import Path

def slugify(title: str) -> str:
    return re.sub(r"[^a-z0-9]+", "-", title.lower()).strip("-")

def main():
    if len(sys.argv) < 2:
        print("Usage: new_page.py <title>", file=sys.stderr)
        sys.exit(1)

    title = " ".join(sys.argv[1:])
    slug = slugify(title)
    iso_str = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")

    pages_dir = Path("content/pages")
    pages_dir.mkdir(parents=True, exist_ok=True)

    filename = pages_dir / f"{slug}.md"
    filename.write_text(
        f'---\ntitle: "{title}"\ndate: {iso_str}\n---\n\nYour content here...\n',
        encoding="utf-8",
    )
    print(f"Created: {filename}")

if __name__ == "__main__":
    main()

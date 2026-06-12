# Markdown → DOCX Converter

Converts Markdown to clean, well-formatted Word documents — no more ugly
copy-paste. Supports standard (CommonMark + GFM) Markdown, Obsidian-flavoured
Markdown, and Mermaid charts (rendered as embedded images).

## What's supported

**Standard Markdown**

- Headings (styled, colour-graded H1–H6)
- Bold / italic / strikethrough / inline code
- Links (real Word hyperlinks)
- Bullet, numbered, and nested lists
- Task lists (`- [ ]` / `- [x]` → ☐ / ☑)
- Blockquotes with a left accent bar
- Fenced code blocks (monospace, shaded background)
- Tables (header shading, column alignment, clean borders)
- Horizontal rules and images

**Obsidian extensions**

- YAML frontmatter (stripped; `title:` becomes the document title)
- Callouts (`> [!NOTE]`, `> [!WARNING]`, …) with icons, colours, custom
  titles, and collapsible (`-`/`+`) markers
- Wikilinks `[[Page]]` and `[[Page|alias]]`
- Embeds `![[image.png]]` (including `![[image.png|300]]`)
- `==Highlights==`
- `#tags` (rendered as inline code chips)
- `%%comments%%` (stripped)
- Block references (`^block-id`, stripped)
- Inline `$math$` and block `$$math$$` (rendered as code so they stay readable)

**Mermaid**

````
```mermaid
flowchart LR
    A --> B
```
````

Fenced `mermaid` blocks are rendered to PNG via mermaid-cli and embedded,
centred, in the document. If mermaid-cli isn't installed, the diagram source
is included as a code block with a notice instead — conversion never fails.

## Setup

```bash
pip install -r requirements.txt

# Optional, required only for Mermaid chart rendering:
npm install -g @mermaid-js/mermaid-cli
```

## Usage

```bash
# Single file → notes.docx next to it
python convert.py notes.md

# Choose the output path
python convert.py notes.md -o report.docx

# Combine multiple files into one document
python convert.py ch1.md ch2.md ch3.md -o book.docx

# Convert many files separately (no -o)
python convert.py *.md
```

Try it on the included sample:

```bash
python convert.py sample.md
```

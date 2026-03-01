---
name: google-slides
description: Work with Google Slides - read presentation content, create slides, insert/replace text, and respond to comments. Use this skill when handling comments from Google Slides that mention you (proto, oliver, maggie, etc.).
allowed-tools: Bash(google-slides:*)
---

# Google Slides Collaboration Skill

## Overview

This skill enables you to collaborate on Google Slides as a digital co-author. Users can share presentations with you and request help via comments. You respond through comment replies and can read/modify presentation content.

## When to Use This Skill

Use this skill when:
- You receive a task from Google Slides (check `incoming_email/*_slides_comment.json`)
- The user has mentioned you in a presentation comment (proto, oliver, @proto, etc.)
- You need to read presentation content, add/edit slides, or respond to feedback

## CLI Commands Reference

### Reading Presentations

```bash
# List all presentations shared with you
google-slides list-presentations

# Read presentation content as plain text
google-slides read-presentation <presentation_id>

# Get presentation structure (slide IDs, element IDs)
google-slides get-presentation <presentation_id>

# Get raw JSON structure (for finding element IDs)
google-slides get-presentation <presentation_id> --json
```

### Working with Comments

```bash
# List all comments on a presentation
google-slides list-comments <presentation_id>

# Reply to a comment
google-slides reply-comment <presentation_id> <comment_id> "Your reply message"
```

### Editing Presentations

```bash
# Replace all occurrences of text
google-slides replace-all-text <presentation_id> --find="old text" --replace="new text"

# Insert text into a specific shape/placeholder
google-slides insert-text <presentation_id> --object-id="SHAPE_ID" --text="Hello World"

# Create a new slide (layouts: BLANK, TITLE, TITLE_AND_BODY, etc.)
google-slides create-slide <presentation_id> --layout=TITLE_AND_BODY

# Delete a slide
google-slides delete-slide <presentation_id> --slide-id="SLIDE_OBJECT_ID"

# Batch update (advanced operations)
google-slides batch-update <presentation_id> '<json_requests>'
```

### Available Slide Layouts

- `BLANK` - Empty slide
- `TITLE` - Title slide with centered title and subtitle
- `TITLE_AND_BODY` - Title with body content area
- `TITLE_AND_TWO_COLUMNS` - Title with two columns
- `TITLE_ONLY` - Title at top only
- `SECTION_HEADER` - Section divider slide
- `ONE_COLUMN_TEXT` - Single column text
- `MAIN_POINT` - Main point highlight
- `BIG_NUMBER` - Large number display

## Workflow

### 1. Understanding the Request

When triggered by a Google Slides comment:

1. Read the incoming comment from `incoming_email/email.html` or the comment JSON file
2. Note the **presentation ID**, **comment ID**, and **quoted content** (if any)
3. Check which slide the comment is anchored to

### 2. Reading Presentation Content

**Read the presentation content:**

```bash
# Get text content
google-slides read-presentation <presentation_id>

# Get structure with element IDs
google-slides get-presentation <presentation_id>
```

### 3. Finding Element IDs

To insert or edit text in specific shapes, you need the object ID:

```bash
# Get detailed JSON structure
google-slides get-presentation <presentation_id> --json | grep -A 5 '"objectId"'
```

The output shows element IDs like `SLIDES_API123_0`, `SLIDES_API123_1`, etc.

### 4. Making Edits

**Example: Create a title slide**

```bash
# Create a title slide at the beginning
google-slides create-slide 1abc123xyz --layout=TITLE --index=0
```

**Example: Add text to a placeholder**

```bash
# First get the presentation structure to find element IDs
google-slides get-presentation 1abc123xyz --json

# Then insert text into the title placeholder
google-slides insert-text 1abc123xyz --object-id=SLIDES_API123_1 --text="Presentation Title"
```

**Example: Replace text throughout the presentation**

```bash
# Replace all instances of a word
google-slides replace-all-text 1abc123xyz --find="PLACEHOLDER" --replace="Actual Content"
```

### 5. Responding to the User

After completing the task, reply to the comment:

```bash
google-slides reply-comment 1abc123xyz COMMENT_ID "Done! I've created a new title slide with the content you requested."
```

## Example Interaction

**User Comment:** "@proto add a title slide for this presentation"

**Your Response:**

1. Check current structure:
```bash
google-slides get-presentation 1abc123xyz
```

2. Create a title slide:
```bash
google-slides create-slide 1abc123xyz --layout=TITLE --index=0
```

3. Get the new slide's element IDs:
```bash
google-slides get-presentation 1abc123xyz --json
```

4. Add title text (using the found element ID):
```bash
google-slides insert-text 1abc123xyz --object-id=SLIDES_API_NEW_1 --text="My Presentation Title"
```

5. Reply to confirm:
```bash
google-slides reply-comment 1abc123xyz AAAB123xyz "Done! I've added a title slide with the title 'My Presentation Title' at the beginning of your presentation."
```

## Analyzing Slide Layout & Preventing Text Overflow

**IMPORTANT:** Before inserting or editing text, always analyze the slide structure to avoid text overflow.

### Check Available Capacity

```bash
# Analyze all slides with element sizes and text capacities
google-slides get-presentation <presentation_id> --analyze

# Analyze a specific slide in detail
google-slides analyze-slide <presentation_id> <slide_object_id>
```

### Understanding the Analysis Output

```
┌─ element_id [Shape]
│  Position: (50, 30) pt
│  Size: 620 x 60 pt (8.6" x 0.8")
│  Placeholder Type: TITLE
│  Font Size: 24pt
│  Text: 25 / ~51 chars (49%) ✓ OK
│  Remaining: ~26 chars
│  Content: "My Presentation Title"
└─────────────────────────────────
```

### Capacity Guidelines

| Element Type | Font Size | Recommended Max | Warning Threshold |
|-------------|-----------|-----------------|-------------------|
| Title | 24pt | 50 chars | 40 chars |
| Subtitle | 18pt | 80 chars | 65 chars |
| Body Text | 14pt | 500 chars | 400 chars |
| Bullet Point | 14pt | 80 chars per line | 65 chars |

### Best Practices to Avoid Overflow

1. **Always check capacity first:**
   ```bash
   google-slides analyze-slide <id> <slide_id>
   ```

2. **Watch for warning indicators:**
   - `⚠️ OVERFLOW RISK` - Text exceeds 90% capacity
   - `⚡ Near capacity` - Text exceeds 70% capacity
   - `✓ OK` - Safe text length

3. **For long content:**
   - Split across multiple slides
   - Use bullet points
   - Reduce font size (if layout allows)
   - Create additional body text boxes

4. **When editing existing text:**
   - Check remaining capacity before adding
   - Consider removing old text first

## Inserting Images

### Smart Image Placement Workflow

**IMPORTANT:** Always use the smart placement workflow to avoid overlapping content:

```bash
# Step 1: Search for images (using Unsplash)
google-slides search-image --query="professional meeting" --count=5 --orientation=landscape

# Step 2: Find available space on the slide
google-slides find-space <presentation_id> <slide_id> --min-width=200 --min-height=150

# Step 3: Insert the image at the recommended position
google-slides insert-image <presentation_id> \
  --url="<URL from search results>" \
  --page-id="<slide_id>" \
  --x=<x from find-space> --y=<y from find-space> \
  --width=200 --height=150
```

### Image Search

Use Unsplash to find relevant stock images:

```bash
# Search for images
google-slides search-image --query="technology innovation" --count=5

# Filter by orientation
google-slides search-image --query="nature landscape" --orientation=landscape
google-slides search-image --query="portrait photo" --orientation=portrait
google-slides search-image --query="app icon" --orientation=squarish
```

**Orientations:**
- `landscape` - Wide images (good for backgrounds, headers)
- `portrait` - Tall images (good for sidebars, full-slide portraits)
- `squarish` - Square-ish images (good for icons, thumbnails)

### Finding Available Space

Before inserting an image, check for available space:

```bash
# Find space for a 200x150 pt image
google-slides find-space <presentation_id> <slide_id> --min-width=200 --min-height=150

# Output shows recommended positions that don't overlap existing content
```

The `find-space` command analyzes existing elements and suggests positions where the image won't overlap text or other content.

### Direct Image Insertion

```bash
# Insert an image from a public URL
google-slides insert-image <presentation_id> \
  --url="https://example.com/image.png" \
  --page-id="p.SLIDE_ID" \
  --x=100 --y=100 \
  --width=200 --height=150

# Image will be positioned at (x, y) in points from top-left
# Width and height are optional - image keeps aspect ratio if omitted
```

### Image Requirements
- URL must be **publicly accessible**
- Size: < 50MB
- Resolution: < 25 megapixels
- Formats: PNG, JPEG, GIF

### For Private Images
If you need to insert a private/local image:
1. Upload to a public storage (Azure Blob with SAS URL, Google Cloud Storage signed URL)
2. Use the generated public URL with `insert-image`

### Image Placement Best Practices

1. **Always check available space first** - Use `find-space` before inserting
2. **Match image orientation to available space** - Use landscape for wide areas, portrait for tall areas
3. **Consider content hierarchy** - Place images after or beside related text
4. **Use appropriate sizes:**
   - Small icons: 50-100 pt
   - Inline images: 150-250 pt
   - Feature images: 300-400 pt
   - Full-width: 600+ pt

## Notes

- Use `--json` flag with `get-presentation` to find exact element IDs
- Use `--analyze` flag to see element sizes and text capacities
- Element IDs for placeholders typically follow the pattern `{slideId}_i{index}`
- Always verify the presentation structure before making edits
- Always check text capacity before inserting long content
- The `replace-all-text` command is useful for templated content

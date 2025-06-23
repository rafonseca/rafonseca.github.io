# Claude Development Notes

This document contains information for Claude about this blog project.

## Project Overview

This is a simple HTML/CSS/JS blog that replaces Jekyll. The main goal was to remove date-centric display and create a clean, minimal blog focused on content over publication dates.

## Key Features

- **No dates displayed** - Posts are organized by tags, not chronologically
- **Tag filtering** - Click tags to filter posts
- **Dark/light mode** - Toggle with persistence via localStorage
- **Clean URLs** - Uses `/posts/title/` structure (no .html extensions)
- **Responsive design** - Works on mobile and desktop
- **RSS feed generation** - Node.js script for syndication

## Project Structure

```
/
├── index.html          # Main blog page
├── style.css           # All styles with CSS variables for theming
├── script.js           # Tag filtering and theme toggle logic
├── posts.js            # Post data (title, tags, preview, URL)
├── posts/              # Individual post directories
│   └── title/          # Each post gets its own directory
│       └── index.html  # Clean URLs without .html
├── generate-rss.js     # RSS feed and sitemap generator
├── Makefile           # Development commands
└── CLAUDE.md          # This file
```

## Development Commands

Use the Makefile for common tasks:

- `make serve` - Start local development server
- `make build` - Generate RSS feed and sitemap  
- `make test` - Verify all files are present
- `make clean` - Remove generated files

## Adding New Posts

1. Create new directory in `posts/` (e.g., `posts/new-post-title/`)
2. Add `index.html` file with post content
3. Update `posts.js` with new post data:
   ```javascript
   {
       title: "Post Title",
       slug: "post-title",
       tags: ["tag1", "tag2"],
       preview: "Brief description...",
       content: "posts/post-title/"
   }
   ```
4. Run `make build` to regenerate RSS feed

## Design Principles

- **Simplicity over features** - Minimal dependencies, vanilla JS
- **Content focus** - No date pressure, posts grouped by topic
- **Fast loading** - Single CSS/JS files, no frameworks
- **Accessibility** - Semantic HTML, proper contrast ratios
- **Mobile first** - Responsive design with clean typography

## Testing

- Test locally with `make serve`
- Verify tag filtering works
- Check dark/light mode toggle
- Ensure clean URLs work: `http://localhost:8000/posts/title/`

## Deployment

This blog is designed for GitHub Pages deployment at the root level for clean URLs like:
- `https://rafonseca.github.io/posts/stackable-sql-statements/`

## Technical Notes

- Uses CSS custom properties for theming
- JavaScript handles theme persistence via localStorage
- RSS generation requires Node.js but blog itself is static
- Clean URLs work via GitHub Pages serving `index.html` from directories
- All posts use consistent theming and navigation
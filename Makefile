# Simple Blog Makefile
# Commands for managing the blog

.PHONY: help serve build rss clean test

# Default target
help:
	@echo "Available commands:"
	@echo "  serve    - Start local development server"
	@echo "  build    - Generate RSS feed and sitemap"
	@echo "  rss      - Generate RSS feed only"
	@echo "  test     - Test blog locally"
	@echo "  clean    - Clean generated files"
	@echo "  help     - Show this help message"

# Start local development server
serve:
	@echo "Starting local server at http://localhost:8000"
	@command -v python3 >/dev/null 2>&1 && python3 -m http.server 8000 || \
	command -v python >/dev/null 2>&1 && python -m http.server 8000 || \
	command -v php >/dev/null 2>&1 && php -S localhost:8000 || \
	(command -v node >/dev/null 2>&1 && npx serve . -p 8000) || \
	echo "No suitable server found. Install Python, PHP, or Node.js"

# Generate RSS feed and sitemap
build:
	@echo "Generating RSS feed and sitemap..."
	@command -v node >/dev/null 2>&1 && node generate-rss.js || \
	echo "Node.js required for RSS generation"

# Generate RSS feed only
rss:
	@echo "Generating RSS feed..."
	@command -v node >/dev/null 2>&1 && node generate-rss.js || \
	echo "Node.js required for RSS generation"

# Test blog functionality
test:
	@echo "Testing blog..."
	@echo "✓ Checking main files exist..."
	@test -f index.html || (echo "✗ index.html missing" && exit 1)
	@test -f style.css || (echo "✗ style.css missing" && exit 1)
	@test -f script.js || (echo "✗ script.js missing" && exit 1)
	@test -f posts.js || (echo "✗ posts.js missing" && exit 1)
	@echo "✓ Checking posts directory..."
	@test -d posts || (echo "✗ posts directory missing" && exit 1)
	@echo "✓ All tests passed!"

# Clean generated files
clean:
	@echo "Cleaning generated files..."
	@rm -f feed.xml sitemap.xml
	@echo "✓ Cleaned RSS feed and sitemap"
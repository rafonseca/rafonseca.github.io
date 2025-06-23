// Theme management
function initTheme() {
    const savedTheme = localStorage.getItem('theme');
    const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
    
    if (savedTheme === 'dark' || (!savedTheme && prefersDark)) {
        document.body.classList.add('dark-theme');
        updateThemeIcon(true);
    }
}

function toggleTheme() {
    const isDark = document.body.classList.toggle('dark-theme');
    localStorage.setItem('theme', isDark ? 'dark' : 'light');
    updateThemeIcon(isDark);
}

function updateThemeIcon(isDark) {
    const themeToggle = document.getElementById('theme-toggle');
    themeToggle.textContent = isDark ? '☀️' : '🌙';
}

// Post management
let currentFilter = 'all';

function renderPosts(filteredPosts = posts) {
    const postsContainer = document.getElementById('posts');
    
    if (filteredPosts.length === 0) {
        postsContainer.innerHTML = '<p>No posts found for the selected tag.</p>';
        return;
    }
    
    postsContainer.innerHTML = filteredPosts.map(post => `
        <article class="post-card">
            <h2 class="post-title">
                <a href="${post.content}">${post.title}</a>
            </h2>
            <div class="post-tags">
                ${post.tags.map(tag => 
                    `<span class="post-tag" onclick="filterByTag('${tag}')">${tag}</span>`
                ).join('')}
            </div>
            <p class="post-preview">${post.preview}</p>
        </article>
    `).join('');
}

function createTagButtons() {
    const allTags = [...new Set(posts.flatMap(post => post.tags))].sort();
    const tagButtonsContainer = document.getElementById('tag-buttons');
    
    tagButtonsContainer.innerHTML = allTags.map(tag => 
        `<button class="tag-filter" data-tag="${tag}" onclick="filterByTag('${tag}')">${tag}</button>`
    ).join('');
}

function filterByTag(tag) {
    currentFilter = tag;
    
    // Update active button
    document.querySelectorAll('.tag-filter').forEach(btn => {
        btn.classList.remove('active');
        if (btn.dataset.tag === tag || (tag === 'all' && btn.dataset.tag === 'all')) {
            btn.classList.add('active');
        }
    });
    
    // Filter posts
    if (tag === 'all') {
        renderPosts(posts);
    } else {
        const filteredPosts = posts.filter(post => post.tags.includes(tag));
        renderPosts(filteredPosts);
    }
}

// Search functionality (bonus feature)
function searchPosts(query) {
    if (!query.trim()) {
        renderPosts(posts);
        return;
    }
    
    const filteredPosts = posts.filter(post => 
        post.title.toLowerCase().includes(query.toLowerCase()) ||
        post.preview.toLowerCase().includes(query.toLowerCase()) ||
        post.tags.some(tag => tag.toLowerCase().includes(query.toLowerCase()))
    );
    
    renderPosts(filteredPosts);
}

// Initialize everything when page loads
document.addEventListener('DOMContentLoaded', function() {
    initTheme();
    createTagButtons();
    renderPosts();
    
    // Event listeners
    document.getElementById('theme-toggle').addEventListener('click', toggleTheme);
    
    // Handle "All Posts" button
    document.querySelector('[data-tag="all"]').addEventListener('click', () => filterByTag('all'));
});

// Handle system theme changes
window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', function(e) {
    if (!localStorage.getItem('theme')) {
        if (e.matches) {
            document.body.classList.add('dark-theme');
            updateThemeIcon(true);
        } else {
            document.body.classList.remove('dark-theme');
            updateThemeIcon(false);
        }
    }
});
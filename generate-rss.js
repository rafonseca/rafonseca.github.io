#!/usr/bin/env node

// Simple RSS feed generator
// Run with: node generate-rss.js

const fs = require('fs');
const path = require('path');

// Import posts data
const postsData = fs.readFileSync('./posts.js', 'utf8');
const posts = eval(postsData.replace('const posts = ', ''));

const siteConfig = {
    title: 'Renan A Fonseca',
    description: 'some random thoughts',
    url: 'https://rafonseca.github.io',
    author: 'Renan Alves Fonseca',
    email: 'renanfonseca@gmail.com'
};

function generateRSS() {
    const now = new Date().toUTCString();
    
    const rssItems = posts.map(post => `
    <item>
        <title><![CDATA[${post.title}]]></title>
        <link>${siteConfig.url}/${post.content}</link>
        <guid>${siteConfig.url}/${post.content}</guid>
        <description><![CDATA[${post.preview}]]></description>
        <category>${post.tags.join(', ')}</category>
    </item>`).join('');

    const rss = `<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">
    <channel>
        <title>${siteConfig.title}</title>
        <description>${siteConfig.description}</description>
        <link>${siteConfig.url}/</link>
        <atom:link href="${siteConfig.url}/feed.xml" rel="self" type="application/rss+xml"/>
        <language>en-us</language>
        <managingEditor>${siteConfig.email} (${siteConfig.author})</managingEditor>
        <webMaster>${siteConfig.email} (${siteConfig.author})</webMaster>
        <lastBuildDate>${now}</lastBuildDate>
        <pubDate>${now}</pubDate>
        <ttl>60</ttl>
        ${rssItems}
    </channel>
</rss>`;

    fs.writeFileSync('./feed.xml', rss);
    console.log('RSS feed generated: feed.xml');
}

// Generate sitemap as bonus
function generateSitemap() {
    const urls = [
        `${siteConfig.url}/`,
        ...posts.map(post => `${siteConfig.url}/${post.content}`)
    ];

    const sitemap = `<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
    ${urls.map(url => `
    <url>
        <loc>${url}</loc>
        <changefreq>weekly</changefreq>
        <priority>0.8</priority>
    </url>`).join('')}
</urlset>`;

    fs.writeFileSync('./sitemap.xml', sitemap);
    console.log('Sitemap generated: sitemap.xml');
}

if (require.main === module) {
    generateRSS();
    generateSitemap();
}
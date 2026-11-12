// ===========================================================================
// discord-rust Documentation & Wiki Engine
// ===========================================================================

const DOCS_CATALOG = [
  {
    category: "Getting Started",
    items: [
      { id: "intro", title: "Introduction & Philosophy", file: "content/intro.md" },
      { id: "quickstart", title: "Step-by-Step Quickstart", file: "content/quickstart.md" },
      { id: "bot-builder", title: "BotBuilder & Lifecycle", file: "content/bot-builder.md" }
    ]
  },
  {
    category: "Core Architecture",
    items: [
      { id: "gateway", title: "Gateway & Event Dispatcher", file: "content/gateway.md" },
      { id: "rest-api", title: "REST API Client & Limits", file: "content/rest-api.md" },
      { id: "cache", title: "State Cache & Edit Tracking", file: "content/cache.md" },
      { id: "sharding", title: "Multi-Shard Auto Scaling", file: "content/sharding.md" },
      { id: "thread-pool", title: "Thread Pool & Pooled Dispatch", file: "content/thread-pool.md" }
    ]
  },
  {
    category: "Features & Components",
    items: [
      { id: "commands", title: "Slash Commands & Options", file: "content/commands.md" },
      { id: "macros", title: "Proc Macros & #[command]", file: "content/macros.md" },
      { id: "ui-components", title: "UI Components & Paginator", file: "content/ui-components.md" },
      { id: "voice", title: "Voice & FFmpeg Streaming", file: "content/voice.md" },
      { id: "webhooks", title: "Webhooks Execution", file: "content/webhooks.md" },
      { id: "models", title: "Domain Models & Assets", file: "content/models.md" },
      { id: "logging-errors", title: "Logging & Error Handling", file: "content/logging-errors.md" }
    ]
  }
];

const cache = new Map();
let currentDocId = null;

// ---------------------------------------------------------------------------
// Syntax Highlighting & Marked Parser Configuration (marked v15+)
// ---------------------------------------------------------------------------

// Register TOML grammar if not built-in
if (typeof hljs !== 'undefined' && !hljs.getLanguage('toml')) {
  hljs.registerLanguage('toml', (hljs) => ({
    name: 'TOML',
    aliases: ['toml'],
    case_insensitive: false,
    contains: [
      hljs.COMMENT('#', '$'),
      {
        className: 'section',
        begin: /^\s*\[+/,
        end: /\]+/
      },
      {
        className: 'string',
        begin: /"/,
        end: /"/
      },
      {
        className: 'number',
        begin: /\b\d+(\.\d+)?\b/
      },
      {
        className: 'literal',
        begin: /\b(true|false)\b/
      },
      {
        className: 'attr',
        begin: /^[a-zA-Z0-9_-]+(?=\s*=)/
      }
    ]
  }));
}

// In marked v15+, custom code rendering is provided via marked.use({ renderer: { code(...) } })
if (typeof marked !== 'undefined') {
  marked.use({
    renderer: {
      code(token) {
        const text = (typeof token === 'object' && token.text !== undefined) ? token.text : String(token || '');
        const rawLang = (typeof token === 'object' && token.lang !== undefined) ? token.lang : '';
        const lang = (rawLang.match(/^\S*/) || [''])[0];

        let highlighted = null;
        if (typeof hljs !== 'undefined') {
          const targetLang = (lang && hljs.getLanguage(lang)) ? lang : (lang === 'toml' ? 'toml' : null);
          if (targetLang && hljs.getLanguage(targetLang)) {
            try {
              highlighted = hljs.highlight(text, { language: targetLang }).value;
            } catch (e) {}
          } else if (lang !== 'text') {
            try {
              highlighted = hljs.highlightAuto(text).value;
            } catch (e) {}
          }
        }

        const className = lang ? `language-${lang} hljs hljs-done` : 'hljs hljs-done';
        const codeHtml = highlighted !== null ? highlighted : text.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
        return `<pre><code class="${className}">${codeHtml}</code></pre>\n`;
      }
    }
  });
}

// ---------------------------------------------------------------------------
// View Switching & Navigation
// ---------------------------------------------------------------------------

function showLanding() {
  document.getElementById('landing-view').style.display = 'block';
  document.getElementById('wiki-view').style.display = 'none';
  document.querySelectorAll('.nav-link').forEach(el => el.classList.remove('active'));
  const homeBtn = document.getElementById('nav-home');
  if (homeBtn) homeBtn.classList.add('active');
  window.location.hash = '';
  window.scrollTo({ top: 0, behavior: 'smooth' });
}

function showWiki(docId = 'intro') {
  document.getElementById('landing-view').style.display = 'none';
  document.getElementById('wiki-view').style.display = 'flex';
  document.querySelectorAll('.nav-link').forEach(el => el.classList.remove('active'));
  const wikiBtn = document.getElementById('nav-wiki');
  if (wikiBtn) wikiBtn.classList.add('active');
  loadDocument(docId);
}

// ---------------------------------------------------------------------------
// Document Loading & Rendering
// ---------------------------------------------------------------------------

async function loadDocument(docId) {
  currentDocId = docId;
  const contentEl = document.getElementById('markdown-render');
  
  // Highlight active sidebar item
  document.querySelectorAll('.sidebar-nav-link').forEach(link => {
    link.classList.toggle('active', link.dataset.docId === docId);
  });

  // Show loading spinner
  contentEl.innerHTML = `
    <div class="loading-spinner">
      <div class="spinner"></div>
      <p>Loading document...</p>
    </div>
  `;

  let item = null;
  for (const cat of DOCS_CATALOG) {
    const found = cat.items.find(i => i.id === docId);
    if (found) { item = found; break; }
  }

  if (!item) {
    contentEl.innerHTML = `<h1>Document Not Found</h1><p>The document <code>${docId}</code> does not exist.</p>`;
    return;
  }

  try {
    let markdown = cache.get(item.file);
    if (!markdown) {
      const resp = await fetch(item.file);
      if (!resp.ok) throw new Error(`HTTP ${resp.status}`);
      markdown = await resp.text();
      cache.set(item.file, markdown);
    }

    // Process GitHub style callout alerts: > [!NOTE], > [!TIP], > [!WARNING]
    markdown = markdown.replace(/^>\s*\[!(NOTE|INFO)\]\s*(.*)$/gim, '<div class="alert alert-note"><strong>Note:</strong> $2');
    markdown = markdown.replace(/^>\s*\[!(TIP|SUCCESS)\]\s*(.*)$/gim, '<div class="alert alert-tip"><strong>Tip:</strong> $2');
    markdown = markdown.replace(/^>\s*\[!(WARNING|CAUTION|IMPORTANT)\]\s*(.*)$/gim, '<div class="alert alert-warning"><strong>Warning:</strong> $2');

    const html = marked.parse(markdown);
    contentEl.innerHTML = html;

    // Highlight any unhighlighted code blocks in DOM
    if (typeof hljs !== 'undefined') {
      contentEl.querySelectorAll('pre code').forEach(code => {
        if (!code.classList.contains('hljs-done')) {
          try {
            hljs.highlightElement(code);
            code.classList.add('hljs-done');
          } catch (e) {}
        }
      });
    }

    // Attach copy buttons to pre blocks
    attachCodeCopyButtons(contentEl);

    // Build Table of Contents
    buildTableOfContents(contentEl);

    // Scroll to top of reading pane
    window.scrollTo({ top: 0, behavior: 'smooth' });
    window.location.hash = docId;
  } catch (err) {
    contentEl.innerHTML = `<h1>Error Loading Document</h1><p>Failed to load <code>${item.file}</code>: ${err.message}</p>`;
  }
}

const ICON_COPY_SVG = `<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path></svg>`;
const ICON_CHECK_SVG = `<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#57F287" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"></polyline></svg>`;

// ---------------------------------------------------------------------------
// Code Block Copy Buttons
// ---------------------------------------------------------------------------

function attachCodeCopyButtons(container) {
  const codeBlocks = container.querySelectorAll('pre');
  codeBlocks.forEach(pre => {
    const btn = document.createElement('button');
    btn.className = 'code-copy-btn';
    btn.innerHTML = `${ICON_COPY_SVG} <span>Copy</span>`;
    btn.addEventListener('click', () => {
      const code = pre.querySelector('code') ? pre.querySelector('code').innerText : pre.innerText;
      navigator.clipboard.writeText(code).then(() => {
        btn.innerHTML = `${ICON_CHECK_SVG} <span style="color: #57F287;">Copied!</span>`;
        setTimeout(() => { btn.innerHTML = `${ICON_COPY_SVG} <span>Copy</span>`; }, 2000);
      });
    });
    pre.appendChild(btn);
  });
}

// ---------------------------------------------------------------------------
// Dynamic Table of Contents (Right Rail)
// ---------------------------------------------------------------------------

function buildTableOfContents(container) {
  const tocNav = document.getElementById('toc-nav');
  if (!tocNav) return;
  tocNav.innerHTML = '';

  const headings = container.querySelectorAll('h2, h3');
  if (headings.length === 0) {
    document.querySelector('.wiki-toc').style.display = 'none';
    return;
  }
  document.querySelector('.wiki-toc').style.display = 'block';

  headings.forEach((heading, idx) => {
    const id = heading.id || `section-${idx}`;
    heading.id = id;

    const li = document.createElement('li');
    li.style.paddingLeft = heading.tagName === 'H3' ? '0.75rem' : '0';
    
    const a = document.createElement('a');
    a.className = 'toc-link';
    a.href = `#${id}`;
    a.textContent = heading.textContent;
    a.addEventListener('click', (e) => {
      e.preventDefault();
      heading.scrollIntoView({ behavior: 'smooth' });
    });

    li.appendChild(a);
    tocNav.appendChild(li);
  });
}

// ---------------------------------------------------------------------------
// Sidebar Building & Search
// ---------------------------------------------------------------------------

function buildSidebar() {
  const navContainer = document.getElementById('sidebar-nav-container');
  if (!navContainer) return;
  navContainer.innerHTML = '';

  DOCS_CATALOG.forEach(category => {
    const group = document.createElement('div');
    group.className = 'sidebar-group';

    const title = document.createElement('div');
    title.className = 'sidebar-group-title';
    title.textContent = category.category;
    group.appendChild(title);

    const ul = document.createElement('ul');
    ul.className = 'sidebar-nav';

    category.items.forEach(item => {
      const li = document.createElement('li');
      li.className = 'sidebar-nav-item';

      const a = document.createElement('a');
      a.className = 'sidebar-nav-link';
      a.textContent = item.title;
      a.dataset.docId = item.id;
      a.addEventListener('click', (e) => {
        e.preventDefault();
        loadDocument(item.id);
      });

      li.appendChild(a);
      ul.appendChild(li);
    });

    group.appendChild(ul);
    navContainer.appendChild(group);
  });

  // Setup search filter
  const searchInput = document.getElementById('sidebar-search-input');
  if (searchInput) {
    searchInput.addEventListener('input', (e) => {
      const q = e.target.value.toLowerCase();
      document.querySelectorAll('.sidebar-nav-item').forEach(item => {
        const text = item.textContent.toLowerCase();
        item.style.display = text.includes(q) ? 'block' : 'none';
      });
    });
  }
}

// ---------------------------------------------------------------------------
// Copy Install Command Helper
// ---------------------------------------------------------------------------

function setupQuickInstallCopy() {
  const btn = document.getElementById('copy-install-btn');
  if (btn) {
    btn.innerHTML = ICON_COPY_SVG;
    btn.addEventListener('click', () => {
      navigator.clipboard.writeText('cargo add discord-rust').then(() => {
        btn.innerHTML = ICON_CHECK_SVG;
        setTimeout(() => { btn.innerHTML = ICON_COPY_SVG; }, 2000);
      });
    });
  }

  const agentBtn = document.getElementById('copy-agent-btn');
  if (agentBtn) {
    agentBtn.addEventListener('click', () => {
      navigator.clipboard
        .writeText('git clone -b wiki https://github.com/notwinterdust/discord-rust.git')
        .then(() => {
          agentBtn.innerHTML = `${ICON_CHECK_SVG} <span style="color: #57F287;">Copied!</span>`;
          setTimeout(() => {
            agentBtn.innerHTML = `${ICON_COPY_SVG} <span>Copy</span>`;
          }, 2000);
        });
    });
  }
}

// ---------------------------------------------------------------------------
// App Initialization
// ---------------------------------------------------------------------------

window.addEventListener('DOMContentLoaded', () => {
  buildSidebar();
  setupQuickInstallCopy();

  const hash = window.location.hash.replace('#', '');
  if (hash) {
    // Check if hash corresponds to a document
    let isDoc = false;
    for (const cat of DOCS_CATALOG) {
      if (cat.items.some(i => i.id === hash)) {
        isDoc = true;
        break;
      }
    }
    if (isDoc) {
      showWiki(hash);
      return;
    }
  }

  showLanding();
});

// Handle browser Back/Forward buttons
window.addEventListener('hashchange', () => {
  const hash = window.location.hash.replace('#', '');
  if (hash && hash !== currentDocId) {
    showWiki(hash);
  } else if (!hash) {
    showLanding();
  }
});

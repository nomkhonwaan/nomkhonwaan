import { marked } from "https://esm.sh/marked@5.1.1";

export function renderMarkdown(md: string) {
  try {
    return marked.parse(md);
  } catch (e) {
    console.error("renderMarkdown error:", e);
    return md;
  }
}

export const CSS = `
  @import url('https://fonts.googleapis.com/css2?family=Maitree:wght@400;700&family=Prompt:wght@700&family=Source+Code+Pro&display=swap');

  .markdown-body {
    font-family: 'Maitree', sans-serif;
    line-height: 1.8;
    color: #333;
  }

  .markdown-body h1, .markdown-body h2, .markdown-body h3,
  .markdown-body h4, .markdown-body h5, .markdown-body h6 {
    font-family: 'Prompt', sans-serif;
    line-height: 1.3;
    color: #111;
    margin-top: 1.5em;
    margin-bottom: 0.5em;
  }

  .markdown-body h1 { font-size: 2rem; }
  .markdown-body h2 { font-size: 1.5rem; }
  .markdown-body h3 { font-size: 1.25rem; }

  .markdown-body p { margin-bottom: 1rem; }

  .markdown-body a { color: #2563eb; }
  .markdown-body a:hover { text-decoration: underline; }

  .markdown-body code {
    font-family: 'Source Code Pro', monospace;
    font-size: 0.9em;
    background: #f1f5f9;
    padding: 0.15em 0.4em;
    border-radius: 4px;
  }

  .markdown-body pre {
    background: #1e293b;
    color: #e2e8f0;
    padding: 1rem;
    border-radius: 8px;
    overflow-x: auto;
    margin-bottom: 1rem;
  }

  .markdown-body pre code {
    background: none;
    padding: 0;
    color: inherit;
  }

  .markdown-body ul, .markdown-body ol {
    padding-left: 1.5rem;
    margin-bottom: 1rem;
  }

  .markdown-body li { margin-bottom: 0.25rem; }

  .markdown-body blockquote {
    border-left: 4px solid #2563eb;
    padding-left: 1rem;
    margin: 1rem 0;
    color: #475569;
    font-style: italic;
  }

  .markdown-body table {
    width: 100%;
    border-collapse: collapse;
    margin-bottom: 1rem;
  }

  .markdown-body th, .markdown-body td {
    border: 1px solid #e2e8f0;
    padding: 0.5rem 0.75rem;
    text-align: left;
  }

  .markdown-body th {
    background: #f8fafc;
    font-weight: 700;
  }

  .markdown-body img { max-width: 100%; }

  .markdown-body hr {
    border: none;
    border-top: 1px solid #e2e8f0;
    margin: 2rem 0;
  }
`;

import { marked } from "https://esm.sh/marked@5.1.1";

const md = `# Hello\n\nThis is a test of marked via esm.sh`;

console.log(marked.parse(md));

import { assertEquals, assertStringIncludes } from "std/testing/asserts.ts";
import { renderMarkdown, CSS } from "./markdown.ts";

Deno.test("renderMarkdown", () => {
  const cases = [
    {
      name: "renders plain text as paragraph",
      input: "hello world",
      assert: (result: string) => assertEquals(result, "<p>hello world</p>\n"),
    },
    {
      name: "renders headings",
      input: "# Heading 1\n## Heading 2",
      assert: (result: string) => {
        assertStringIncludes(result, "<h1>Heading 1</h1>");
        assertStringIncludes(result, "<h2>Heading 2</h2>");
      },
    },
    {
      name: "renders code blocks",
      input: "```ts\nconst x = 1;\n```",
      assert: (result: string) => {
        assertStringIncludes(result, '<pre><code class="language-ts">');
        assertStringIncludes(result, "const x = 1;");
      },
    },
    {
      name: "renders inline code",
      input: "Use `deno task` to build",
      assert: (result: string) => {
        assertStringIncludes(result, "<code>deno task</code>");
      },
    },
    {
      name: "renders links",
      input: "[GitHub](https://github.com)",
      assert: (result: string) => {
        assertStringIncludes(result, '<a href="https://github.com">GitHub</a>');
      },
    },
    {
      name: "renders tables",
      input: "| A | B |\n|---|---|\n| 1 | 2 |",
      assert: (result: string) => {
        assertStringIncludes(result, "<table>");
        assertStringIncludes(result, "<th>A</th>");
        assertStringIncludes(result, "<td>1</td>");
      },
    },
    {
      name: "renders blockquotes",
      input: "> hello",
      assert: (result: string) => {
        assertStringIncludes(result, "<blockquote>");
        assertStringIncludes(result, "<p>hello</p>");
      },
    },
    {
      name: "returns a string for any input",
      input: "plain text",
      assert: (result: string) => assertEquals(typeof result, "string"),
    },
    {
      name: "suppresses deprecated mangle behavior (email not mangled)",
      input: "Contact me@example.com",
      assert: (result: string) => {
        assertStringIncludes(result, "me@example.com");
      },
    },
    {
      name: "suppresses deprecated headerIds behavior (no id attribute)",
      input: "# Title",
      assert: (result: string) => assertEquals(result, "<h1>Title</h1>\n"),
    },
  ];

  for (const { name, input, assert } of cases) {
    const result = renderMarkdown(input);
    assert(result);
  }
});

Deno.test("CSS", () => {
  assertStringIncludes(CSS, ".markdown-body");
  assertStringIncludes(CSS, "font-family");
  assertStringIncludes(CSS, "pre");
  assertStringIncludes(CSS, "blockquote");
  assertEquals(typeof CSS, "string");
  assertEquals(CSS.length > 500, true);
});
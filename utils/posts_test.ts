import { assertEquals } from "std/testing/asserts.ts";
import { extractFrontMatter, extractFirstParagraph } from "./posts.ts";

Deno.test("extractFrontMatter", () => {
  const cases = [
    {
      name: "parses front matter and body",
      input: `---
title: Hello
publish_date: 2024-01-01
tags: ['go', 'rust']
---

Body content here`,
      assert: (result: ReturnType<typeof extractFrontMatter>) => {
        assertEquals(result.attrs.title, "Hello");
        // YAML parser auto-converts date-like strings to Date objects
        assertEquals(
          (result.attrs.publish_date as Date).toISOString().split("T")[0],
          "2024-01-01",
        );
        assertEquals(result.attrs.tags, ["go", "rust"]);
        assertEquals(result.body, "Body content here");
      },
    },
    {
      name: "returns whole text as body when no front matter",
      input: "Just body, no front matter",
      assert: (result: ReturnType<typeof extractFrontMatter>) => {
        assertEquals(result.attrs, {});
        assertEquals(result.body, "Just body, no front matter");
      },
    },
    {
      name: "returns whole text as body when closing --- missing",
      input: "---\ntitle: Broken\nbody text",
      assert: (result: ReturnType<typeof extractFrontMatter>) => {
        assertEquals(result.attrs, {});
        assertEquals(result.body, "---\ntitle: Broken\nbody text");
      },
    },
  ];

  for (const { name, input, assert } of cases) {
    const result = extractFrontMatter(input);
    assert(result);
  }
});

Deno.test("extractFirstParagraph", () => {
  const cases = [
    {
      name: "returns first paragraph",
      input: "First paragraph text.\n\nSecond paragraph.",
      expected: "First paragraph text.",
    },
    {
      name: "stops before first heading",
      input: "Intro sentence.\n\n## Section\n\nMore text.",
      expected: "Intro sentence.",
    },
    {
      name: "strips markdown links",
      input: "Read [this article](https://example.com) for details.\n\nNext.",
      expected: "Read this article for details.",
    },
    {
      name: "strips inline code",
      input: "Run `deno task` to build.\n\nNext.",
      expected: "Run deno task to build.",
    },
    {
      name: "strips bold and italic markers",
      input: "This is **bold** and *italic* text.\n\nNext.",
      expected: "This is bold and italic text.",
    },
    {
      name: "returns empty string for empty body",
      input: "",
      expected: "",
    },
    {
      name: "returns empty string for whitespace-only body",
      input: "   \n\n  ",
      expected: "",
    },
    {
      name: "handles single paragraph without trailing newline",
      input: "Just one line.",
      expected: "Just one line.",
    },
  ];

  for (const { name, input, expected } of cases) {
    assertEquals(extractFirstParagraph(input), expected, `case: ${name}`);
  }
});
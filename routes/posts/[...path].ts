import { Handlers } from "$fresh/server.ts";
import { extname, join } from "std/path/mod.ts";

const IMAGE_EXTENSIONS = new Set([".png", ".jpg", ".jpeg", ".gif", ".webp", ".svg"]);

export const handler: Handlers = {
  async GET(_req, ctx) {
    const filePath = join("posts", ctx.params.path);
    const ext = extname(filePath).toLowerCase();
    if (!IMAGE_EXTENSIONS.has(ext)) {
      return new Response("Not Found", { status: 404 });
    }
    try {
      const data = await Deno.readFile(filePath);
      const mimeTypes: Record<string, string> = {
        ".png": "image/png",
        ".jpg": "image/jpeg",
        ".jpeg": "image/jpeg",
        ".gif": "image/gif",
        ".webp": "image/webp",
        ".svg": "image/svg+xml",
      };
      return new Response(data, {
        headers: { "Content-Type": mimeTypes[ext] ?? "application/octet-stream" },
      });
    } catch {
      return new Response("Not Found", { status: 404 });
    }
  },
};
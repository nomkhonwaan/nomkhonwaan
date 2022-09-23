import blog, { ga } from "https://deno.land/x/blog@0.5.0/blog.tsx";

blog({
  title: "Nomkhonwaan",
  description: "Trust me I'm Petdo",
  avatar: "avatar.png",
  avatarClass: "rounded-full",
  author: "Natcha Luangaroonchai",
  links: [
    { title: "Email", url: "mailto:me@nomkhonwaan.com" },
    { title: "GitHub", url: "https://github.com/nomkhonwaan" },
    { title: "LinkedIn", url: "https://linkedin.com/in/nomkhonwaan" },
  ],
  style:
    `
      @import url('https://fonts.googleapis.com/css2?family=Maitree:wght@400;700&family=Prompt:wght@700&family=Source+Code+Pro&display=swap');

      h1, h2, h3, h4, h5, h6 {
        font-family: 'Prompt', sans-serif;
      }
      
      p {
        font-family: 'Maitree', sans-serif;
      }

      code {
        font-family: 'Source Code Pro', monospace;
      }

      .text-2xl + .flex > .text-bluegray-500 {
        font-style: italic;
        font-family: 'Maitree', sans-serif;
        font-weight: normal;
      }
    `,
  middlewares: [
    ga("UA-33411047-1"),
  ],
  lang: "en",
  dateStyle: "long",
  theme: "auto",
});
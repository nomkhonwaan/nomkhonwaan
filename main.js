import blog, { ga, redirects } from "https://deno.land/x/blog@0.3.3/blog.tsx";

blog({
  author: "Natcha Luangaroonchai",
  title: "Nomkhonwaan",
  description: "Trust me I'm Petdo",
  avatar: "avatar.png",
  avatarClass: "rounded-full",
  links: [
    { title: "Email", url: "mailto:me@nomkhonwaan.com" },
    { title: "GitHub", url: "https://github.com/nomkhonwaan" },
    { title: "LinkedIn", url: "https://linkedin.com/in/nomkhonwaan" },
  ],
  background: "#f9f9f9",
  middlewares: [
    ga("UA-33411047-1"),
    // redirects({
    //  "/hello_world.html": "/hello_world",
    // }),
  ]
});

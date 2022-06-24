import blog, { ga, redirects } from "https://deno.land/x/blog@0.3.3/blog.tsx";

blog({
  title: "Nomkhonwaan",
  description: "Trust me I'm Petdo",
  cover: "https://deno-avatar.deno.dev/avatar/blog.svg",
  coverStyle: "avatar-rounded",
  author: "Natcha Luangaroonchai",
  background: "#f9f9f9",

  middlewares: [
    
    // If you want to set up Google Analytics, paste your GA key here.
    ga("UA-33411047-1"),

    // If you want to provide some redirections, you can specify them here,
    // pathname specified in a key will redirect to pathname in the value.
    // redirects({
    //  "/hello_world.html": "/hello_world",
    // }),

  ]
});

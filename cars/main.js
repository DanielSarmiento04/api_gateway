Bun.serve({
  fetch(req) {
    const url = new URL(req.url);
    if (url.pathname === "/cars/home") return new Response("Home page! cars");
    if (url.pathname === "/cars/home") return new Response("Cars Blog! ");
    return new Response("404!");
  },
  port:8001,
},);

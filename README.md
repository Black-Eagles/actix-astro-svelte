# actix-astro-svelte
Actix-Web x Astro template with Svelte

After cloning the repo, make sure to run `npm i ` inside `www` and enable the recommended VSCode extensions, then you're ready to go!  
The template only features the essentials to run the server, obviously developing but also something like reading from a config before starting the server is up to you.

## Astro Stuff
Inside of your Astro project, you'll see the following folders and files:

```
/
├── public/
│   └── favicon.ico
├── src/
│   ├── components/
│   │   └── Layout.astro
│   └── pages/
│       └── index.astro
└── package.json
```

Astro looks for `.astro` or `.md` files in the `src/pages/` directory. Each page is exposed as a route based on its file name.

There's nothing special about `src/components/`, but that's where we like to put any Astro/React/Vue/Svelte/Preact components or layouts.

Any static assets, like images, can be placed in the `public/` directory.
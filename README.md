
1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install the tailwind css cli: https://tailwindcss.com/docs/installation
3. Run the following command in the root of the project to start the tailwind CSS compiler:

```bash
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
```

Launch the Dioxus Fullstack app:

```bash
dx serve --platform fullstack
```

### To Do

Discover and resolve styling issues with the button. The code specifies a white border, but none is showing.

Strange, but the above issue appears resolved without code changes.

I need to figure out the next few controls, and then decide how to incorporate Rustcn into projects.

Textbox will be the next one.
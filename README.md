<div align="center">
    <picture>
        <source srcset="www/img/player.gif" />
        <img src="www/img/player.webp" />
    </picture>
</div>

# Website for Vallentin's Untitled Infinite Survival Game

Visit [survival.vallentin.dev](https://survival.vallentin.dev) for the online version.

Testing the website locally, can be done by first executing `npm run build` followed by `npm run localhost`.

|                     |                                                                 |
| ------------------- | --------------------------------------------------------------- |
| `npm run bake`      | Executes `cargo run` which renders all templates and blog posts |
| `npm run css`       | Compiles the SCSS into CSS                                      |
| `npm run html`      | Minifies the HTML                                               |
| `npm run img`       | Copies the images to the static dir                             |
| `npm run vid`       | Copies the videos to the static dir                             |
| `npm run build`     | _Runs all the above_                                            |
| `npm run localhost` | Runs a local dev server, hosting the static dir                 |

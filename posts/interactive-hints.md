---
title: Interactive hints
published: 2023-11-25
links:
- https://discord.gg/Ev8me66KCq
---

Today is a short one, I want to talk a bit about interactivity and adding highlights to visualize that entities are interactive.

I tested out multiple ways from:

- Re-drawing the same object twice with additive blending
- Drawing a translucent white silhouette of the object on top of it
- Rendering an outline on the entity

![](/img/2023-11-25/20231121_190644.png)

The different visualizations had different downsides.

**Translucent White Silhouette:**

Drawing a translucent white silhouette was definitely the easiest, as it essentially only required tinting all the pixels of the sprite to white.

There's also a lazier solution, which is to use a [stencil buffer], following by drawing a white quad covering the sprite. However, using the stencil buffer "just" for that seems a bit overkill. Additionally, it also requires splitting entity rendering into additional separate draw calls.

[stencil buffer]: https://en.wikipedia.org/wiki/Stencil_buffer

**Additive Blending:**

The downside to [additive blending], besides also having to split entity rendering into additional separate draw calls. Is that color changes can be quite extreme. Darker colors visually change less, while inversely the ligher colors change a lot.

Given that the game uses a lot of dark and saturated colors, then using additive blending might end up being hard to see.

[additive blending]: https://en.wikipedia.org/wiki/Blend_modes

**Outline:**

Using an outline _essentially_ doesn't require any (major) changes to the rendering. In short, it is technically just another sprite rendered on top of the entity.

However, a downside is that it requires drawing outline versions of all interactive entities.

That being said, for entities which have a clear (literal) outline, then the outline versions can somewhat be algorithmically generated. By literally tracing an outline **around** an entity.

-----

Ultimately, I received the most positive feedback for outlines, which I also preferred the most myself. I find outlines are the most readable out of those I tested. Adding an outline made it clear _which_ part is interactive, instead of just highlighting the whole object. Additionally, highlighting the whole object felt quite "extreme" and in-your-face.

-----

Thanks for reading! Feel free to leave a comment through any of the following links:

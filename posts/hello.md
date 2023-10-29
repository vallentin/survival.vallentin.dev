---
title: Hello DevLog
published: 2023-10-28
links:
- https://www.reddit.com/r/rust_gamedev/comments/17ik0in/two_months_later_and_ive_created_a_devlog_for_my/
- https://twitter.com/VallentinDev/status/1718332444628197492
- https://discord.gg/Ev8me66KCq
social-image: /img/2023-10-28-world.png
---

Hello, happy almost Halloween, and welcome to the devlog!

![](/img/2023-10-28-world.png)

Alright, so what's all this about? Well, let's rewind a bit first...

Around [3 months ago](https://twitter.com/VallentinDev/status/1686788931612798992) I randomly started working on an idea I had. The idea wasn't even for a game, but a rendering technique. However, after working on the project for a few days, then I blinked twice and suddenly a few weeks had passed. All of a sudden I had added...

- a player,
- enemies,
- a flamethrower,
- an infinite procedural world,
- flowers,
- trees,
- water,
- roads,
- buildings (which you can enter),
- vehicles,
- and more.

![](/vid/2023-09-16-200702.mp4)

I even had a drivable car!

![](/vid/2023-09-12-044534.mp4)

Throughout the first 2 months, I realized that I wanted to turn this into a game. I mean, I basically was already slowly doing that in the first place anyways.

-----

So the question is, what kind of game? It's a tale as old as time. _"The game I want to play doesn't exist."_ _(or maybe it does and I just never encountered it.)_

There's a handful of base aspects I love in games, which this game includes:

- **Survival** - Give me an axe, let me chop some trees, and let me build my own camp.
- **Hundreds of Enemies** - If it's a survival game, then give me something to survive against. I want hundreds of enemies at the same time, not just at most 10 or 20. I want to accidentally trigger a car alarm and actually get scared, for the horde of enemies down the street.
- **Procedural World** - Give me an infinite world to explore, which actually feels big.
- **Large Scale Cities** - Give me huge random cities to explore, let me find resources, weapons, maybe even a place to wait out the night.
- **No Barriers** - If I see a building, I want to be able to get into it. Either the door is open or it requires a crowbar. But I'm getting in!
- **Consistent Setting** - If you're telling me it's a post-apocalyptic game, then make it look and feel that way. Rusted cars, buildings falling apart, vines and vegetation everywhere.

I've tried a lot of games, and so far none seem to satisfy my itch for all those aspects in a single game.

Conceptually, I've already implemented and tested the main aspects. The game is implemented in Rust, with a custom engine that can easily render [400 million tiles](https://twitter.com/VallentinDev/status/1695410986944016883), as well as simulating [10,000 enemies](https://twitter.com/VallentinDev/status/1697266492700770318) interacting with each other.

![](/vid/2023-08-23-161129.mp4)

-----

So what's the game setting? It's the end of the world, but it's not _Another Zombie Game&trade;_. What is it then? Well, for now, that will remain a surprise for later.

However, the goal is not to remove people from a city, and then call it a post-apocalyptic game. The goal is to make it look and feel like an abandoned world as well.

Now, yes, I am fully aware that I've been using a green zombie-like enemy until now. Which I also use as the background for this website. However, that is just for experimenting, it was simply the player sprite turned green.

_I actually already have made some of the new enemies, but again, let's leave them for a future devlog._

-----

So what now? Right now, I'm in the process of recreating all graphics for the game. The graphics I originally made were some I made quickly. Additionally, I'm a software engineer not an artist, so even then making pixel art isn't really something I've attempted for an extended period of time, until now. So there's a lot of learning involved as well, which I've been doing exclusively for the past month.

Here's what some of the graphics in the game currently looks like:

_(as well as the first image of this post)_

![](/img/2023-10-27-world.png)

-----

This is the first devlog post in a series of posts, that I plan to release every other Saturday. Since I have worked on the game for now 3 months, then I felt it was time for a devlog. I plan to share information about the game and the process of making it.

This post was more of an introductory to the game itself. I'm still considering how technical future devlogs should be. Whether they should just cover the latest news and changes. Inversely some might be interested in actual development, behind the scenes, and code snippets. I will try out stuff based on feedback, or otherwise let the devlog evolve organically.

_Currently the game doesn't have a name, which is why it lives under my personal domain with a generic subdomain._

-----

I'm planning on making a demo, when I have something that's playable for more than just 10 minutes. Additionally, I'll be posting experimental releases for people to test on [Discord](https://discord.gg/Ev8me66KCq) when the time comes.

-----

Thanks for reading! Feel free to leave a comment through any of the following links:

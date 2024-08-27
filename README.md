# WebSocket Tower Defense Game 🏗️
## A super simple attempt at a multiplayer web game :) Filled to the brim with slightly poor decisions and a can-do spirit 🥲

This project exists as the biggest challenge in my journey to understand websockets and asynchronous Rust as a whole. I aimed to create a game similar to [The Battle Cats](https://battlecats.club/en/) minus any of the charm. 

As such, one area that could be significantly improved upon is the way the *actual* game is being handled. Specifically, most of the game logic occurs on the TypeScript frontend, only communicating with Rust to synchronize and update state. This of course is not good, and will possibly be revisited at a later stage when I'm satisfied with the parts of the project that I wanted to learn :)

![Sample game footage](./actionshot.png)

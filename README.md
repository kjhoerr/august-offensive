# August Offensive
[![Build Status](https://travis-ci.org/kjhoerr/august-offensive.svg?branch=master)](https://travis-ci.org/kjhoerr/august-offensive)

August Offensive is a game as a web service that enables players to achieve world domination in turn-based strategy. Players as Allegiances can generate armies, gain control of regions, and defeat their enemies. The Allegiance that controls all of the regions around the world claims victory of the August Offensive.

The main goal of August Offensive is to enable players to interact with the service only when they are available to do so. Effectively, players do not need to maintain connection to the service for the entirety of the game. To prevent games from deadlocking, a customary time limit is enforced (ex. 96 hours).

## Technical Objectives

The project is built using the [Rust programming language](https://www.rust-lang.org/en-US/), [actix-web](https://actix.rs/) as the web framework, and [Diesel](https://diesel.rs/) for interacting with a PostgreSQL database. Diesel and actix-web work on stable Rust, removing the constant need to support nightly features. In addition, Diesel provides a client that makes it easy to migrate between schemas as they evolve between releases. This helps the project maintain a great level of portability without putting too great pressure on the application code.

The front-end of this project will be written using [Elm](https://elm-lang.org/) (although this is subject to change). Development on the front-end portion will not begin until August Offensive's web API has reached relative stability. The target version number for stability is 1.0.0.

## Deployment

Visit the Wiki for guides detailing [how to deploy August Offensive](https://github.com/kjhoerr/august-offensive/wiki/Deployment) (e.g. standalone or using Docker).

## Contributing to the Project

While the project is still getting off the ground, there is not much back-end of which to build off. That doesn't mean you can't help brainstorm at least! There is much about the core functionality that is still up in the air at this point. Please have a look at [the Roadmap](https://github.com/kjhoerr/august-offensive/wiki/Roadmap) for a detailed layout of intended features and / or milestones.

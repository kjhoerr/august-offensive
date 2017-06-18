# August Offensive

August Offensive is a game as a web service that enables players to achieve world domination in turn-based strategy. Players as Allegiences can generate armies, gain control of regions, and defeat their enemies. The Allegiance that controls all of the regions around the world claims victory of the August Offensive.

The main goal of August Offensive is to enable players to interact with the service only when they're available to do so. Effectively, players do not need to maintain connection to the service to complete the game. To prevent games from deadlocking, a customary time limit is enforced (ex. 96 hours).

## Technical Objectives

The web service will provide a RESTful API that is written in PHP 7 and connects to a PostGreSQL database. This connection will be managed behind an abstraction layer using PDO. The service will handle users and their game sessions. 

The front-end of this project will be written using [Elm](http://elm-lang.org/). Work on the front-end portion will not begin until August Offensive's web API has reached relative stability. The version for target stability is alpha 1.0.0.

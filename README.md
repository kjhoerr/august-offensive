# August Offensive

August Offensive is a game as a web service that enables players to achieve world domination in turn-based strategy. Players as Allegiances can generate armies, gain control of regions, and defeat their enemies. The Allegiance that controls all of the regions around the world claims victory of the August Offensive.

The main goal of August Offensive is to enable players to interact with the service only when they are available to do so. Effectively, players do not need to maintain connection to the service for the entirety of the game. To prevent games from deadlocking, a customary time limit is enforced (ex. 96 hours).

## Getting Started

Getting the code running should be fairly simple. Given you have the properly installed packages (PHP 7.0, PostGreSQL 9.6) there should be few if any configuration changes to existing services. Make sure [your credentials are configured](https://github.com/kjhoerr/august-offensive/wiki/Credentials-for-PGSQL) and try visiting /api/callback at your server - you should get a response if it's set up correctly.

In order to run tests, install [PHPUnit](https://phpunit.de/) (for PHP 7.0). At the project root, run `phpunit --bootstrap autoload.php test`.

## Technical Objectives

The web service will provide a RESTful API that is written in PHP 7.0 and connects to a PostGreSQL database. This connection will be managed behind an abstraction layer using PDO. The service will handle users and their game sessions. In addition, all PHP code shall follow PSR-1, PSR-2, PSR-4, and PSR-5 coding standards. It is also recommended to try to write code as immutably as possible.

The front-end of this project will be written using [Elm](http://elm-lang.org/) (although this is subject to change). Work on the front-end portion will not begin until August Offensive's web API has reached relative stability. The target version number for stability is 1.0.0.

## Contributing to the Project

While the project is still getting off the ground, there is not much back-end of which to build off. That doesn't mean you can't help brainstorm at least! There is much about the core functionality that is still up in the air at this point. Please have a look at [the Roadmap](https://github.com/kjhoerr/august-offensive/wiki/Roadmap) for a detailed layout of intended features and / or milestones.

# Platformer

A generic platformer game to continue practice/learning game dev with Rust.


## TODO

### Gameplay
- [ ] Enemies need to walk towards the player
- [ ] Player needs full movement
- [ ] Lives?
- [ ] Do enemies spawn?
- [ ] Need to figure out how do detect what is colliding with an enemy/player/etc
- [ ] Health bars

### Refactor
- [ ] Abstract the physics/draw objects so that the math does not need to be done for each drawn object. See [Player render method](src/game/player.rs)
- [ ] Implement entity component system

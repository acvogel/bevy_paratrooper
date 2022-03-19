TODO
X planes texture
X bullet collisions with planes and associated despawns
X explosion animation
X paratrooper drops
X gun cooldown or bullet limit
x score
* sound effects
* paratrooper shots?
X background image
* webasm build and deploy
* heron or bevy_rapier for physics

https://opengameart.org/content/war-on-water-gfx


physics plugin: when spawning bundles you add the collision and body handles
then you watch for like collision event and the associated entities,
then fire the BulletCollisionEvent and whatnot.

ground is a big fixed rectangle

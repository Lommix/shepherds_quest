# A strange shepherd's Quest

![screenshot](docs/screen.jpeg)

My bevy jam #4 entry.

# Used third party assets

-   https://opengameart.org/content/space-backgrounds-3
-   https://opengameart.org/content/low-poly-3d-flying-saucer-model
-   https://opengameart.org/content/lowpoly-animated-farm-animal-pack
-   https://opengameart.org/content/textured-low-poly-pine
-   https://opengameart.org/content/forest

Everything else is made by myself (Yes even the bark sound). The Sheep sounds were made by my wife :)

# Make your own levels

In the game menu you can load a level from file. The file format is ron. You can tweak most values and create absurd levels.
Here is an example:

```ron
(
    name: "Pug in training",
    intro: "Shepherd Bob: This is your last test, Henk. I gave you super treats. You are faster now! Get 50% of the sheeps to the ship!",
    win: "Good boy! Let's rob the next farm!",
    loose: "Bad boy! You messed up! Do a better job or I'll have to find another dog!",
    sheeps_per_spawn: 80,
    win_percent: 50,
    animal_behavior: Some(
        (
            alignment: 0.8,
            cohesion: 1.0,
            separation: 0.1,
            sheep_speed: 25.0,
            vision: 20.0,
            fear: 1.0,
            motivation: 0.1,
            dog_speed: 70.0,
            llama_stomp_rate: 6.,
            llama_stomp_range: 64.,
            llama_stomp_force: 600.,
        ),
    ),
    layout:
"
###################################################
#-----------------------T#T-----------------------#
#-----------------------T#T-----------------------#
#-----------------------T#T-----------------------#
#---S-------------------T#T-------------------S---#
#-----------------------T#T-----------------------#
#-----------------------T#T-----------------------#
#-----------------------T#T-----------------------#
#-----------------------T#T-----------------------#
#-----------------------T#T-----------------------#
#-----------------------T#T-----------------------#
#-------------------L----------L------------------#
#-------------------------------------------------#
#------------------------D------------------------#
#TTTTTTTTTTTTTTTTTT#----GGG----#TTTTTTTTTTTTTTTTTT#
####################----GGG----####################
#TTTTTTTTTTTTTTTTTT#----GGG----#TTTTTTTTTTTTTTTTTT#
#-------------------------------------------------#
#-------------------------------------------------#
#-------------------L---###----L------------------#
#-----------------------T#T-----------------------#
#-----------------------T#T-----------------------#
#-----------------------T#T-----------------------#
#-----------------------T#T-----------------------#
#-----------------------T#T-----------------------#
#-----------------------T#T-----------------------#
#---S-------------------T#T-------------------S---#
#-----------------------T#T-----------------------#
#-----------------------T#T-----------------------#
#-----------------------T#T-----------------------#
###################################################
",
)
```

-   `T` = Trap Tile
-   `S` = Sheep Spawn Tile
-   `L` = Llama Spawn Tile
-   `D` = Dog Spawn Tile
-   `G` = Goal Tile
-   `#` = Wall Tile
-   `-` = Floor Tile

At the end I just wanted to finish. The code got a bit messy, but I'm happy with the result. I hope you enjoy it.

*"Evolution is war. The prey wishes to escape, the predator wishes to catch - one will inevitably suffer, from hunger or fang."*

*"When you step in a flowery meadow and frolick in joy through the bushes and butterflies... You are blind to the agonizing battlefield raging all around you. Each blade of grass strives to choke its neighbour, each insect would love to devour you whole had you been a hundredth of your size."*

*"Fu! And you say nature and its diversity is beautiful. With affection, you lick the hand which chokes you by the throat. We would laugh, had we not been feeling infinite pity for you in this moment."*

# The Games Foxes Play
*([complete source code](https://github.com/Oneirical/The-Games-Foxes-Play) - [rust source code](https://github.com/Oneirical/rust_tgfp) | [view all previous posts](https://github.com/Oneirical/The-Games-Foxes-Play/tree/main/design/Development%20Logs))*

The advancements of this week are highly experimental and possibly insane. Given repeated troubles related to the player constantly breaking the smooth flow and design of the game, I am attempting the obvious: removing the player.

A grid of tiles contains a certain number of creatures. Each one operates as follows:

* It receives some Senses, defined by the player, like the distance between itself and the yellow X Beacon, or the angle between itself and the Beacon.
* The Senses pass through a simple neural network (only 1 hidden layer as to not turn my computer into a miniaturized sun)
* The neural network chooses one among a list of Axioms, also defined by the player, and casts it. This could be moving one tile, or doing some cool ability.

These creatures are then tortured over and over again in my test chamber. Every 100 turns, each one is judged according to an efficiency score. The player defines the criteria, like "get as close to the Beacon as possible".

Through some genetic algorithm technomagic, all the creatures are returned to their start position, and their neural networks are tweaked to look more like those of the best performers, with some random mutations thrown in there for science.

Eventually, the creatures "learn" how to complete the objective defined by the player! I believe the buzzword for this technique is "neuroevolution".

In order for things to remain smooth, the game window actually makes you watch a re-enactment of a simulation that already happened while the game is simulating hundreds of 100-turn sequences in the background. This means that even if the CPU is choking through the neural networks, there is no visible FPS drop, only a mild slowdown of the training speed. Thank you, Rust, for this so-called "fearless concurrency".

While I find this educational and entertaining, the complexity of this design is ramping up very fast. I am currently trying to set up a paintball minigame of sorts where two factions of creatures compete to paint the map in their colour. I may fail and give up, or I may succeed and be inspired. We will see.

Videos:

[An oddly satisfying anthill.](https://yewtu.be/embed/0k9tPXNCEv4?)

[A tango line caused by a poorly thought out efficiency score formula.](https://yewtu.be/embed/hB14TX1fW4w) (walls disabled)

[A well-trained population.](https://yewtu.be/embed/DCK_dFT3g3Q) (walls disabled)
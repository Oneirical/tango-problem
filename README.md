# The Tango Problem

*Personal experiments with genetic algorithms and neuroevolution, in Rust, with the Bevy engine.*

A number of "Psychics" are placed inside a 45x45 play area, and submitted to one of the following two challenges:

1. Get as close as possible to a "Beacon of Light" randomly placed in the space before the 100 turn limit is elapsed. Walls are disabled in this experiment.
2. Paint as many walls in pink as possible within the space before the 100 turn limit is elapsed. Walls are enabled in this experiment.

The Psychics do not "know" these objectives. All they are given is a series of inputs representing their senses, such as:

* The angle between themselves and the Beacon of Light
* The North-West-East-South direction in which the Beacon of Light is located

* Which adjacent tiles are solid
* Which tiles in range 2 of themselves are solid
* Which adjacent tiles are painted walls
* Which adjacent tiles are unpainted walls

A neural network receives these inputs as a vector of floating-point numbers (for example, "the Beacon is North of you" could look like [1.0, 0.0, 0.0, 0.0], "the Beacon is East of you" would be [0.0, 0.0, 1.0, 0.0]).

Then, snazzy things happen inside one or more hidden layer(s), and a vector of floating-point numbers comes out (such as [0.32, 0.55, 0.38, 0.85, 0.11]). These correspond to actions, such as [Move North, Move West, Move East, Move South, Paint Nearby Walls]. The highest number is the chosen action.

After each 100-turn sequence, each Psychic has their fitness calculated - how well they performed. Initially, this was only based on the objective, such as "the number of tiles separating a Psychic from the Beacon" or "the number of tiles painted." The best Psychics are selected through "roulette pick" (everyone has a chance to be chosen, that chance increases with the fitness score), some mutations are applied to the neural network's weights, and a new generation is born, which will hopefully fare better than the last.

But, this proved insufficient. I first observed the "Tango Problem" in the Beacon experiment - large numbers of Psychics would simply move down until they were on the same Y coordinate as the Beacon, [then simply move up and down over and over](https://yewtu.be/embed/hB14TX1fW4w).

Why was this happening? After some struggling, I figured it out.

A lucky Psychic would occasionally spawn on the same X level as the Beacon. Then, they "figured out" that they could just move down until the Beacon was reached, and start moving up and down over and over again. This was gloriously rewarded by the system - after all, they perfectly completed the task of reaching the Beacon and staying on it!

Other Psychics would start copying this behaviour - except these are not actually on the same X level of the Beacon, and as such would fail the test, forming a line passing through the Beacon across the entire level. Naturally, there would always be at least one lucky Psychic succeeding through this strategy and making this behaviour repeat itself.

My answer was to add, in the fitness function, a HUGE reward for Psychics who would dare to use more than just 2 different actions. This immediately solved the issue, as the "bob up and down and hope to be lucky" strategy could no longer result in the maximum possible fitness a Psychic could reach. [Here is the result!](https://yewtu.be/embed/DCK_dFT3g3Q)

The painting experiment followed. It, too, experienced struggled related to the Tango Problem (which, I believe, is known as "getting stuck in a local maximum" in the literature): some Psychics would occasionally be very lucky and spawn next to many walls, which means they can just stay there, paint their surroundings, never move and still score 3-7 points without effort. Psychics that actually see an unpainted wall, move towards it and paint it are not rewarded as well.

Once again, I added extra objectives:

* Do not stay in the same place for too long. (this resulted in Psychics bobbing up and down just like in the original Tango Problem and not doing anything else)
* Receive a penalty for re-painting already painted walls (this resulted in Psychics being terrified of ever painting anything)
* Receive a bonus for moving far away from the spawn location (this result in Psychics all rushing the bottom of the screen and painting it, leaving the rest of the level unpainted)

[Some of these behaviours can be observed here.]()

Significant tweaking may eventually produce even more performant results, but for now, a balance of all these objectives does succeed in achieving a ["trickle down" strategy where all Psychics move towards the bottom of the screen, painting walls as they fall down.]() (Creatures being in walls at the bottom is purely a visual glitch - I am not quite sure how to handle getting just the right amount of Bevy entities in each simulation without causing major performance issues).

***

Overall, I am happy with how much I have learned from this mini-project. Studying genetic algorithms initially made it seem like they could handle anything with brute force and sufficient time, but I see now that the objectives, "sense" inputs and possible action outputs change everything. It is very easy for these algorithms to find something judged to be "good enough" and stick to it without much of an attempt to improve or innovate. I am aware a good algorithm should balance between exploitation and exploration, but the amount of tweaking required to reach this fine balance is truly a work of absolute precision and patience.

Rust is an interesting piece of technology. Its "fearless concurrency" was very useful in making this program able to BOTH display the results of a completed simulation and train more simulations in the background **at the same time**. As someone coming from dynamic, untyped languages like JavaScript and Python, it is truly astounding how many errors are prevented just through rust-analyzer's watchful eye. I am however disappointed by the constant long recompilation times at each added library, the rather poor step-by-step debugging support, and how rust-analyzer loves turning my CPU into a localized micro-sun.
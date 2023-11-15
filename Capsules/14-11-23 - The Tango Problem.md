While experimenting with the following challenge:

- All Psychics are randomly spread across the map.
- There are no walls or collisions.
- They must reach the Beacon located at a random spot.

I was growing increasingly frustrated with the constant urge of the Psychics to line up on the same Y level as the Beacon and just endlessly bob up and down without actually trying to reach the Beacon. Why did they not even attempt to use other actions than up and down?

I got so puzzled that I wondered for a moment if the fundamental idea of this game was flawed. That my genetic algorithm simply cannot solve these puzzles in such a dumbed down way. I looked at them dancing in tango across the screen, a rose between their maws, mocking me.

But, giving it some thought, I had a hypothesis. They didn't do this behaviour before when the Psychics started clumped up. Spreading them apart (and also increasing their numbers) made it so a lucky Psychic would occasionally spawn on the same X level as the Beacon. Then, they "figured out" that they could just move down until the Beacon was reached, and start moving up and down over and over again. This was gloriously rewarded by the system - after all, they perfectly completed the task of reaching the Beacon and staying on it!

Other Psychics would start copying this behaviour - except these are not actually on the same X level of the Beacon, and as such would fail the test, forming a line passing through the Beacon across the entire level. Naturally, there would always be at least one lucky Psychic succeeding through this strategy and making this behaviour repeat itself.

My answer was to add, in the fitness function, a HUGE reward for Psychics who would dare to use more than just 2 different actions. This immediately solved the issue, as the "bob up and down and hope to be lucky" strategy could no longer result in the maximum possible fitness a Psychic could reach.

This was on the same day where I struggled to optimize map generation to solve massive lag only to finally realize the cause was in map.tiles.clone() happening every turn to check for collisions that could simply be replaced by &map.tiles.

The point is, if something seems to be without a solution, stop. Think. Put yourself in the place of the Psychics. Think about what they sense, and what hasty conclusions one might be influenced into making.
/**
The jungle must be too overgrown and difficult to navigate in vehicles or access from the air; the Elves' expedition traditionally goes on foot. As your boats approach land, the Elves begin taking inventory of their supplies. One important consideration is food - in particular, the number of Calories each Elf is carrying (your puzzle input).
The Elves take turns writing down the number of Calories contained by the various meals, snacks, rations, etc. that they've brought with them, one item per line. Each Elf separates their own inventory from the previous Elf's inventory (if any) by a blank line.
For example, suppose the Elves finish writing their items' Calories and end up with the following list:
See input.txt
*/

import { elves, totalCalories } from "./input.ts";

const mostCaloriesSorted = elves.sort(
  (a, b) => totalCalories(b) - totalCalories(a)
);

const topThree = mostCaloriesSorted
  .slice(0, 3)
  .map((elf) => ({ elf, total: totalCalories(elf) }));

const totalOfTopThree = topThree.reduce(
  (totalCalories, nextElf) => totalCalories + nextElf.total,
  0
);

console.log({
  totalOfTopThree,
  mostCaloriesSorted: topThree,
});

export const answer = totalOfTopThree;

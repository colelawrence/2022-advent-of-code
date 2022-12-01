/**
The jungle must be too overgrown and difficult to navigate in vehicles or access from the air; the Elves' expedition traditionally goes on foot. As your boats approach land, the Elves begin taking inventory of their supplies. One important consideration is food - in particular, the number of Calories each Elf is carrying (your puzzle input).
The Elves take turns writing down the number of Calories contained by the various meals, snacks, rations, etc. that they've brought with them, one item per line. Each Elf separates their own inventory from the previous Elf's inventory (if any) by a blank line.
For example, suppose the Elves finish writing their items' Calories and end up with the following list:
See input.txt
*/
const encoder = new TextDecoder();
const input = encoder.decode(await Deno.readFile("./input.txt"));
console.log(input);

interface ElfFoodItem {
  index: number;
  calories: number;
}

interface Elf {
  index: number;
  foodItems: ElfFoodItem[];
}

function totalCalories(elf: Elf) {
  return elf.foodItems.reduce((total, next) => total + next.calories, 0);
}

const elves = input.split(/\n\s*\n/g).map((desc, idx): Elf => {
  return {
    foodItems: desc
      .split(/\s+/g)
      .map(
        (cals, idx): ElfFoodItem => ({ index: idx, calories: Number(cals) })
      ),
    index: idx,
  };
});

// console.log(elves);

const hasMostCalories = elves.reduce((mostCalories, currentElf) =>
  totalCalories(currentElf) > totalCalories(mostCalories)
    ? currentElf
    : mostCalories
);

console.log({
  hasMostCalories,
  calories: totalCalories(hasMostCalories),
});

let fs = require("fs");
const input = fs.readFileSync("./day4", "utf-8").toString().trim();

let assignmentPairs = input.split("\n");

assignmentPairs = assignmentPairs.map(assignmentPair =>
  assignmentPair.split(",")
);

let fullyContainsAmount = 0;
let v = 0;

for (const assignmentPair of assignmentPairs) {
  let assignment1start = +assignmentPair[0].split("-")[0];
  let assignment1end = +assignmentPair[0].split("-")[1];
    v++;
    console.log(v)

  let assignment2start = +assignmentPair[1].split("-")[0];
  let assignment2end = +assignmentPair[1].split("-")[1];

  // Check if first fully contains the second

  if (
    assignment1start <= assignment2start &&
    assignment1end >= assignment2end
  ) {
    fullyContainsAmount += 1;
    console.log(
      `assignment 1 (${assignment1start} - ${assignment1end}) fully contains assignment 2 (${assignment2start}- ${assignment2end})`
    );
  }

  // Check if second fully contains the first
  else if (
    assignment2start <= assignment1start &&
    assignment2end >= assignment1end
  ) {
    fullyContainsAmount += 1;
    console.log(
      `assignment 2 (${assignment2start} - ${assignment2end}) fully contains assignment 1 (${assignment1start}- ${assignment1end})`
    );
  }
}

console.log(fullyContainsAmount);

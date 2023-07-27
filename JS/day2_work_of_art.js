const fs = require('fs');

let data = fs.readFileSync("../inputs/day2_input", "utf8").trim().split("\n");

let scorept1 = 0;
let scorept2 = 0;

const matchPoints = {
    "X": 1,
    "Y": 2,
    "Z": 3
}
const toWin = {
    "A": "Y",
    "B": "Z",
    "C": "X",
}
const toDraw = {
    "A": "X",
    "B": "Y",
    "C": "Z",
}
const toLose = {
    "A": "Z",
    "B": "X",
    "C": "Y",
}

data.forEach((line) => {
    let [opp, me] = line.split(" ");
    // part 1 shit
    if (toWin[opp] === me) {
        scorept1 = scorept1 + matchPoints[me] + 6;
    } else if (toDraw[opp] === me) {
        scorept1 = scorept1 + matchPoints[me] + 3;
    } else {
        scorept1 = scorept1 + matchPoints[me];
    }

    // part 2 shit
    switch (me) {
        case "X":
            scorept2 = scorept2 + matchPoints[toLose[opp]];
            break;
        case "Y":
            scorept2 = scorept2 + matchPoints[toDraw[opp]] + 3;
            break;
        default:
            scorept2 = scorept2 + matchPoints[toWin[opp]] + 6;
    }
});

console.log(`part 1:- ${scorept1}\npart 2:- ${scorept2}`);

const fs = require('fs');

let data = fs.readFileSync('./day4_input', 'utf8').trim().split('\n');

let countpt1 = 0;
let countpt2 = 0;
data.forEach((line) => {
    let sections = line.split(',');
    let firstSection = sections[0];
    let secondSection = sections[1];

    let [start1, end1] = firstSection.split('-').map(Number);
    let [start2, end2] = secondSection.split('-').map(Number);
    // console.log(`start1: ${start1}\nend1: ${end1}\nstart2: ${start2}\nend2: ${end2}\n`)

    // part 1 shit
    if ((start1 <= start2 && end1 >= end2) || (start2 <= start1 && end2 >= end1)) {
        countpt1++;
    }

    // part 2 shit
    // if (start1 <= start2 && start2 <= end1 && end1 <= end2) {
    //     countpt2++;
    // } else if (start2 <= start1 && start1 <= end2 && end2 <= end1) {
    //     countpt2++;
    // } else if (start1 <= start2 && end2 <= end1) {
    //     countpt2++;
    // } else if (start2 <= start1 && end1 <= end2) {
    //     countpt2++;
    // }


    // Shitty line logic but it works consider we have 2 lines (2, 6) and (4, 8)
    //       (2)-------------(6)
    //              (4)------------(8)
    // start1 = 2 and start2 = 4 ============ max = 4
    // end1 = 6 and start2 = 8   ============ min = 6
    // so by this if the max <= min then we can say that some portiion of line is overlaping(we don't have to give what is overlaping)

    let maxStart = Math.max(start1, start2);
    let minEnd = Math.min(end1, end2);
    if (maxStart <= minEnd) {
        countpt2++;
    }

    // console.log(`firstSection: ${firstSection}\nsecondSection: ${secondSection}`);
});

console.log(`part 1:- ${countpt1}\npart 2:- ${countpt2}`);

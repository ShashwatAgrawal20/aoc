const fs = require('fs');

fs.readFile('./day1_input', 'utf8', (err, data) => {
    if (err) {
        console.error(err);
        return;
    }
    const sep = data.split('\n');
    let sum = 0;
    let max = 0;
    let sumList = [];
    for (let i = 0; i < sep.length; ++i) {
        if (sep[i] === '') {
            sumList.push(sum);
            if (sum > max) {
                max = sum;
            }
            sum = 0;
        }
        else {
            sum += parseInt(sep[i]);
        }
    }
    const sorted = sumList.sort((a, b) => b - a);
    const topThree = sorted.slice(0, 3);
    const sumOfTopThree = topThree.reduce((acc, value) => acc + value, 0);

    console.log(`part 1:- ${max}\npart 2:- ${sumOfTopThree}`);
});

const fs = require('fs');

let data = fs.readFileSync('./day3_input', 'utf8').trim().split('\n');

const characterPriority = {
    'a': 1, 'b': 2, 'c': 3, 'd': 4, 'e': 5, 'f': 6, 'g': 7, 'h': 8, 'i': 9, 'j': 10, 'k': 11, 'l': 12, 'm': 13, 'n': 14, 'o': 15, 'p': 16, 'q': 17, 'r': 18, 's': 19, 't': 20, 'u': 21, 'v': 22, 'w': 23, 'x': 24, 'y': 25, 'z': 26,
    'A': 27, 'B': 28, 'C': 29, 'D': 30, 'E': 31, 'F': 32, 'G': 33, 'H': 34, 'I': 35, 'J': 36, 'K': 37, 'L': 38, 'M': 39, 'N': 40, 'O': 41, 'P': 42, 'Q': 43, 'R': 44, 'S': 45, 'T': 46, 'U': 47, 'V': 48, 'W': 49, 'X': 50, 'Y': 51, 'Z': 52
};

let sumpt1 = 0;
let sumpt2 = 0;

// part 1 shit
data.forEach((line) => {
    const firstCompartment = line.slice(0, line.length / 2);
    const secondompartment = line.slice(line.length / 2, line.length);
    const commonCharacter = [...firstCompartment].find(char => secondompartment.includes(char));

    if (commonCharacter === commonCharacter.toLowerCase()) {
        sumpt1 += characterPriority[commonCharacter];
    } else {
        sumpt1 += characterPriority[commonCharacter];
    }
});

// part 2
for (let i = 0; i < data.length; i += 3) {
    const chunk = data.slice(i, i + 3);
    let common = '';
    for (let char of chunk[0]) {
        if (chunk[1].includes(char) && chunk[2].includes(char)) {
            common = char;
            break;
        }
    }

    if (common === common.toLowerCase()) {
        sumpt2 += characterPriority[common];
    } else {
        sumpt2 += characterPriority[common];
    }
}

console.log(`part 1:- ${sumpt1}\npart 2:- ${sumpt2}`);

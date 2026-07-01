const san = [2, 3, 4, 5, 4];

let wow = san.map((n) => n ** n).filter((e) => e !== e / 2);
console.log(wow);

export { wow };

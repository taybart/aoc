const fs = require("fs");
let dial = 50;
let password = 0;
const data = fs.readFileSync("input.txt", { encoding: "utf8" });
const dataStringsArr = data.split("\n");
for (const dataString of dataStringsArr) {
  const n = +dataString.substring(1);
  let rotation = n % 100;
  //add full rotations to the password (floor from dividing by 100 vodoo)
  password += (n / 100) | 0;
  if (dataString[0] === "L") {
    rotation = -rotation;
    const next = dial + rotation;
    // add 100 for a wrap around
    if (next < 0 && next + 100 !== 0 && dial !== 0) password++;
  } else {
    const next = dial + rotation;
    // substract 100 for a wrap around
    if (next > 99 && next - 100 !== 0 && dial !== 0) password++;
  }

  if (dial === 0) password++;
  dial = (dial + rotation + 100) % 100;
}
console.log(password);

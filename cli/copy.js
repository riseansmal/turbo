const { cpSync } = require("fs");
const [dest, ...files] = process.argv.slice(2).reverse();
files.forEach((src) => cpSync(src, dest));

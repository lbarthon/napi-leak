const { A } = require('./index.node');
const a = new A();

gc();

const arr = [];
for (let i = 0; i < 100_000; i++) {
    arr.push(a.newB());
}

gc();

// setTimeout(() => {
// console.log(scheme);
// }, 999999999);

while (true) {}

const { A } = require('./index.node');

gc();

{
    let a = A.new();
    for (let i = 0; i < 100_000; i++) {
        a.newB();
    }
}

gc();

// setTimeout(() => {
// console.log(scheme);
// }, 999999999);

while (true) {}

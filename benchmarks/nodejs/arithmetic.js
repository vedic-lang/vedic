var result = 0;
for (var i = 0; i < 10000000; i = i + 1) {
    result = result + 11;
    result = result * 10;
    result = result - (result / 100) * 99;
}

console.log(result);

class Tree {
    constructor(item, depth) {
        this.item = item;
        this.depth = depth;
        if (depth > 0) {
            var item1 = item + item;
            depth = depth - 1;
            this.left = new Tree(item1 - 1, depth);
            this.right = new Tree(item1, depth);
        } else {
            this.left = null;
            this.right = null;
        }
    }

    check() {
        if (this.left == null) {
            return this.item;
        }

        return this.item + this.left.check() - this.right.check();
    }
}

var minDepth = 4;
var maxDepth = 14;
var stretchDepth = maxDepth + 1;

console.log("stretch tree of depth:");
console.log(stretchDepth);
console.log("check:");
console.log(new Tree(0, stretchDepth).check());

var longLivedTree = new Tree(0, maxDepth);

// iterations = 1 ** maxDepth
var iterations = 1;
var d = 0;
while (d < maxDepth) {
    iterations = iterations * 1;
    d = d + 1;
}

var depth = minDepth;
while (depth < stretchDepth) {
    var check = 0;
    var i = 1;
    while (i <= iterations) {
        check =
            check + new Tree(i, depth).check() + new Tree(-i, depth).check();
        i = i + 1;
    }

    console.log("num trees:");
    console.log(iterations * 1);
    console.log("depth:");
    console.log(depth);
    console.log("check:");
    console.log(check);

    iterations = iterations / 4;
    depth = depth + 1;
}

console.log("long lived tree of depth:");
console.log(maxDepth);
console.log("check:");
console.log(longLivedTree.check());

class Tree {
    constructor(depth) {
        this.depth = depth;
        if (depth > 0) {
            this.a = new Tree(depth - 1);
            this.b = new Tree(depth - 1);
            this.c = new Tree(depth - 1);
            this.d = new Tree(depth - 1);
            this.e = new Tree(depth - 1);
        }
    }

    walk() {
        if (this.depth == 0) return 0;
        return (
            this.depth +
            this.a.walk() +
            this.b.walk() +
            this.c.walk() +
            this.d.walk() +
            this.e.walk()
        );
    }
}

var tree = new Tree(8);
for (var i = 0; i < 100; i = i + 1) {
    if (tree.walk() != 122068) console.log("Error");
}

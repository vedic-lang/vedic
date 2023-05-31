class Toggle {
    constructor(startState) {
        this.state = startState;
    }

    value() {
        return this.state;
    }

    activate() {
        this.state = !this.state;
        return this;
    }
}

class NthToggle extends Toggle {
    constructor(startState, maxCounter) {
        super(startState);
        this.countMax = maxCounter;
        this.count = 0;
    }

    activate() {
        this.count = this.count + 1;
        if (this.count >= this.countMax) {
            super.activate();
            this.count = 0;
        }

        return this;
    }
}

var n = 100000;
var val = true;
var toggle = new Toggle(val);

for (var i = 0; i < n; i = i + 1) {
    val = toggle.activate().value();
    val = toggle.activate().value();
    val = toggle.activate().value();
    val = toggle.activate().value();
    val = toggle.activate().value();
    val = toggle.activate().value();
    val = toggle.activate().value();
    val = toggle.activate().value();
    val = toggle.activate().value();
    val = toggle.activate().value();
}

console.log(toggle.value());

val = true;
var ntoggle = new NthToggle(val, 3);

for (var i = 0; i < n; i = i + 1) {
    val = ntoggle.activate().value();
    val = ntoggle.activate().value();
    val = ntoggle.activate().value();
    val = ntoggle.activate().value();
    val = ntoggle.activate().value();
    val = ntoggle.activate().value();
    val = ntoggle.activate().value();
    val = ntoggle.activate().value();
    val = ntoggle.activate().value();
    val = ntoggle.activate().value();
}

console.log(ntoggle.value());

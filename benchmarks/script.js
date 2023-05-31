// Color Codes const
LANG_COLORS = {
    vedic: "255, 159, 64", // orange
    nodejs: "75, 192, 192", // green
    python3: "54, 162, 235", // blue
    purple: "153, 102, 255",
    red: "255, 99, 132",
    yellow: "255, 205, 86",
    grey: "201, 203, 207",
};
let delayed;
async function graphConfig() {
    // fetch data from JSON file
    const response = await fetch("data.json");
    const data = await response.json();
    // create a new DataFrame
    const DataFrame = [];
    // get distinct test values
    const tests = [...new Set(data.map((item) => item.test))];
    // get distinct lang values
    const lang = [...new Set(data.map((item) => item.lang))];
    // loop through data and extract time values
    lang.forEach((lang) => {
        time_array = [];
        tests.forEach((test) => {
            const time = data.find(
                (item) => item.lang === lang && item.test === test
            ).time;
            time_array.push(Number(time).toFixed(3));
        });
        DataFrame.push({
            label: lang,
            data: time_array,
            backgroundColor: `rgba(${LANG_COLORS[lang]}, 0.5)`,
            borderColor: `rgba(${LANG_COLORS[lang]}, 1)`,
            borderWidth: 2,
            borderRadius: 2,
        });
    });
    config = {
        type: "bar",
        data: { labels: tests, datasets: DataFrame },
        options: {
            responsive: true,
            indexAxis: "x",
            plugins: {
                legend: { position: "top" },
                title: {
                    display: true,
                    text: "Comparison of the execution time",
                },
                subtitle: {
                    display: true,
                    text: "Vedic vs Nodejs vs Python3",
                    padding: {
                        bottom: 10,
                    },
                },
                tooltip: {
                    callbacks: {
                        label: function (context) {
                            let label = context.dataset.label || "";
                            if (label) {
                                label += ": ";
                            }
                            if (context.parsed.y !== null) {
                                label += context.parsed.y + " ms";
                            }
                            return label;
                        },
                    },
                },
            },
            animation: {
                onComplete: () => {
                    delayed = true;
                },
                delay: (context) => {
                    let delay = 0;
                    if (
                        context.type === "data" &&
                        context.mode === "default" &&
                        !delayed
                    ) {
                        delay =
                            context.dataIndex * 300 +
                            context.datasetIndex * 100;
                    }
                    return delay;
                },
            },
            scales: {
                x: {
                    title: {
                        color: "rgb(111, 111, 111)",
                        display: true,
                        text: "Test Cases",
                        font: {
                            weight: "bold",
                        },
                    },
                },
                y: {
                    ticks: {
                        // Include a ms sign in the ticks
                        callback: function (value, index, ticks) {
                            return value + " ms";
                        },
                    },
                    title: {
                        color: "rgb(111, 111, 111)",
                        display: true,
                        text: "Execution time (in Miliseconds)",
                        font: {
                            weight: "bold",
                        },
                    },
                },
            },
        },
    };
    return config;
}

window.onload = async function () {
    // Get the canvas element from the HTML file
    const canvas = document.getElementById("myChart");
    // Get the context of the canvas element
    const ctx = canvas.getContext("2d");
    const config = await graphConfig();
    window.graph = new Chart(ctx, config);
};

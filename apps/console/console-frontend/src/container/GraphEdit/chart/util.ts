export const toPorts = (way: "in" | "out", count: number) =>
    [...Array(count).keys()]
        .map((x) => ({
            id: `port-${way}-${x}`,
            type: way === "in" ? "top" : "bottom",
        }))
        .reduce((r, v) => {
            r[v.id] = v;
            return r;
        }, {});

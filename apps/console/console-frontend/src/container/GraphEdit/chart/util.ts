export const toPorts = (way: "in" | "out", count: number) => {
    const r = {};
    if (count != 0) {
        const k = `port-${way}-0`;
        r[k] = {
            id: `port-${way}-0`,
            type: way === "in" ? "top" : "bottom",
            properties: {
                max: count,
            },
        };
    }
    return r;
};

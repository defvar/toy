import { reducer, initialState } from "../../../src/modules/graphEdit/reducers";

describe("reducer GraphEdit", () => {
    it("action ZoomChart plus", () => {
        const state = { ...initialState };
        const r = reducer(state, { type: "ZoomChart", payload: 1 });
        expect(r.graph.scale).toBe(2);
    });

    it("action ZoomChart minus", () => {
        const state = initialState;
        const r = reducer(state, { type: "ZoomChart", payload: -1 });
        expect(r.graph.scale).toBe(0);
    });
});

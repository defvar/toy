import * as React from "react";
import { GraphList } from "./GraphList";
import { reducer, initialState } from "../../modules/graphs";

export const Graphs = () => {
    const [_state, dispatch] = React.useReducer(reducer, initialState);
    return (
        <>
            <GraphList dispatch={dispatch} />
        </>
    );
};

export default Graphs;

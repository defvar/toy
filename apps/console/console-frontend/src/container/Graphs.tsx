import * as React from 'react';
import { GraphList } from "../components/GraphList";
import { reducer, GraphState } from "../modules/graphs";

const items: { [key: string]: GraphState } = {
    "aaaa": { name: "aaaa", labels:["one", "a1", "two"], isActive: false },
    "bbbb": { name: "bbbb", labels:["b1", "b1v"], isActive: false },
};

export const Graphs = () => {
    const [state, dispatch] = React.useReducer(reducer, { items })
    // const onChangeActive = React.useCallback((name: string, isActive: boolean) => {
    //     setState(prev => {
    //         console.log(`onchange ${name}, ${isActive}`);
    //         return {
    //             ...prev,
    //             [name]: {
    //                 ...prev[name],
    //                 isActive
    //             }
    //         };
    //     });
    // }, []);

    return <GraphList items={state.items} onChangeActive={dispatch} />;
};

export default Graphs;

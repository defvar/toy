import { GraphEditState } from "./types";
import { Actions } from "./actions";

export const initialState: GraphEditState = {
    services: {},
    namespaces: {},
    graph: {
        nodes: {},
        wires: {},
    },
};

export const reducer = (
    state: GraphEditState = initialState,
    action: Actions
): GraphEditState => {
    switch (action.type) {
        case "GetServices":
            return {
                ...state,
                services: action.payload.services,
                namespaces: action.payload.namespaces,
            };
        case "GetGraph":
            return {
                ...state,
                graph: action.payload.graph,
            };
    }
};

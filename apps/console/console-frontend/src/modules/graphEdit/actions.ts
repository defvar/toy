import { ServiceState, GraphState } from "./types";

export interface GetServices {
    type: "GetServices";
    payload: {
        services: { [fullName: string]: ServiceState };
        namespaces: { [namespace: string]: string[] };
    };
}

export interface GetGraph {
    type: "GetGraph";
    payload: {
        graph: GraphState;
    };
}

export type Actions = GetServices | GetGraph;

import { ServiceResponse, GraphResponse } from "../api/toy-api";
import { ChartElements, NodeData } from "./types";

export interface GetServices {
    type: "GetServices";
    payload: ServiceResponse;
}

export interface GetGraph {
    type: "GetGraph";
    payload: GraphResponse;
}

export interface ChangeChart {
    type: "ChangeChart";
    payload: (prev: ChartElements) => ChartElements;
}

export interface AddNodeOnChart {
    type: "AddNodeOnChart";
    payload: {
        f: (prev: ChartElements) => ChartElements;
        node: NodeData;
    };
}

export interface RemoveNodeOnChart {
    type: "RemoveNodeOnChart";
    payload: {
        f: (prev: ChartElements) => ChartElements;
        removeNodeId: string;
    };
}

export interface ZoomChart {
    type: "ZoomChart";
    payload: number;
}

export interface StartEditNode {
    type: "StartEditNode";
    /**
     * node id
     */
    payload: string;
}

export interface ChangeEditNode {
    type: "ChangeEditNode";
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    payload: any;
}

export interface SubmitEditNode {
    type: "SubmitEditNode";
}

export interface CancelEditNode {
    type: "CancelEditNode";
}

export type Actions =
    | GetServices
    | GetGraph
    | ChangeChart
    | AddNodeOnChart
    | RemoveNodeOnChart
    | ZoomChart
    | StartEditNode
    | ChangeEditNode
    | SubmitEditNode
    | CancelEditNode;

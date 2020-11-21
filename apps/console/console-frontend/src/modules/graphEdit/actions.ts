import { ServiceResponse, GraphResponse } from "../api/toy-api";

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
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    payload: (prev: any) => any;
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
    | ZoomChart
    | StartEditNode
    | ChangeEditNode
    | SubmitEditNode
    | CancelEditNode;

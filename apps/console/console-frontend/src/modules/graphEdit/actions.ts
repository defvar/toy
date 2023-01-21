import { Graph } from "../api";
import { NodeData, LinkData } from "./types";

export interface GetGraph {
    type: "GetGraph";
    payload: Graph;
}

export interface AddLink {
    type: "AddLink";
    payload: (prev: LinkData[]) => LinkData[];
}

export interface UpdateLink {
    type: "UpdateLink";
    payload: (prev: LinkData[]) => LinkData[];
}

export interface ChangeLink {
    type: "ChangeLink";
    payload: (prev: LinkData[]) => LinkData[];
}

export interface AddNodeOnChart {
    type: "AddNodeOnChart";
    payload: {
        node: NodeData;
    };
}

export interface ChangeNode {
    type: "ChangeNode";
    payload: (prev: NodeData[]) => NodeData[];
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
    | GetGraph
    | AddLink
    | UpdateLink
    | ChangeLink
    | AddNodeOnChart
    | ChangeNode
    | ZoomChart
    | StartEditNode
    | ChangeEditNode
    | SubmitEditNode
    | CancelEditNode;

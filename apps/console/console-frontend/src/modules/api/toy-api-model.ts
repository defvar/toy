import { JsonSchema } from "../common";

export type PortTypeValue = number;
export type PortType = {
    Source?: PortTypeValue;
    Flow?: PortTypeValue[];
    Sink?: PortTypeValue;
};

export interface ErrorMessage {
    code: number;
    message: string;
}

//////////////////////////////////////
// service
//////////////////////////////////////
export interface ServiceSpec {
    service_type: string;
    name_space: string;
    service_name: string;
    port_type: PortType;
    schema?: JsonSchema;
}

export interface ServiceSpecList {
    readonly count: number;
    readonly items: ServiceSpec[];
}

//////////////////////////////////////
// graph
//////////////////////////////////////
export interface Graph {
    readonly name: string;
    readonly services?: GraphNode[];
    readonly labels?: string[];
}

export interface GraphNode {
    type: string;
    uri: string;
    position: {
        x: number;
        y: number;
    };
    port_type?: PortType;
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    config: any;
    wires: string[];
}

export interface GraphNodeList {
    readonly count: number;
    readonly items: Graph[];
}

//////////////////////////////////////
// rbac
//////////////////////////////////////
export interface Rule {
    resources: Array<string>;
    verbs: Array<string>;
}

export interface Role {
    name: string;
    note?: string;
    rules: Array<Rule>;
}

export interface RoleList {
    readonly count: number;
    readonly items: Role[];
}

export interface Subject {
    kind: "User" | "ServiceAccount";
    name: String;
}

export interface RoleBinding {
    name: String;
    role: String;
    subjects: Subject[];
}

export interface RoleBindingList {
    readonly items: RoleBinding[];
    readonly count: number;
}

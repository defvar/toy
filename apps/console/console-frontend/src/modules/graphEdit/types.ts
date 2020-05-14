export interface GraphEditState {
    services: { [fullName: string]: ServiceState };
    namespaces: { [namespace: string]: string[] };
    graph: GraphState;
}

export interface ServiceState {
    fullName: string;
    name: string;
    namespace: string;
    description: string;
    inPort: number;
    outPort: number;
}

export interface GraphState {
    nodes: { [uri: string]: NodeState };
    wires: { [uri: string]: string[] };
}

export interface NodeState {
    uri: string;
    fullName: string;
    name: string;
    namespace: string;
    description: string;
    inPort: number;
    outPort: number;
    position: {
        x: number;
        y: number;
    };
}

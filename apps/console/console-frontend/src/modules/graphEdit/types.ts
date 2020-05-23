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
    configSchema: JsonSchema;
}

export interface GraphState {
    nodes: { [uri: string]: NodeState };
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
    wires: string[];
}

export interface JsonSchema {
    $id?: string;
    $ref?: string;
    $schema?: string;

    type?: string;
    enum?: string[];

    required?: string[];
    properties?: { [key: string]: JsonSchema };

    definitions?: { [key: string]: JsonSchema };

    title?: string;
    description?: string;
}

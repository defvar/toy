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

export interface JsonSchema {
    id?: string;
    $schema?: string;
    title?: string;
    description?: string;
    required?: string[];
    definitions?: JsonSchemaMap;
    properties?: JsonSchemaMap;
    enum?: string[];
    type?: string;
    $ref?: string;
}

export interface JsonSchemaMap {
    [name: string]: JsonSchema;
}

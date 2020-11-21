import { JsonSchema } from "../common";

export type PortType = "Source" | "Flow" | "Sink";

export const initialChartData: ChartData = {
    offset: {
        x: 0,
        y: 0,
    },
    nodes: {},
    links: {},
    scale: 1,
    selected: {},
    hovered: {},
};

export interface ChartData {
    offset: {
        x: number;
        y: number;
    };
    nodes: {
        [id: string]: NodeData;
    };
    links: {
        [id: string]: LinkData;
    };
    scale: number;
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    properties?: any;
    selected: {};
    hovered: {};
}

export interface Port {
    id: string;
    type: string;
    value?: string;
    properties?: {
        max: number;
    };
}

export interface NodeData {
    id: string;
    type: string;
    position: {
        x: number;
        y: number;
    };
    orientation?: number;
    ports: {
        [id: string]: Port;
    };
    properties: {
        name: string;
        fullName: string;
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        config: any;
        dirty: boolean;
        portType: PortType;
    };
}

export interface LinkData {
    id: string;
    from: {
        nodeId: string;
        portId: string;
    };
    to: {
        nodeId?: string;
        portId?: string;
    };
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    properties?: any;
}

export interface GraphEditState {
    services: { [fullName: string]: ServiceState };
    namespaces: { [namespace: string]: string[] };
    graph: ChartData;
    edit: {
        /**
         * current edit node id.
         */
        id?: string;
        /**
         * current edit config object.
         */
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        config: any;
        configSchema: JsonSchema;
    };
}

export interface ServiceState {
    fullName: string;
    name: string;
    namespace: string;
    description: string;
    inPort: number;
    outPort: number;
    configSchema: JsonSchema;
    portType: PortType;
}

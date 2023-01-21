import { JsonSchema } from "../common";

export type PortType = "Source" | "Flow" | "Sink";

export const initialChartData: ChartData = {
    nodes: [],
    links: [],
};

export interface ChartData {
    nodes: Array<NodeData>;
    links: Array<LinkData>;
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
    type?: string;
    position: {
        x: number;
        y: number;
    };
    data: {
        name: string;
        label: string;
        fullName: string;
        dirty: boolean;
        portType: PortType;
    };
}

export interface LinkData {
    id: string;
    type?: string;
    source: string;
    target: string;
}

export interface GraphEditState {
    chart: ChartData;
    nodes: {
        [id: string]: {
            fullName: string;
            configSchema?: JsonSchema;
            // eslint-disable-next-line @typescript-eslint/no-explicit-any
            config: any;
        };
    };
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

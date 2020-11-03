import { getIdToken } from "../auth";
import { JsonSchema, toResource } from "../common";

export type PortTypeValue = number;
export type PortType = {
    Source?: PortTypeValue;
    Flow?: PortTypeValue[];
    Sink?: PortTypeValue;
};

export interface ServiceResponseItem {
    service_type: {
        full_name: string;
        name_space: string;
        service_name: string;
    };
    port_type: PortType;
    schema: JsonSchema;
}

export interface ServiceResponse {
    items: ServiceResponseItem[];
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

export interface GraphResponse {
    name: string;
    services: GraphNode[];
}

export const ToyApi = {
    getServices: async (): Promise<ServiceResponse> => {
        const key = await getIdToken();
        return fetch(`http://localhost:3030/services`, {
            method: "GET",
            mode: "cors",
            headers: {
                Authorization: `Bearer ${key}`,
            },
        })
            .then((res) => {
                if (res.ok) {
                    return res.json();
                }
                throw new Error("response was not ok.");
            })
            .then((json) => {
                return {
                    items: json,
                } as ServiceResponse;
            })
            .catch((error) => {
                console.log(
                    "There has been a problem with your fetch operation: ",
                    error.message
                );
                return {
                    items: [],
                };
            });
    },

    getGraph: async (name: string): Promise<GraphResponse> => {
        const key = await getIdToken();
        return fetch(`http://localhost:3030/graphs/${name}`, {
            method: "GET",
            mode: "cors",
            headers: {
                Authorization: `Bearer ${key}`,
            },
        })
            .then((res) => {
                if (res.ok) {
                    return res.json();
                }
                throw new Error("response was not ok.");
            })
            .then((json) => {
                return json as GraphResponse;
            })
            .catch((error) => {
                console.log(
                    "There has been a problem with your fetch operation: ",
                    error.message
                );
                return {
                    name: "",
                    services: [],
                };
            });
    },
};

export const fetchServices = () => {
    return toResource(ToyApi.getServices);
};

export const fetchGraph = (name: string) => {
    return toResource(() => ToyApi.getGraph(name));
};

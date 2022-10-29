import { getIdToken } from "../auth";
import { JsonSchema, toResource } from "../common";
import { config } from "./config";
import { RoleList, Role } from "./toy-api-model";

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
    readonly count: number;
    readonly items: ServiceResponseItem[];
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

async function commonRequest<T>(
    resource: string,
    method: string,
    defaultFunc: () => T
): Promise<T> {
    const key = await getIdToken();
    return fetch(`${config.root}/${resource}`, {
        method,
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
            return json as T;
        })
        .catch((error) => {
            console.log(
                "There has been a problem with your fetch operation: ",
                error.message
            );
            return defaultFunc();
        });
}

export const ToyApi = {
    getRoles: async (): Promise<RoleList> => {
        return commonRequest<RoleList>("rbac/roles", "GET", () => ({
            count: 0,
            items: [],
        }));
    },

    getRole: async (name: string): Promise<Role> => {
        return commonRequest<Role>(`rbac/roles/${name}`, "GET", () => ({
            name: "",
            rules: [],
        }));
    },

    getServices: async (): Promise<ServiceResponse> => {
        return commonRequest<ServiceResponse>("services", "GET", () => ({
            count: 0,
            items: [],
        }));
    },

    getGraph: async (name: string): Promise<GraphResponse> => {
        await new Promise((resolve) => setTimeout(resolve, 3000));

        const key = await getIdToken();
        return fetch(`${config.root}/graphs/${name}`, {
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

export const RbacClient = {
    fetchRoles: () => {
        return toResource(ToyApi.getRoles);
    },

    fetchRole: (name: string) => {
        return toResource(() => ToyApi.getRole(name));
    },
};

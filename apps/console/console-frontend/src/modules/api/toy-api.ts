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

export interface ErrorMessage {
    code: number;
    message: string;
}

async function commonRequest<T>(
    resource: string,
    method: string,
    body: string,
    defaultFunc: () => T
): Promise<T | ErrorMessage> {
    const key = await getIdToken();
    return fetch(`${config.root}/${resource}?format=json`, {
        method,
        mode: "cors",
        headers: {
            Authorization: `Bearer ${key}`,
        },
        body,
    })
        .then((res) => {
            return res.json().then((json) => ({ json, ok: res.ok }));
        })
        .then(({ json, ok }) => {
            if (ok) {
                return json as T;
            } else {
                return json as ErrorMessage;
            }
        })
        .catch((error) => {
            console.log(
                "There has been a problem with your fetch operation: ",
                error.message
            );
            return { code: -1, message: error.message };
        });
}

export const ToyApi = {
    getRoles: async (): Promise<RoleList | ErrorMessage> => {
        return commonRequest<RoleList>("rbac/roles", "GET", null, () => ({
            count: 0,
            items: [],
        }));
    },

    getRole: async (name: string): Promise<Role | ErrorMessage> => {
        return commonRequest<Role>(`rbac/roles/${name}`, "GET", null, () => ({
            name: "",
            rules: [],
        }));
    },

    putRole: async (
        name: string,
        body: string
    ): Promise<Role | ErrorMessage> => {
        return commonRequest<Role>(`rbac/roles/${name}`, "PUT", body, () => ({
            name: "",
            rules: [],
        }));
    },

    getServices: async (): Promise<ServiceResponse | ErrorMessage> => {
        return commonRequest<ServiceResponse>("services", "GET", null, () => ({
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

    putRole: (name: string, role: Role) => {
        return toResource(() => ToyApi.putRole(name, JSON.stringify(role)));
    },
};

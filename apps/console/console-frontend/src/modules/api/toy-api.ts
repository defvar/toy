import { getIdToken } from "../auth";
import { toResource, Result, Success, Failure } from "../common";
import { config } from "./config";
import {
    RoleList,
    Role,
    ServiceSpecList,
    ErrorMessage,
    GraphResponse,
} from "./toy-api-model";

async function commonRequest<T>(
    resource: string,
    method: string,
    body: string
): Promise<Result<T, ErrorMessage>> {
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
                return new Success(json as T);
            } else {
                return new Failure(json as ErrorMessage);
            }
        })
        .catch((error) => {
            console.log(
                "There has been a problem with your fetch operation: ",
                error.message
            );
            return new Failure({ code: -1, message: error.message });
        });
}

export const ToyApi = {
    getRoles: async (): Promise<Result<RoleList, ErrorMessage>> => {
        return commonRequest<RoleList>("rbac/roles", "GET", null);
    },

    getRole: async (name: string): Promise<Result<Role, ErrorMessage>> => {
        return commonRequest<Role>(`rbac/roles/${name}`, "GET", null);
    },

    putRole: async (
        name: string,
        body: string
    ): Promise<Result<Role, ErrorMessage>> => {
        return commonRequest<Role>(`rbac/roles/${name}`, "PUT", body);
    },

    getServices: async (): Promise<Result<ServiceSpecList, ErrorMessage>> => {
        return commonRequest<ServiceSpecList>("services", "GET", null);
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

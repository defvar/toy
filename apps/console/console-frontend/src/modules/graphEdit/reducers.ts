import {
    GraphEditState,
    ServiceState,
    ChartData,
    LinkData,
    NodeData,
    PortType,
} from "./types";
import { Actions } from "./actions";
import { ServiceResponseItem, GraphNode } from "../api/toy-api";
import { nextState } from "../../utils/immutable";

export const initialState: GraphEditState = {
    services: {},
    namespaces: {},
    graph: {
        offset: {
            x: 0,
            y: 0,
        },
        nodes: {},
        links: {},
        scale: 1,
        selected: {},
        hovered: {},
    },
    edit: {
        config: {},
        configSchema: {},
    },
};

const toServiceState = (item: ServiceResponseItem): ServiceState => {
    let inPort = 0;
    let outPort = 0;
    let portType: PortType = "Flow";
    if (item.port_type.Source) {
        outPort = item.port_type.Source;
        portType = "Source";
    } else if (item.port_type.Flow) {
        inPort = item.port_type.Flow[0];
        outPort = item.port_type.Flow[1];
        portType = "Flow";
    } else if (item.port_type.Sink) {
        inPort = item.port_type.Sink;
        portType = "Sink";
    }

    return {
        fullName: item.service_type.full_name,
        name: item.service_type.service_name,
        namespace: item.service_type.name_space,
        description: "",
        inPort,
        outPort,
        configSchema: item.schema,
        portType,
    };
};

export const toPorts = (way: "in" | "out", count: number) => {
    const r = {};
    if (count != 0) {
        const k = `port-${way}-0`;
        r[k] = {
            id: `port-${way}-0`,
            type: way === "in" ? "top" : "bottom",
            properties: {
                max: count,
            },
        };
    }
    return r;
};

const toLinks = (graph: {
    [uri: string]: GraphNode;
}): { [id: string]: LinkData } => {
    return Object.entries(graph).reduce((r, [k, v]) => {
        v.wires.map((x) => {
            const link = {
                id: `link-${k}-${x}`,
                from: {
                    nodeId: k,
                    portId: "port-out-0",
                },
                to: {
                    nodeId: x,
                    portId: "port-in-0",
                },
            };
            r[link.id] = link;
        });
        return r;
    }, {});
};

const toNodes = (graph: {
    [uri: string]: GraphNode;
}): { [id: string]: NodeData } => {
    return Object.entries(graph).reduce((r, [, node]) => {
        let inPort = 0;
        let outPort = 0;
        let portType: PortType = "Flow";

        if (node.port_type) {
            if (node.port_type.Source) {
                outPort = node.port_type.Source;
                portType = "Source";
            } else if (node.port_type.Flow) {
                inPort = node.port_type.Flow[0];
                outPort = node.port_type.Flow[1];
                portType = "Flow";
            } else if (node.port_type.Sink) {
                inPort = node.port_type.Sink;
                portType = "Sink";
            }
        }
        const inPorts = toPorts("in", inPort);
        const outPorts = toPorts("out", outPort);
        const allPorts = {
            ...inPorts,
            ...outPorts,
        };

        const n: NodeData = {
            id: node.uri,
            type: "top/bottom",
            position: node.position,
            ports: allPorts,
            properties: {
                config: node.config,
                name: node.type.split(".").slice(-1)[0],
                fullName: node.type,
                dirty: false,
                portType,
            },
        };

        r[n.id] = n;
        return r;
    }, {});
};

const toChartData = (graph: { [uri: string]: GraphNode }): ChartData => {
    const nodes = toNodes(graph);
    const links = toLinks(graph);
    const d = {
        offset: { x: 0, y: 0 },
        nodes,
        links,
        scale: 1,
        selected: {},
        hovered: {},
    };
    return d;
};

export const reducer = nextState(
    (state: GraphEditState = initialState, action: Actions): GraphEditState => {
        switch (action.type) {
            case "GetServices": {
                const r = action.payload.items
                    .map((x) => toServiceState(x))
                    .reduce(
                        (r, v) => {
                            r.services[v.fullName] = v;
                            if (r.namespaces[v.namespace]) {
                                r.namespaces[v.namespace].push(v.fullName);
                            } else {
                                r.namespaces[v.namespace] = [v.fullName];
                            }
                            return r;
                        },
                        { services: {}, namespaces: {} }
                    );
                state.services = r.services;
                state.namespaces = r.namespaces;
                return;
            }
            case "GetGraph": {
                const g = action.payload.services.reduce(
                    (r, v) => {
                        r.nodes[v.uri] = v;
                        return r;
                    },
                    { nodes: {} }
                );
                const r = toChartData(g.nodes);
                state.graph = r;
                return;
            }
            case "ChangeChart": {
                const r = action.payload(state.graph);
                state.graph = {
                    ...r,
                    properties: state.graph.properties,
                };
                return;
            }
            case "ZoomChart": {
                const scale = state.graph.scale + action.payload;
                state.graph.scale = scale;
                return;
            }
            case "StartEditNode": {
                console.debug(`StartEditNode:${action.payload}`);
                const currentEditId = state.edit.id;
                if (currentEditId && action.payload != currentEditId) {
                    const n = state.graph.nodes[currentEditId];
                    if (n) {
                        n.properties.config = { ...state.edit.config };
                    }
                }

                const n = state.graph.nodes[action.payload];
                let config = {};
                if (n && n.properties.config) {
                    config = { ...n.properties.config };
                }
                let configSchema = {};
                if (n && state.services[n.properties.fullName]) {
                    configSchema = {
                        ...state.services[n.properties.fullName].configSchema,
                    };
                }
                state.edit = {
                    id: action.payload,
                    config,
                    configSchema,
                };
                return;
            }
            case "ChangeEditNode": {
                const id = state.edit.id;
                const n = state.graph.nodes[id];
                n.properties.dirty = true;
                state.edit.config = { ...action.payload };
                return;
            }
            case "CancelEditNode": {
                state.edit = {
                    config: {},
                    configSchema: {},
                };
                return;
            }
            case "SubmitEditNode": {
                console.debug(`SubmitEditNode:${state.edit.id}`);
                const currentEditId = state.edit.id;
                if (currentEditId) {
                    const n = state.graph.nodes[currentEditId];
                    if (n) {
                        n.properties.config = { ...state.edit.config };
                    }
                    state.edit = {
                        config: {},
                        configSchema: {},
                    };
                }
            }
        }
    },
    initialState
);

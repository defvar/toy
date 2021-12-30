import {
    GraphEditState,
    ServiceState,
    ChartData,
    LinkData,
    NodeData,
    PortType,
    ChartElements,
} from "./types";
import { Actions } from "./actions";
import { ServiceResponseItem, GraphNode } from "../api/toy-api";
import { nextState } from "../../utils/immutable";

export const initialState: GraphEditState = {
    services: {},
    namespaces: {},
    nodes: {},
    chart: {
        elements: [],
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

const toLinks = (graph: { [uri: string]: GraphNode }): LinkData[] => {
    return Object.entries(graph).reduce((r, [k, v]) => {
        v.wires.map((x) => {
            const link = {
                id: `link-${k}-${x}`,
                source: k,
                target: x,
            };
            r.push(link);
        });
        return r;
    }, []);
};

const toNodes = (graph: {
    [uri: string]: GraphNode;
}): [{ [id: string]: any }, NodeData[]] => {
    return Object.entries(graph).reduce(
        (r, [, node]) => {
            let type = "default";
            let portType: PortType = "Flow";

            if (node.port_type) {
                if (node.port_type.Source) {
                    type = "input";
                    portType = "Source";
                } else if (node.port_type.Flow) {
                    type = "default";
                    portType = "Flow";
                } else if (node.port_type.Sink) {
                    type = "output";
                    portType = "Sink";
                }
            }

            const n: NodeData = {
                id: node.uri,
                type,
                position: node.position,
                data: {
                    name: node.type.split(".").slice(-1)[0],
                    label: node.type.split(".").slice(-1)[0],
                    fullName: node.type,
                    dirty: false,
                    portType,
                },
            };

            r[0][node.uri] = { fullName: node.type, config: node.config };
            r[1].push(n);
            return r;
        },
        [{}, []]
    );
};

const toChartData = (graph: {
    [uri: string]: GraphNode;
}): [{ [id: string]: { fullName: string; config: any } }, ChartData] => {
    const [configs, nodes] = toNodes(graph);
    const links = toLinks(graph);
    const elements: ChartElements = [...nodes, ...links];
    return [configs, { elements }];
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
                const [nodes, r] = toChartData(g.nodes);
                state.chart = r;
                state.nodes = nodes;
                return;
            }
            case "ChangeChart": {
                const elm = action.payload(state.chart.elements);
                state.chart.elements = elm;
                return;
            }
            case "StartEditNode": {
                console.debug(`StartEditNode:${action.payload}`);
                const currentEditId = state.edit.id;
                if (currentEditId && action.payload != currentEditId) {
                    const n = state.nodes[currentEditId];
                    if (n) {
                        n.config = { ...state.edit.config };
                    }
                }

                const n = state.nodes[action.payload];
                let config = {};
                if (n && n.config) {
                    config = { ...n.config };
                }
                let configSchema = {};
                if (n && state.services[n.fullName]) {
                    configSchema = {
                        ...state.services[n.fullName].configSchema,
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
                const n = state.nodes[id];
                // n.data.dirty = true;
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
                    const n = state.nodes[currentEditId];
                    if (n) {
                        n.config = { ...state.edit.config };
                    }
                    state.edit = {
                        config: {},
                        configSchema: {},
                    };
                }
                return;
            }
        }
    },
    initialState
);

import {
    GraphEditState,
    ChartData,
    LinkData,
    NodeData,
    PortType,
} from "./types";
import { Actions } from "./actions";
import { GraphNode } from "../api";
import { nextState } from "../../utils/immutable";

export const initialState: GraphEditState = {
    nodes: {},
    chart: {
        nodes: [],
        links: [],
    },
    edit: {
        config: {},
        configSchema: {},
    },
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
    return [configs, { nodes, links }];
};

export const reducer = nextState(
    (state: GraphEditState = initialState, action: Actions): GraphEditState => {
        switch (action.type) {
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
            case "AddLink": {
                const links = action.payload(state.chart.links);
                state.chart.links = links;
                return;
            }
            case "UpdateLink": {
                const links = action.payload(state.chart.links);
                state.chart.links = links;
                return;
            }
            case "ChangeLink": {
                const links = action.payload(state.chart.links);
                state.chart.links = links;
                return;
            }
            case "AddNodeOnChart": {
                const { node } = action.payload;
                if (!state.nodes[node.id]) {
                    state.nodes[node.id] = {
                        fullName: node.data.fullName,
                        config: {},
                    };
                }

                state.chart.nodes = state.chart.nodes.concat(node);
                return;
            }
            case "ChangeNode": {
                const nodes = action.payload(state.chart.nodes);
                state.chart.nodes = nodes;
                return;
            }
            case "StartEditNode": {
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
                if (n && n.configSchema) {
                    configSchema = {
                        ...n.configSchema,
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

import { NodeState, GraphState } from "../../../modules/graphEdit";
import { LinkData, NodeData, ChartData } from "./types";

export const toPorts = (way: "in" | "out", count: number) =>
    [...Array(count).keys()]
        .map((x) => ({
            id: `port-${way}-${x}`,
            type: way === "in" ? "top" : "bottom",
        }))
        .reduce((r, v) => {
            r[v.id] = v;
            return r;
        }, {});

const toLinks = (graph: {
    [uri: string]: NodeState;
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
    [uri: string]: NodeState;
}): { [id: string]: NodeData } => {
    return Object.entries(graph).reduce((r, [, node]) => {
        const inPorts = toPorts("in", node.inPort);
        const outPorts = toPorts("out", node.outPort);
        const allPorts = {
            ...inPorts,
            ...outPorts,
        };

        const n = {
            id: node.uri,
            type: "top/bottom",
            position: node.position,
            ports: allPorts,
            properties: {
                title: node.name,
                subheader: node.namespace,
            },
        };

        r[n.id] = n;
        return r;
    }, {});
};

export const toChartData = (graph: GraphState): ChartData => {
    const nodes = toNodes(graph.nodes);
    const links = toLinks(graph.nodes);
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

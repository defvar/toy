import { useState, DragEvent, MouseEvent } from "react";
import * as React from "react";
import { styled } from "@mui/material/styles";
import { ChartData, PortType } from "../../../modules/graphEdit/types";
import { Actions } from "../../../modules/graphEdit";
import ReactFlow, {
    ReactFlowProvider,
    addEdge,
    Connection,
    NodeChange,
    EdgeChange,
    Edge,
    Node,
    updateEdge,
    applyEdgeChanges,
    applyNodeChanges,
} from "reactflow";
import "reactflow/dist/style.css";
import { Resource } from "../../../modules/common";
import { GraphResponse } from "../../../modules/api/toy-api";

export interface ChartProps {
    data: ChartData;
    graphResource: Resource<GraphResponse>;
    dispatch: React.Dispatch<Actions>;
    height?: string | number;
}

export interface DragProps {
    type: string;
    name: string;
    fullName: string;
    portType: PortType;
}

const FlowArea = styled("div")(({ theme }) => ({
    display: "flex",
    width: "100%",
    height: "100%",
    flexGrow: 1,
}));

const FlowWrapper = styled("div")(({ theme }) => ({
    height: "100%",
    flexGrow: 1,
}));

const onDragOver = (event: DragEvent) => {
    event.preventDefault();
    event.dataTransfer.dropEffect = "move";
};

let id = 0;
const getId = () => `dndnode_${id++}`;

export const Chart = (props: ChartProps) => {
    const { data, graphResource, dispatch, height } = props;

    const graph = graphResource.read();
    React.useEffect(() => {
        dispatch({
            type: "GetGraph",
            payload: graph,
        });
    }, [graph]);

    const [reactFlowInstance, setReactFlowInstance] = useState(null);

    const onConnect = React.useCallback(
        (params: Connection | Edge) => {
            dispatch({
                type: "AddLink",
                payload: (e) => addEdge(params, e),
            });
        },
        [dispatch]
    );

    const onEdgeUpdate = React.useCallback(
        (oldEdge, newConnection) => {
            dispatch({
                type: "UpdateLink",
                payload: (e) => updateEdge(oldEdge, newConnection, e),
            });
        },
        [dispatch]
    );

    const onLoad = (_reactFlowInstance) =>
        setReactFlowInstance(_reactFlowInstance);

    const onDrop = React.useCallback(
        (event: DragEvent) => {
            event.preventDefault();

            if (reactFlowInstance) {
                const json = event.dataTransfer.getData(
                    "application/reactflow"
                );
                const obj: DragProps = JSON.parse(json);
                const position = reactFlowInstance.project({
                    x: event.clientX,
                    y: event.clientY,
                });
                const newNode: Node = {
                    id: getId(),
                    type: obj.type,
                    position,
                    data: {
                        name: obj.name,
                        label: obj.name,
                        fullName: obj.fullName,
                        dirty: false,
                        portType: obj.portType,
                    },
                };
                dispatch({
                    type: "AddNodeOnChart",
                    payload: {
                        node: newNode,
                    },
                });
            }
        },
        [dispatch, reactFlowInstance]
    );

    const onNodeDoubleClick = React.useCallback(
        (_event: MouseEvent, node: Node) => {
            dispatch({
                type: "StartEditNode",
                payload: node.id,
            });
        },
        [dispatch]
    );

    const onNodesChange = React.useCallback(
        (changes: NodeChange[]) => {
            dispatch({
                type: "ChangeNode",
                payload: (prev) => applyNodeChanges(changes, prev),
            });
        },
        [dispatch]
    );

    const onEdgesChange = React.useCallback(
        (changes: EdgeChange[]) => {
            dispatch({
                type: "ChangeLink",
                payload: (prev) => applyEdgeChanges(changes, prev),
            });
        },
        [dispatch]
    );

    return (
        <FlowArea sx={{ height }}>
            <ReactFlowProvider>
                <FlowWrapper sx={{ height }}>
                    <ReactFlow
                        nodes={data.nodes}
                        edges={data.links}
                        onConnect={onConnect}
                        onNodesChange={onNodesChange}
                        onEdgesChange={onEdgesChange}
                        onEdgeUpdate={onEdgeUpdate}
                        onLoad={onLoad}
                        onDrop={onDrop}
                        onDragOver={onDragOver}
                        onNodeDoubleClick={onNodeDoubleClick}
                    ></ReactFlow>
                </FlowWrapper>
            </ReactFlowProvider>
        </FlowArea>
    );
};

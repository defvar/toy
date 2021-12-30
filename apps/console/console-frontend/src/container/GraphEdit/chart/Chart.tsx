import { useState, DragEvent, MouseEvent } from "react";
import * as React from "react";
import { styled } from "@mui/material/styles";
import { ChartData } from "../../../modules/graphEdit/types";
import { Actions } from "../../../modules/graphEdit";
import ReactFlow, {
    ReactFlowProvider,
    addEdge,
    removeElements,
    OnLoadParams,
    Elements,
    Connection,
    Edge,
    ElementId,
    Node,
} from "react-flow-renderer";
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
const getId = (): ElementId => `dndnode_${id++}`;

export const Chart = (props: ChartProps) => {
    const { data, graphResource, dispatch, height } = props;

    const graph = graphResource.read();
    React.useEffect(() => {
        dispatch({
            type: "GetGraph",
            payload: graph,
        });
    }, [graph]);

    const [reactFlowInstance, setReactFlowInstance] = useState<OnLoadParams>();

    const onConnect = React.useCallback(
        (params: Connection | Edge) => {
            dispatch({
                type: "ChangeChart",
                payload: (elm) => addEdge(params, elm),
            });
        },
        [dispatch]
    );
    const onElementsRemove = React.useCallback(
        (elementsToRemove: Elements) => {
            dispatch({
                type: "ChangeChart",
                payload: (elm) => removeElements(elementsToRemove, elm),
            });
        },
        [dispatch]
    );
    const onLoad = (_reactFlowInstance: OnLoadParams) =>
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
                    data: { label: obj.name },
                };
                dispatch({
                    type: "ChangeChart",
                    payload: (elm) => elm.concat(newNode),
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

    return (
        <FlowArea sx={{ height }}>
            <ReactFlowProvider>
                <FlowWrapper sx={{ height }}>
                    <ReactFlow
                        elements={data.elements}
                        onConnect={onConnect}
                        onElementsRemove={onElementsRemove}
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

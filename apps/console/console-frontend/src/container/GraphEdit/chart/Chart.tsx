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
    Controls,
} from "react-flow-renderer";

export interface ChartProps {
    data: ChartData;
    dispatch: React.Dispatch<Actions>;
}

export interface DragProps {
    type: string;
    fullName: string;
}

const FlowArea = styled("div")(({ theme }) => ({
    flexDirection: "column",
    display: "flex",
    height: "100%",
    width: "100%",
    flexGrow: 1,
}));

const FlowWrapper = styled("div")(({ theme }) => ({
    flexGrow: 1,
    height: "70vh",
}));

const onDragOver = (event: DragEvent) => {
    event.preventDefault();
    event.dataTransfer.dropEffect = "move";
};

let id = 0;
const getId = (): ElementId => `dndnode_${id++}`;

export const Chart = React.memo((props: ChartProps) => {
    const { data, dispatch } = props;

    const [reactFlowInstance, setReactFlowInstance] = useState<OnLoadParams>();
    const [elements, setElements] = useState<Elements>(data.elements);

    const onConnect = (params: Connection | Edge) =>
        setElements((els) => addEdge(params, els));
    const onElementsRemove = (elementsToRemove: Elements) =>
        setElements((els) => removeElements(elementsToRemove, els));
    const onLoad = (_reactFlowInstance: OnLoadParams) =>
        setReactFlowInstance(_reactFlowInstance);

    const onDrop = (event: DragEvent) => {
        event.preventDefault();

        if (reactFlowInstance) {
            const json = event.dataTransfer.getData("application/reactflow");
            const obj: DragProps = JSON.parse(json);
            const position = reactFlowInstance.project({
                x: event.clientX,
                y: event.clientY - 40,
            });
            const newNode: Node = {
                id: getId(),
                type: obj.type,
                position,
                data: { label: obj.fullName },
            };

            setElements((es) => es.concat(newNode));
        }
    };

    const onNodeDoubleClick = React.useCallback(
        (_event: MouseEvent, node: Node) => {
            dispatch({
                type: "StartEditNode",
                payload: node.id,
            });
        },
        [dispatch]
    );

    console.log(data.elements);

    return (
        <FlowArea>
            <ReactFlowProvider>
                <FlowWrapper>
                    <ReactFlow
                        elements={elements}
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
});

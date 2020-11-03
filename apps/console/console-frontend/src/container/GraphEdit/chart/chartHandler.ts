import {
    actions,
    IFlowChartCallbacks,
    IOnDragNodeInput,
    IOnDragNodeStopInput,
    IOnDragCanvasInput,
    IOnCanvasDropInput,
    IOnDragCanvasStopInput,
    IOnLinkBaseEvent,
    IOnLinkMoveInput,
    IOnLinkCompleteInput,
    IOnPortPositionChangeInput,
    ILinkBaseInput,
    IConfig,
    INodeBaseInput,
    IOnNodeSizeChangeInput,
} from "@mrblenny/react-flow-chart";
import { Actions } from "../../../modules/graphEdit";

export const chartHandler = (
    dispatch: React.Dispatch<Actions>
): IFlowChartCallbacks => {
    const r = {
        onDragNode: (input: IOnDragNodeInput) => {
            dispatch({
                type: "ChangeChart",
                payload: actions.onDragNode(input),
            });
        },
        onDragNodeStop: (input: IOnDragNodeStopInput) => {
            dispatch({
                type: "ChangeChart",
                payload: actions.onDragNodeStop(input),
            });
        },
        onDragCanvas: (input: IOnDragCanvasInput) => {
            actions.onDragCanvas(input);
        },
        onCanvasDrop: (input: IOnCanvasDropInput) => {
            dispatch({
                type: "ChangeChart",
                payload: actions.onCanvasDrop(input),
            });
        },
        onDragCanvasStop: (input: IOnDragCanvasStopInput) => {
            dispatch({
                type: "ChangeChart",
                payload: actions.onDragCanvasStop(input),
            });
        },
        onLinkStart: (input: IOnLinkBaseEvent) => {
            dispatch({
                type: "ChangeChart",
                payload: actions.onLinkStart(input),
            });
        },
        onLinkMove: (input: IOnLinkMoveInput) => {
            dispatch({
                type: "ChangeChart",
                payload: actions.onLinkMove(input),
            });
        },
        onLinkComplete: (input: IOnLinkCompleteInput) => {
            dispatch({
                type: "ChangeChart",
                payload: actions.onLinkComplete(input),
            });
        },
        onLinkCancel: (input: IOnLinkBaseEvent) => {
            dispatch({
                type: "ChangeChart",
                payload: actions.onLinkCancel(input),
            });
        },
        onPortPositionChange: (input: IOnPortPositionChangeInput) => {
            dispatch({
                type: "ChangeChart",
                payload: actions.onPortPositionChange(input),
            });
        },
        onLinkMouseEnter: (input: ILinkBaseInput) => {
            dispatch({
                type: "ChangeChart",
                payload: actions.onLinkMouseEnter(input),
            });
        },
        onLinkMouseLeave: (input: ILinkBaseInput) => {
            dispatch({
                type: "ChangeChart",
                payload: actions.onLinkMouseLeave(input),
            });
        },
        onLinkClick: (input: ILinkBaseInput) => {
            dispatch({
                type: "ChangeChart",
                payload: actions.onLinkClick(input),
            });
        },
        onCanvasClick: (input: { config?: IConfig }) => {
            dispatch({
                type: "ChangeChart",
                payload: actions.onCanvasClick(input),
            });
        },
        onDeleteKey: (input: { config?: IConfig }) => {
            dispatch({
                type: "ChangeChart",
                payload: actions.onDeleteKey(input),
            });
        },
        onNodeClick: (input: INodeBaseInput) => {
            dispatch({
                type: "ChangeChart",
                payload: actions.onNodeClick(input),
            });
        },
        onNodeDoubleClick: (input: INodeBaseInput) => {
            dispatch({
                type: "ChangeChart",
                payload: actions.onNodeDoubleClick(input),
            });
        },
        onNodeMouseEnter: (input: INodeBaseInput) => {
            dispatch({
                type: "ChangeChart",
                payload: actions.onNodeMouseEnter(input),
            });
        },
        onNodeMouseLeave: (input: INodeBaseInput) => {
            dispatch({
                type: "ChangeChart",
                payload: actions.onNodeMouseLeave(input),
            });
        },
        onNodeSizeChange: (input: IOnNodeSizeChangeInput) => {
            dispatch({
                type: "ChangeChart",
                payload: actions.onNodeSizeChange(input),
            });
        },
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        onZoomCanvas: (input: { config?: IConfig; data: any }) => {
            actions.onZoomCanvas(input);
        },
    };
    return r;
};

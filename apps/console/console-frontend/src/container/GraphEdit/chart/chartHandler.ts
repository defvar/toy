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
import { ChartData } from "./types";

export const chartHandler = (
    dispatch: React.Dispatch<React.SetStateAction<ChartData>>
): IFlowChartCallbacks => {
    const r = {
        onDragNode: (input: IOnDragNodeInput) => {
            dispatch((prev) => {
                const r = actions.onDragNode(input)(prev);
                return {
                    ...prev,
                    ...r,
                };
            });
        },
        onDragNodeStop: (input: IOnDragNodeStopInput) => {
            dispatch((prev) => {
                const r = actions.onDragNodeStop(input)(prev);
                return {
                    ...prev,
                    ...r,
                };
            });
        },
        onDragCanvas: (input: IOnDragCanvasInput) => {
            dispatch((prev) => {
                actions.onDragCanvas(input);
                return prev;
            });
        },
        onCanvasDrop: (input: IOnCanvasDropInput) => {
            dispatch((prev) => {
                const r = actions.onCanvasDrop(input)(prev);
                return {
                    ...prev,
                    ...r,
                };
            });
        },
        onDragCanvasStop: (input: IOnDragCanvasStopInput) => {
            dispatch((prev) => {
                const r = actions.onDragCanvasStop(input)(prev);
                return {
                    ...prev,
                    ...r,
                };
            });
        },
        onLinkStart: (input: IOnLinkBaseEvent) => {
            dispatch((prev) => {
                const r = actions.onLinkStart(input)(prev);
                return {
                    ...prev,
                    ...r,
                };
            });
        },
        onLinkMove: (input: IOnLinkMoveInput) => {
            dispatch((prev) => {
                const r = actions.onLinkStart(input)(prev);
                return {
                    ...prev,
                    ...r,
                };
            });
        },
        onLinkComplete: (input: IOnLinkCompleteInput) => {
            dispatch((prev) => {
                const r = actions.onLinkStart(input)(prev);
                return {
                    ...prev,
                    ...r,
                };
            });
        },
        onLinkCancel: (input: IOnLinkBaseEvent) => {
            dispatch((prev) => {
                const r = actions.onLinkStart(input)(prev);
                return {
                    ...prev,
                    ...r,
                };
            });
        },
        onPortPositionChange: (input: IOnPortPositionChangeInput) => {
            dispatch((prev) => {
                const r = actions.onPortPositionChange(input)(prev);
                return {
                    ...prev,
                    ...r,
                };
            });
        },
        onLinkMouseEnter: (input: ILinkBaseInput) => {
            dispatch((prev) => {
                const r = actions.onLinkMouseEnter(input)(prev);
                return {
                    ...prev,
                    ...r,
                };
            });
        },
        onLinkMouseLeave: (input: ILinkBaseInput) => {
            dispatch((prev) => {
                const r = actions.onLinkMouseLeave(input)(prev);
                return {
                    ...prev,
                    ...r,
                };
            });
        },
        onLinkClick: (input: ILinkBaseInput) => {
            dispatch((prev) => {
                const r = actions.onLinkClick(input)(prev);
                return {
                    ...prev,
                    ...r,
                };
            });
        },
        onCanvasClick: (input: { config?: IConfig }) => {
            dispatch((prev) => {
                const r = actions.onCanvasClick(input)(prev);
                return {
                    ...prev,
                    ...r,
                };
            });
        },
        onDeleteKey: (input: { config?: IConfig }) => {
            dispatch((prev) => {
                const r = actions.onDeleteKey(input)(prev);
                return {
                    ...prev,
                    ...r,
                };
            });
        },
        onNodeClick: (input: INodeBaseInput) => {
            dispatch((prev) => {
                const r = actions.onNodeClick(input)(prev);
                return {
                    ...prev,
                    ...r,
                };
            });
        },
        onNodeDoubleClick: (input: INodeBaseInput) => {
            dispatch((prev) => {
                const r = actions.onNodeDoubleClick(input)(prev);
                return {
                    ...prev,
                    ...r,
                };
            });
        },
        onNodeMouseEnter: (input: INodeBaseInput) => {
            dispatch((prev) => {
                const r = actions.onNodeMouseEnter(input)(prev);
                return {
                    ...prev,
                    ...r,
                };
            });
        },
        onNodeMouseLeave: (input: INodeBaseInput) => {
            dispatch((prev) => {
                const r = actions.onNodeMouseLeave(input)(prev);
                return {
                    ...prev,
                    ...r,
                };
            });
        },
        onNodeSizeChange: (input: IOnNodeSizeChangeInput) => {
            dispatch((prev) => {
                const r = actions.onNodeSizeChange(input)(prev);
                return {
                    ...prev,
                    ...r,
                };
            });
        },
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        onZoomCanvas: (input: { config?: IConfig; data: any }) => {
            dispatch((prev) => {
                actions.onZoomCanvas(input);
                return prev;
            });
        },
    };
    return r;
};

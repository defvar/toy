import * as React from "react";
import { FlowChart, actions } from "@mrblenny/react-flow-chart";
import { Node } from "./Node";
import { ChartData } from "./types";

export interface ChartProps {
    data: ChartData;
    dispatch: React.Dispatch<(prev: ChartData) => ChartData>;
}

const createHandlers = (
    dispatch: React.Dispatch<(prev: ChartData) => ChartData>
) =>
    Object.entries(actions).reduce((r, [key, fn]) => {
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        r[key] = (...args: any) => {
            dispatch((prev) => {
                const r = fn(...args)(prev);
                return {
                    ...prev,
                    ...r,
                };
            });
            return;
        };
        return r;
    }, {}) as typeof actions;

export const Chart = (props: ChartProps) => {
    const { data, dispatch } = props;
    const [handlers] = React.useState(() => createHandlers(dispatch));

    return (
        <FlowChart
            chart={data}
            Components={{ NodeInner: Node }}
            callbacks={handlers}
            config={{ zoom: { wheel: { disabled: true } } }}
        />
    );
};

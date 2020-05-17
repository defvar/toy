import * as React from "react";
import { FlowChart } from "@mrblenny/react-flow-chart";
import { Node } from "./Node";
import { ChartData } from "./types";
import { chartHandler } from "./chartHandler";

export interface ChartProps {
    data: ChartData;
    dispatch: React.Dispatch<React.SetStateAction<ChartData>>;
}

export const Chart = (props: ChartProps) => {
    const { data, dispatch } = props;
    const [handlers] = React.useState(() => chartHandler(dispatch));

    return (
        <FlowChart
            chart={data}
            Components={{ NodeInner: Node }}
            callbacks={handlers}
            config={{ zoom: { wheel: { disabled: true } } }}
        />
    );
};

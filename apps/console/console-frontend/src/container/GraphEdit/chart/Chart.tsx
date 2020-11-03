import * as React from "react";
import { FlowChart } from "@mrblenny/react-flow-chart";
import { Node } from "./Node";
import { ChartData } from "../../../modules/graphEdit/types";
import { chartHandler } from "./chartHandler";
import { Actions } from "../../../modules/graphEdit";

export interface ChartProps {
    data: ChartData;
    dispatch: React.Dispatch<Actions>;
}

export const Chart = React.memo((props: ChartProps) => {
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
});

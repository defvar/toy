import * as React from "react";
import { FlowChart } from "@mrblenny/react-flow-chart";
import { Node } from "./Node";
import { ChartData } from "../../../modules/graphEdit/types";
import { chartHandler } from "./chartHandler";
import { Actions } from "../../../modules/graphEdit";
import { CustomPort } from "./Port";

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
            Components={{ NodeInner: Node, Port: CustomPort }}
            callbacks={handlers}
            config={{
                zoom: { wheel: { disabled: true } },
                selectable: true,
                nodeProps: { dispatch },
            }}
        />
    );
});

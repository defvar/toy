import * as React from "react";
import { REACT_FLOW_CHART } from "@mrblenny/react-flow-chart";
import { createStyles, Theme, makeStyles } from "@material-ui/core/styles";
import Card from "@material-ui/core/Card";
import { ServiceCardHeader } from "./ServiceCardHeader";
import { toPorts } from "./util";

const useStyles = makeStyles((theme: Theme) =>
    createStyles({
        sideBarItem: {
            cursor: "move",
            margin: theme.spacing(0, 0),
            flex: 1,
            maxWidth: 300,
        },
    })
);

export interface SidebarItemProps {
    fullName: string;
    name: string;
    namespace: string;
    description: string;
    inPort: number;
    outPort: number;
    properties?: {
        icon?: string;
    };
}

export const SidebarItem = ({
    name,
    namespace,
    inPort,
    outPort,
}: SidebarItemProps): JSX.Element => {
    const classes = useStyles();

    const inPorts = toPorts("in", inPort);
    const outPorts = toPorts("out", outPort);
    const allPorts = {
        ...inPorts,
        ...outPorts,
    };

    return (
        <div
            className={classes.sideBarItem}
            draggable={true}
            onDragStart={(event): void => {
                event.dataTransfer.setData(
                    REACT_FLOW_CHART,
                    JSON.stringify({
                        type: "top/bottom",
                        ports: allPorts,
                        properties: {
                            title: name,
                            subheader: namespace,
                        },
                    })
                );
            }}
        >
            <Card>
                <ServiceCardHeader title={name} subheader={namespace} />
            </Card>
        </div>
    );
};

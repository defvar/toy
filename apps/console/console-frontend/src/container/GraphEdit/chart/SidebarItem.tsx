import * as React from "react";
import { REACT_FLOW_CHART } from "@mrblenny/react-flow-chart";
import { createStyles, Theme, makeStyles } from "@material-ui/core/styles";
import Card from "@material-ui/core/Card";
import { ServiceCardHeader } from "./ServiceCardHeader";
import { toPorts } from "./util";
import { NodeData, PortType } from "../../../modules/graphEdit/types";

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
    portType: PortType;
}

export const SidebarItem = ({
    fullName,
    name,
    inPort,
    outPort,
    portType,
}: SidebarItemProps): JSX.Element => {
    const classes = useStyles();

    const inPorts = toPorts("in", inPort);
    const outPorts = toPorts("out", outPort);
    const allPorts = {
        ...inPorts,
        ...outPorts,
    };

    const dropItem: NodeData = {
        id: null,
        type: "top/bottom",
        position: { x: 0, y: 0 },
        ports: allPorts,
        properties: {
            name,
            fullName,
            config: {},
            dirty: false,
            portType,
        },
    };

    return (
        <div
            className={classes.sideBarItem}
            draggable={true}
            onDragStart={(event): void => {
                event.dataTransfer.setData(
                    REACT_FLOW_CHART,
                    JSON.stringify(dropItem)
                );
            }}
        >
            <Card>
                <ServiceCardHeader
                    title={name}
                    dirty={false}
                    portType={portType}
                />
            </Card>
        </div>
    );
};

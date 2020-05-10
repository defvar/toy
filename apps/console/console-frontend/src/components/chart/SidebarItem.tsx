import * as React from "react";
import { REACT_FLOW_CHART } from "@mrblenny/react-flow-chart";
import { createStyles, Theme, makeStyles } from "@material-ui/core/styles";
import Card from "@material-ui/core/Card";
import { ServiceCardHeader } from "./ServiceCardHeader";

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

const getPorts = (way: "in" | "out", count: number) =>
    [...Array(count).keys()]
        .map((x) => ({
            id: `port-${way}-${x}`,
            type: way === "in" ? "top" : "bottom",
        }))
        .reduce((r, v) => {
            r[v.id] = v;
            return r;
        }, {});

export const SidebarItem = ({
    fullName,
    name,
    namespace,
    description,
    inPort,
    outPort,
}: SidebarItemProps): JSX.Element => {
    const classes = useStyles();

    const inPorts = getPorts("in", inPort);
    const outPorts = getPorts("out", outPort);
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
                            name,
                            fullName,
                            namespace,
                            description,
                        },
                    })
                );
            }}
        >
            <Card>
                <ServiceCardHeader
                    name={name}
                    fullName={fullName}
                    namespace={namespace}
                />
            </Card>
        </div>
    );
};

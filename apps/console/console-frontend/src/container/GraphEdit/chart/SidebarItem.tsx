import * as React from "react";
import { Theme } from "@mui/material/styles";
import createStyles from "@mui/styles/createStyles";
import makeStyles from "@mui/styles/makeStyles";
import Card from "@mui/material/Card";
import { ServiceCardHeader } from "./ServiceCardHeader";
import { PortType } from "../../../modules/graphEdit/types";

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
    portType: PortType;
}

export const SidebarItem = ({
    fullName,
    name,
    portType,
}: SidebarItemProps): JSX.Element => {
    const classes = useStyles();
    return (
        <div
            className={classes.sideBarItem}
            draggable
            onDragStart={(event): void => {
                event.dataTransfer.setData(
                    "application/reactflow",
                    JSON.stringify({ type: "default", fullName })
                    );
                event.dataTransfer.effectAllowed = "move";
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

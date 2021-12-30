import * as React from "react";
import { styled } from "@mui/material/styles";
import { PortType } from "../../../modules/graphEdit/types";
import FunctionsIcon from "@mui/icons-material/Functions";
import AllOutIcon from "@mui/icons-material/AllOut";
import CheckBoxOutlineBlankIcon from "@mui/icons-material/CheckBoxOutlineBlank";
import { Card, CardContent, CardHeader, Typography } from "@mui/material";

export interface SidebarItemProps {
    fullName: string;
    name: string;
    namespace: string;
    description: string;
    portType: PortType;
}

const getIcon = (portType: PortType) => {
    switch (portType) {
        case "Source":
            return <AllOutIcon />;
        case "Flow":
            return <FunctionsIcon />;
        case "Sink":
            return <CheckBoxOutlineBlankIcon />;
        default:
            return <CheckBoxOutlineBlankIcon />;
    }
};

const Draggable = styled("div")(({ theme }) => ({
    cursor: "move",
}));

export const SidebarItem = ({
    fullName,
    name,
    portType,
    description,
}: SidebarItemProps): JSX.Element => {
    return (
        <Card sx={{ flex: 1, maxWidth: 300 }}>
            <Draggable
                draggable
                onDragStart={(event): void => {
                    event.dataTransfer.setData(
                        "application/reactflow",
                        JSON.stringify({ type: "default", fullName, name })
                    );
                    event.dataTransfer.effectAllowed = "move";
                }}
            >
                <CardHeader avatar={getIcon(portType)} title={name} />
            </Draggable>
            <CardContent>
                <Typography
                    variant="body2"
                    color="text.secondary"
                    component="div"
                >
                    {description ? description : "no description..."}
                </Typography>
            </CardContent>
        </Card>
    );
};

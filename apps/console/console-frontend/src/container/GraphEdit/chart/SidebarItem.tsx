import * as React from "react";
import { styled } from "@mui/material/styles";
import Card from "@mui/material/Card";
import { PortType } from "../../../modules/graphEdit/types";
import FunctionsIcon from "@mui/icons-material/Functions";
import AllOutIcon from "@mui/icons-material/AllOut";
import CheckBoxOutlineBlankIcon from "@mui/icons-material/CheckBoxOutlineBlank";
import CardHeader from "@mui/material/CardHeader";

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

const Wrapper = styled("div")(({ theme }) => ({
    cursor: "move",
    margin: theme.spacing(0, 0),
    flex: 1,
    maxWidth: 300,
}));

export const SidebarItem = ({
    fullName,
    name,
    portType,
}: SidebarItemProps): JSX.Element => {
    return (
        <Wrapper
            draggable
            onDragStart={(event): void => {
                event.dataTransfer.setData(
                    "application/reactflow",
                    JSON.stringify({ type: "default", fullName, name })
                );
                event.dataTransfer.effectAllowed = "move";
            }}
        >
            <Card>
                <CardHeader avatar={getIcon(portType)} title={name} />
            </Card>
        </Wrapper>
    );
};

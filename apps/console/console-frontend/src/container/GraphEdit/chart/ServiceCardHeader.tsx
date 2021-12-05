import * as React from "react";
import { Theme } from "@mui/material/styles";
import makeStyles from '@mui/styles/makeStyles';
import createStyles from '@mui/styles/createStyles';
import FunctionsIcon from "@mui/icons-material/Functions";
import AllOutIcon from "@mui/icons-material/AllOut";
import CheckBoxOutlineBlankIcon from "@mui/icons-material/CheckBoxOutlineBlank";
import CardHeader from "@mui/material/CardHeader";
import { PortType } from "../../../modules/graphEdit/types";

const useStyles = makeStyles((theme: Theme) =>
    createStyles({
        card: {
            padding: theme.spacing(1),
        },
    })
);

export interface ServiceCardHeaderProps {
    title: string;
    dirty: boolean;
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

export const ServiceCardHeader = (props: ServiceCardHeaderProps) => {
    const classes = useStyles();
    const title = props.dirty ? ` * ${props.title}` : props.title;

    return (
        <CardHeader
            className={classes.card}
            avatar={getIcon(props.portType)}
            title={title}
        />
    );
};

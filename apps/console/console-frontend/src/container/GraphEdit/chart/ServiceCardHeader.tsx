import * as React from "react";
import { makeStyles, Theme, createStyles } from "@material-ui/core/styles";
import FunctionsIcon from "@material-ui/icons/Functions";
import AllOutIcon from "@material-ui/icons/AllOut";
import CheckBoxOutlineBlankIcon from "@material-ui/icons/CheckBoxOutlineBlank";
import CardHeader from "@material-ui/core/CardHeader";
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

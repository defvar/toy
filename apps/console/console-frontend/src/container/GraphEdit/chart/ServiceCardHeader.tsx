import * as React from "react";
import { makeStyles, Theme, createStyles } from "@material-ui/core/styles";
import DescriptionIcon from "@material-ui/icons/Description";
import CheckBoxOutlineBlankIcon from "@material-ui/icons/CheckBoxOutlineBlank";
import CardHeader from "@material-ui/core/CardHeader";

const useStyles = makeStyles((theme: Theme) =>
    createStyles({
        card: {
            padding: theme.spacing(1),
        },
    })
);

export interface ServiceCardHeaderProps {
    title: string;
    subheader: string;
    icon?: string;
}

const getIcon = (icon: string) => {
    switch (icon) {
        case "file":
            return <DescriptionIcon />;
        default:
            return <CheckBoxOutlineBlankIcon />;
    }
};

export const ServiceCardHeader = (props: ServiceCardHeaderProps) => {
    const classes = useStyles();

    return (
        <CardHeader
            className={classes.card}
            avatar={
                getIcon(props.icon)
                // <Avatar aria-label="service-icon">{getIcon(props.icon)}</Avatar>
            }
            title={props.title}
            subheader={props.subheader}
        />
    );
};

import * as React from "react";
import { Theme } from "@mui/material/styles";
import createStyles from "@mui/styles/createStyles";
import makeStyles from "@mui/styles/makeStyles";
import List from "@mui/material/List";
import ListSubheader from "@mui/material/ListSubheader";
import ListItem from "@mui/material/ListItem";
import ListItemText from "@mui/material/ListItemText";
import Collapse from "@mui/material/Collapse";
import ExpandLess from "@mui/icons-material/ExpandLess";
import ExpandMore from "@mui/icons-material/ExpandMore";
import { SidebarItem } from "./SidebarItem";
import { PortType } from "../../../modules/graphEdit/types";

const useStyles = makeStyles((theme: Theme) =>
    createStyles({
        root: {
            padding: theme.spacing(0),
            backgroundColor: theme.palette.background.paper,
        },
        nested: {
            paddingLeft: theme.spacing(4),
        },
    })
);

export interface SidebarProps {
    services: {
        [fullName: string]: {
            fullName: string;
            name: string;
            namespace: string;
            description: string;
            inPort: number;
            outPort: number;
            portType: PortType;
        };
    };
    namespaces: {
        [namespace: string]: string[];
    };
}

export const Sidebar = (props: SidebarProps) => {
    const classes = useStyles();
    const [open, setOpen] = React.useState<{ [namespace: string]: boolean }>(
        {}
    );

    const handleClick = (e: React.MouseEvent<HTMLDivElement, MouseEvent>) => {
        const namespace = e.currentTarget.dataset.namespace;
        setOpen((prev) => ({
            ...prev,
            [namespace]: !prev[namespace],
        }));
    };

    return (
        <List>
            {Object.entries(props.namespaces).map(([namespace, entry]) => {
                return (
                    <React.Fragment key={namespace}>
                        <ListItem
                            key={`header-${namespace}`}
                            data-namespace={namespace}
                            button
                            onClick={handleClick}
                        >
                            <ListItemText primary={namespace} />
                            {open[namespace] ? <ExpandLess /> : <ExpandMore />}
                        </ListItem>
                        <Collapse
                            key={`collapse-${namespace}`}
                            in={open[namespace]}
                            timeout="auto"
                            unmountOnExit
                        >
                            <List
                                key={`list-${namespace}`}
                                component="div"
                                disablePadding
                            >
                                {entry.map((x) => {
                                    const s = props.services[x];
                                    return (
                                        <ListItem
                                            className={classes.nested}
                                            key={s.fullName}
                                        >
                                            <SidebarItem {...s} />
                                        </ListItem>
                                    );
                                })}
                            </List>
                        </Collapse>
                    </React.Fragment>
                );
            })}
        </List>
    );
};

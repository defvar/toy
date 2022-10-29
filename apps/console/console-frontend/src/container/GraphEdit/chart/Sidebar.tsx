import * as React from "react";
import List from "@mui/material/List";
import ListItem from "@mui/material/ListItem";
import ListItemText from "@mui/material/ListItemText";
import Collapse from "@mui/material/Collapse";
import ExpandLess from "@mui/icons-material/ExpandLess";
import ExpandMore from "@mui/icons-material/ExpandMore";
import { SidebarItem } from "./SidebarItem";
import { PortType } from "../../../modules/graphEdit/types";
import { styled } from "@mui/material/styles";

const NestedListItem = styled(ListItem)(({ theme }) => ({
    paddingLeft: theme.spacing(3),
}));

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
    height?: number | string;
}

export const Sidebar = (props: SidebarProps) => {
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

    const { height } = props;

    return (
        <List sx={{ height, overflowY: "auto" }}>
            {Object.entries(props.namespaces).map(([namespace, entry]) => {
                let displayNameSpace = namespace.replace("plugin.", "");
                return (
                    <React.Fragment key={namespace}>
                        <ListItem
                            key={`header-${namespace}`}
                            data-namespace={namespace}
                            button
                            onClick={handleClick}
                        >
                            <ListItemText
                                primary={displayNameSpace}
                                primaryTypographyProps={{ variant: "body2" }}
                            />
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
                                        <NestedListItem key={s.fullName}>
                                            <SidebarItem {...s} />
                                        </NestedListItem>
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

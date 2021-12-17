import * as React from "react";
import { Theme, useTheme, styled, CSSObject } from "@mui/material/styles";
import MuiDrawer from "@mui/material/Drawer";
import List from "@mui/material/List";
import ListItem from "@mui/material/ListItem";
import ListItemIcon from "@mui/material/ListItemIcon";
import ListItemText from "@mui/material/ListItemText";
import IconButton from "@mui/material/IconButton";
import ChevronLeftIcon from "@mui/icons-material/ChevronLeft";
import ChevronRightIcon from "@mui/icons-material/ChevronRight";
import DrawerHeader from "./DrawerHeader";
import { Divider } from "@mui/material";

const openedMixin = (theme: Theme): CSSObject => ({
    width: 240,
    transition: theme.transitions.create("width", {
        easing: theme.transitions.easing.sharp,
        duration: theme.transitions.duration.enteringScreen,
    }),
    overflowX: "hidden",
});

const closedMixin = (theme: Theme): CSSObject => ({
    transition: theme.transitions.create("width", {
        easing: theme.transitions.easing.sharp,
        duration: theme.transitions.duration.leavingScreen,
    }),
    overflowX: "hidden",
    width: `calc(${theme.spacing(7)} + 1px)`,
    [theme.breakpoints.up("sm")]: {
        width: `calc(${theme.spacing(9)} + 1px)`,
    },
});

const Drawer = styled(MuiDrawer, {
    shouldForwardProp: (prop) => prop !== "open",
})(({ theme, open }) => ({
    width: 240,
    flexShrink: 0,
    whiteSpace: "nowrap",
    boxSizing: "border-box",
    ...(open && {
        ...openedMixin(theme),
        "& .MuiDrawer-paper": openedMixin(theme),
    }),
    ...(!open && {
        ...closedMixin(theme),
        "& .MuiDrawer-paper": closedMixin(theme),
    }),
}));

export interface SideMenuProps {
    width: string | number;
    open: boolean;
    options: {
        key: string;
        display: string;
        icon?: React.ReactNode;
    }[];
    onDrawerClose: () => void;
    onMenuItemChange: (key: string) => void;
}

export const SideMenu = (props: SideMenuProps): JSX.Element => {
    const theme = useTheme();

    const [selectedIndex, setSelectedIndex] = React.useState("");
    const handleListItemClick = (key: string): void => {
        setSelectedIndex((prev) => {
            if (prev != key) {
                props.onMenuItemChange(key);
            }
            return key;
        });
    };

    return (
        <Drawer variant="permanent" anchor="left" open={props.open}>
            <DrawerHeader>
                <IconButton onClick={props.onDrawerClose} size="large">
                    {theme.direction === "ltr" ? (
                        <ChevronLeftIcon />
                    ) : (
                        <ChevronRightIcon />
                    )}
                </IconButton>
            </DrawerHeader>
            <Divider />
            <List>
                {props.options.map((option) => {
                    return (
                        <ListItem
                            button
                            key={option.key}
                            selected={selectedIndex === option.key}
                            onClick={(): void =>
                                handleListItemClick(option.key)
                            }
                        >
                            <ListItemIcon>{option.icon}</ListItemIcon>
                            <ListItemText primary={option.display} />
                        </ListItem>
                    );
                })}
            </List>
        </Drawer>
    );
};

export default SideMenu;

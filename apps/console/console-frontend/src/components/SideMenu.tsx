import * as React from "react";
import { Theme, useTheme } from "@mui/material/styles";
import makeStyles from '@mui/styles/makeStyles';
import Drawer from "@mui/material/Drawer";
import List from "@mui/material/List";
import ListItem from "@mui/material/ListItem";
import ListItemIcon from "@mui/material/ListItemIcon";
import ListItemText from "@mui/material/ListItemText";
import IconButton from "@mui/material/IconButton";
import ChevronLeftIcon from "@mui/icons-material/ChevronLeft";
import ChevronRightIcon from "@mui/icons-material/ChevronRight";

const useStyles = makeStyles<Theme, SideMenuProps>((theme: Theme) => ({
    drawer: (props) => ({
        width: props.width,
        flexShrink: 0,
    }),
    drawerPaper: (props) => ({
        width: props.width,
    }),
    drawerContainer: {
        overflow: "auto",
    },
    drawerHeader: {
        display: "flex",
        alignItems: "center",
        padding: theme.spacing(0, 1),
        // necessary for content to be below app bar
        ...theme.mixins.toolbar,
        justifyContent: "flex-end",
    },
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
    const classes = useStyles(props);
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
        <Drawer
            className={classes.drawer}
            variant="persistent"
            anchor="left"
            open={props.open}
            classes={{
                paper: classes.drawerPaper,
            }}
        >
            <div className={classes.drawerHeader}>
                <IconButton onClick={props.onDrawerClose} size="large">
                    {theme.direction === "ltr" ? (
                        <ChevronLeftIcon />
                    ) : (
                        <ChevronRightIcon />
                    )}
                </IconButton>
            </div>
            <div className={classes.drawerContainer}>
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
            </div>
        </Drawer>
    );
};

export default SideMenu;

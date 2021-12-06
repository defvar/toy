import * as React from "react";
import clsx from "clsx";
import { Theme } from "@mui/material/styles";
import createStyles from "@mui/styles/createStyles";
import makeStyles from "@mui/styles/makeStyles";
import { SideMenu } from "../components/SideMenu";
import AppBar from "@mui/material/AppBar";
import Toolbar from "@mui/material/Toolbar";
import Typography from "@mui/material/Typography";
import IconButton from "@mui/material/IconButton";
import MenuIcon from "@mui/icons-material/Menu";
import { useHistory } from "react-router-dom";
import TimelineIcon from "@mui/icons-material/Timeline";
import WidgetsIcon from "@mui/icons-material/Widgets";
import DesktopWindowsIcon from "@mui/icons-material/DesktopWindows";
import ExitToAppIcon from "@mui/icons-material/ExitToApp";
import AccountCircle from "../components/AccountCircle";

const drawerWidth = 240;

const useStyles = makeStyles((theme: Theme) =>
    createStyles({
        root: {
            flexGrow: 1,
            display: "flex",
        },
        content: {
            flexGrow: 1,
            backgroundColor: theme.palette.background.default,
            // paddingTop: theme.spacing(2),
            padding: theme.spacing(3),
            transition: theme.transitions.create("margin", {
                easing: theme.transitions.easing.sharp,
                duration: theme.transitions.duration.leavingScreen,
            }),
            marginLeft: -drawerWidth,
        },
        contentShift: {
            transition: theme.transitions.create("margin", {
                easing: theme.transitions.easing.easeOut,
                duration: theme.transitions.duration.enteringScreen,
            }),
            marginLeft: 0,
        },
        appBar: {
            transition: theme.transitions.create(["margin", "width"], {
                easing: theme.transitions.easing.sharp,
                duration: theme.transitions.duration.leavingScreen,
            }),
        },
        appBarShift: {
            width: `calc(100% - ${drawerWidth}px)`,
            marginLeft: drawerWidth,
            transition: theme.transitions.create(["margin", "width"], {
                easing: theme.transitions.easing.easeOut,
                duration: theme.transitions.duration.enteringScreen,
            }),
        },
        menuButton: {
            marginRight: theme.spacing(2),
        },
        hide: {
            display: "none",
        },
        drawerHeader: {
            display: "flex",
            alignItems: "center",
            padding: theme.spacing(0, 1),
            // necessary for content to be below app bar
            ...theme.mixins.toolbar,
            justifyContent: "flex-end",
        },
    })
);

export interface AppDrawerProps {
    children: React.ReactNode;
}

const menuOptions = (history) => {
    return {
        width: drawerWidth,
        options: [
            { key: "top", display: "top", icon: <DesktopWindowsIcon /> },
            { key: "timeline", display: "timeline", icon: <TimelineIcon /> },
            { key: "graphs", display: "graphs", icon: <WidgetsIcon /> },
        ],
        onMenuItemChange: (key: string): void => {
            switch (key) {
                case "top":
                    history.push("/");
                    break;
                case "timeline":
                    history.push("/timeline");
                    break;
                case "graphs":
                    history.push("/graphs");
                    break;
                default:
                    break;
            }
        },
    };
};

const accountCircleProps = (history) => {
    return {
        options: [
            { key: "signOut", display: "sign out", icon: <ExitToAppIcon /> },
        ],
        onMenuItemClick: (key: string): void => {
            switch (key) {
                case "signOut":
                    history.push("/signOut");
                    break;
                default:
                    break;
            }
        },
    };
};

const AppDrawer = (props: AppDrawerProps): JSX.Element => {
    const classes = useStyles();
    const history = useHistory();
    const [open, setOpen] = React.useState(true);

    const handleDrawerOpen = (): void => {
        setOpen(true);
    };

    const handleDrawerClose = (): void => {
        setOpen(false);
    };

    return (
        <div className={classes.root}>
            <AppBar
                position="fixed"
                className={clsx(classes.appBar, {
                    [classes.appBarShift]: open,
                })}
            >
                <Toolbar>
                    <IconButton
                        color="inherit"
                        aria-label="open drawer"
                        onClick={handleDrawerOpen}
                        edge="start"
                        className={clsx(
                            classes.menuButton,
                            open && classes.hide
                        )}
                        size="large"
                    >
                        <MenuIcon />
                    </IconButton>
                    <Typography variant="h6" noWrap style={{ flexGrow: 1 }}>
                        graph system
                    </Typography>
                    <AccountCircle {...accountCircleProps(history)} />
                </Toolbar>
            </AppBar>
            <SideMenu
                open={open}
                onDrawerClose={handleDrawerClose}
                {...menuOptions(history)}
            />
            <main
                className={clsx(classes.content, {
                    [classes.contentShift]: open,
                })}
            >
                <div className={classes.drawerHeader} />
                {props.children}
            </main>
        </div>
    );
};

export default AppDrawer;

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
import { NavigateFunction, useNavigate } from "react-router-dom";
import TimelineIcon from "@mui/icons-material/Timeline";
import WidgetsIcon from "@mui/icons-material/Widgets";
import DesktopWindowsIcon from "@mui/icons-material/DesktopWindows";
import ExitToAppIcon from "@mui/icons-material/ExitToApp";
import AccountCircle from "../components/AccountCircle";
import Box from "@mui/material/Box";
import DrawerHeader from "../components/DrawerHeader";

const drawerWidth = 240;

const useStyles = makeStyles((theme: Theme) =>
    createStyles({
        appBar: {
            zIndex: theme.zIndex.drawer + 1,
            transition: theme.transitions.create(["margin", "width"], {
                easing: theme.transitions.easing.sharp,
                duration: theme.transitions.duration.leavingScreen,
            }),
        },
        appBarShift: {
            width: `calc(100% - ${drawerWidth}px)`,
            marginLeft: drawerWidth,
            transition: theme.transitions.create(["margin", "width"], {
                easing: theme.transitions.easing.sharp,
                duration: theme.transitions.duration.enteringScreen,
            }),
        },
    })
);

export interface AppDrawerProps {
    children: React.ReactNode;
}

const menuOptions = (navigate: NavigateFunction) => {
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
                    navigate("/");
                    break;
                case "timeline":
                    navigate("/timeline");
                    break;
                case "graphs":
                    navigate("/graphs");
                    break;
                default:
                    break;
            }
        },
    };
};

const accountCircleProps = (navigate: NavigateFunction) => {
    return {
        options: [
            { key: "signOut", display: "sign out", icon: <ExitToAppIcon /> },
        ],
        onMenuItemClick: (key: string): void => {
            switch (key) {
                case "signOut":
                    navigate("/signOut");
                    break;
                default:
                    break;
            }
        },
    };
};

const AppDrawer = (props: AppDrawerProps): JSX.Element => {
    const classes = useStyles();
    const navigate = useNavigate();
    const [open, setOpen] = React.useState(true);

    const handleDrawerOpen = (): void => {
        setOpen(true);
    };

    const handleDrawerClose = (): void => {
        setOpen(false);
    };

    return (
        <Box sx={{ display: "flex", flexGrow: 1 }}>
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
                        sx={{
                            marginRight: "36px",
                            ...(open && { display: "none" }),
                        }}
                    >
                        <MenuIcon />
                    </IconButton>
                    <Typography variant="h6" noWrap style={{ flexGrow: 1 }}>
                        graph system
                    </Typography>
                    <AccountCircle {...accountCircleProps(navigate)} />
                </Toolbar>
            </AppBar>
            <SideMenu
                open={open}
                onDrawerClose={handleDrawerClose}
                {...menuOptions(navigate)}
            />
            <Box component="main" sx={{ flexGrow: 1, p: 3 }}>
                <DrawerHeader />
                {props.children}
            </Box>
        </Box>
    );
};

export default AppDrawer;

import * as React from "react";
import clsx from 'clsx';
import { createStyles, Theme, makeStyles } from '@material-ui/core/styles';
import { SideMenu } from "../components/SideMenu";
import Grid from '@material-ui/core/Grid';
import CssBaseline from '@material-ui/core/CssBaseline';
import AppBar from '@material-ui/core/AppBar';
import Toolbar from '@material-ui/core/Toolbar';
import Typography from '@material-ui/core/Typography';
import IconButton from '@material-ui/core/IconButton';
import MenuIcon from '@material-ui/icons/Menu';
import { useHistory } from 'react-router-dom';
import TimelineIcon from '@material-ui/icons/Timeline';
import WidgetsIcon from '@material-ui/icons/Widgets';
import DesktopWindowsIcon from '@material-ui/icons/DesktopWindows';

const drawerWidth = 240;

const useStyles = makeStyles((theme: Theme) =>
    createStyles({
        root: {
            flexGrow: 1,
        },
        content: {
            flexGrow: 1,
            backgroundColor: theme.palette.background.default,
            padding: theme.spacing(3),
            transition: theme.transitions.create('margin', {
                easing: theme.transitions.easing.sharp,
                duration: theme.transitions.duration.leavingScreen,
            }),
            marginLeft: -drawerWidth,
        },
        contentShift: {
            transition: theme.transitions.create('margin', {
                easing: theme.transitions.easing.easeOut,
                duration: theme.transitions.duration.enteringScreen,
            }),
            marginLeft: 0,
        },
        appBar: {
            transition: theme.transitions.create(['margin', 'width'], {
                easing: theme.transitions.easing.sharp,
                duration: theme.transitions.duration.leavingScreen,
            }),
        },
        appBarShift: {
            width: `calc(100% - ${drawerWidth}px)`,
            marginLeft: drawerWidth,
            transition: theme.transitions.create(['margin', 'width'], {
                easing: theme.transitions.easing.easeOut,
                duration: theme.transitions.duration.enteringScreen,
            }),
        },
        menuButton: {
            marginRight: theme.spacing(2),
        },
        hide: {
            display: 'none',
        },
        drawerHeader: {
            display: 'flex',
            alignItems: 'center',
            padding: theme.spacing(0, 1),
            // necessary for content to be below app bar
            ...theme.mixins.toolbar,
            justifyContent: 'flex-end',
        },
    }),
);

export interface AppDrawerProps {
    children: React.ReactNode
}

const menuOptions = (history) => {
    return {
        width: drawerWidth,
        options: [
            { key: 'top', display: 'top', icon: (<DesktopWindowsIcon />) },
            { key: 'timeline', display: 'timeline', icon: (<TimelineIcon />) },
            { key: 'graphs', display: 'graphs', icon: (<WidgetsIcon />) },
        ],
        onMenuItemChange: (key: string) => {
            switch (key) {
                case 'top':
                    history.push('/');
                    break;
                case 'timeline':
                    history.push('/timeline');
                    break;
                case 'graphs':
                    history.push('/graphs');
                    break;
                default:
                    break;
            }
        }
    };
};

const AppDrawer = (props: AppDrawerProps) => {
    const classes = useStyles();
    const history = useHistory();
    const [open, setOpen] = React.useState(true);
    const handleDrawerOpen = () => {
        setOpen(true);
    };

    const handleDrawerClose = () => {
        setOpen(false);
    };

    return (
        <div className={classes.root} >
            <CssBaseline />
            <AppBar
                position="fixed"
                className={clsx(classes.appBar, {
                    [classes.appBarShift]: open,
                })}>
                <Toolbar>
                    <IconButton
                        color="inherit"
                        aria-label="open drawer"
                        onClick={handleDrawerOpen}
                        edge="start"
                        className={clsx(classes.menuButton, open && classes.hide)}
                    >
                        <MenuIcon />
                    </IconButton>
                    <Typography variant="h6" noWrap>graph system</Typography>
                </Toolbar>
            </AppBar>
            <Grid container spacing={2}>
                <Grid item xs={2}>
                    <SideMenu open={open} onDrawerClose={handleDrawerClose} {...menuOptions(history)} />
                </Grid>
                <Grid item xs={10}>
                    <main className={clsx(classes.content, {
                        [classes.contentShift]: open,
                    })}>
                        <div className={classes.drawerHeader} />
                        {props.children}
                    </main>
                </Grid>
            </Grid>
        </div>
    )
};

export default AppDrawer;

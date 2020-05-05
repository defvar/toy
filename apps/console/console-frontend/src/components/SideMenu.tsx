import * as React from 'react';
import { Theme, makeStyles, useTheme } from '@material-ui/core/styles';
import Drawer from '@material-ui/core/Drawer';
import Divider from '@material-ui/core/Divider';
import List from '@material-ui/core/List';
import ListItem from '@material-ui/core/ListItem';
import ListItemIcon from '@material-ui/core/ListItemIcon';
import ListItemText from '@material-ui/core/ListItemText';
import ViewModuleIcon from '@material-ui/icons/ViewModule';
import TimelineIcon from '@material-ui/icons/Timeline';
import LabelImportantIcon from '@material-ui/icons/LabelImportant';
import { useHistory } from 'react-router-dom';
import IconButton from '@material-ui/core/IconButton';
import ChevronLeftIcon from '@material-ui/icons/ChevronLeft';
import ChevronRightIcon from '@material-ui/icons/ChevronRight';

const useStyles = makeStyles<Theme, SideMenuProps>((theme: Theme) =>
    ({
        drawer: props => ({
            width: props.width,
            flexShrink: 0,
        }),
        drawerPaper: props => ({
            width: props.width,
        }),
        drawerContainer: {
            overflow: 'auto',
        },
        drawerHeader: {
            display: 'flex',
            alignItems: 'center',
            padding: theme.spacing(0, 1),
            // necessary for content to be below app bar
            ...theme.mixins.toolbar,
            justifyContent: 'flex-end',
        },
    })
);

export interface SideMenuProps {
    width: string | number,
    open: boolean,
    handleDrawerClose: () => void,
}

export const SideMenu = (props: SideMenuProps) => {
    const classes = useStyles(props);
    const history = useHistory();
    const theme = useTheme();

    const [selectedIndex, setSelectedIndex] = React.useState(0);
    const handleListItemClick = (
        event: React.MouseEvent<HTMLDivElement, MouseEvent>,
        index: number,
        url: string,
    ) => {
        setSelectedIndex(index);
        history.push(url);
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
                <IconButton onClick={props.handleDrawerClose}>
                    {theme.direction === 'ltr' ? <ChevronLeftIcon /> : <ChevronRightIcon />}
                </IconButton>
            </div>
            <div className={classes.drawerContainer}>
                <List>
                    <ListItem button key="top" selected={selectedIndex === 1} onClick={(e) => handleListItemClick(e, 1, "/")}>
                        <ListItemIcon><ViewModuleIcon /></ListItemIcon>
                        <ListItemText primary="top" />
                    </ListItem>
                    <ListItem button key="timeline" selected={selectedIndex === 2} onClick={(e) => handleListItemClick(e, 2, "/timeline")}>
                        <ListItemIcon><TimelineIcon /></ListItemIcon>
                        <ListItemText primary="timeline" />
                    </ListItem>
                    <ListItem button key="graphs" selected={selectedIndex === 3} onClick={(e) => handleListItemClick(e, 3, "/graphs")}>
                        <ListItemIcon><LabelImportantIcon /></ListItemIcon>
                        <ListItemText primary="graphs" />
                    </ListItem>
                </List>
                <Divider />
                <List>
                    {['All mail', 'Trash', 'Spam'].map((text, index) => (
                        <ListItem button key={text}>
                            <ListItemIcon>{index % 2 === 0 ? <LabelImportantIcon /> : <TimelineIcon />}</ListItemIcon>
                            <ListItemText primary={text} />
                        </ListItem>
                    ))}
                </List>
            </div>
        </Drawer>
    );
};

export default SideMenu;

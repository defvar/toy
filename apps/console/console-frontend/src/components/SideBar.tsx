import * as React from 'react';
import { createStyles, Theme, makeStyles } from '@material-ui/core/styles';
import Drawer from '@material-ui/core/Drawer';
import Divider from '@material-ui/core/Divider';
import List from '@material-ui/core/List';
import ListItem from '@material-ui/core/ListItem';
import ListItemIcon from '@material-ui/core/ListItemIcon';
import ListItemText from '@material-ui/core/ListItemText';
import Toolbar from '@material-ui/core/Toolbar';
import ViewModuleIcon from '@material-ui/icons/ViewModule';
import TimelineIcon from '@material-ui/icons/Timeline';
import LabelImportantIcon from '@material-ui/icons/LabelImportant';
import { useHistory } from 'react-router-dom';

const drawerWidth = 240;

const useStyles = makeStyles((theme: Theme) =>
    createStyles({
        drawer: {
            width: drawerWidth,
            flexShrink: 0,
        },
        drawerPaper: {
            width: drawerWidth,
        },
        drawerContainer: {
            overflow: 'auto',
        },
    }),
);

export const SideBar = () => {
    const classes = useStyles();
    const history = useHistory();
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
            variant="permanent"
            classes={{
                paper: classes.drawerPaper,
            }}
        >
            <Toolbar />
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

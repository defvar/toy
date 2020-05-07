import * as React from 'react';
import { Theme, makeStyles, useTheme } from '@material-ui/core/styles';
import Drawer from '@material-ui/core/Drawer';
import List from '@material-ui/core/List';
import ListItem from '@material-ui/core/ListItem';
import ListItemIcon from '@material-ui/core/ListItemIcon';
import ListItemText from '@material-ui/core/ListItemText';
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
    options: {
        key: string,
        display: string,
        icon?: React.ReactNode,
    }[],
    onDrawerClose: () => void,
    onMenuItemChange: (key: string) => void,
}

export const SideMenu = (props: SideMenuProps) => {
    const classes = useStyles(props);
    const theme = useTheme();

    const [selectedIndex, setSelectedIndex] = React.useState('');
    const handleListItemClick = (
        key: string,
    ) => {
        setSelectedIndex(prev => {
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
                <IconButton onClick={props.onDrawerClose}>
                    {theme.direction === 'ltr' ? <ChevronLeftIcon /> : <ChevronRightIcon />}
                </IconButton>
            </div>
            <div className={classes.drawerContainer}>
                <List>
                    {
                        props.options.map(option => {
                            return (
                                < ListItem button key={option.key} selected={selectedIndex === option.key} onClick={() => handleListItemClick(option.key)}>
                                    <ListItemIcon>{option.icon}</ListItemIcon>
                                    <ListItemText primary={option.display} />
                                </ListItem>
                            );
                        })
                    }
                </List>
            </div>
        </Drawer >
    );
};

export default SideMenu;

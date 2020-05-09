import * as React from 'react';
import { createStyles, Theme, makeStyles } from '@material-ui/core/styles';
import List from '@material-ui/core/List';
import ListSubheader from '@material-ui/core/ListSubheader';
import ListItem from '@material-ui/core/ListItem';
import ListItemText from '@material-ui/core/ListItemText';
import Collapse from '@material-ui/core/Collapse';
import ExpandLess from '@material-ui/icons/ExpandLess';
import ExpandMore from '@material-ui/icons/ExpandMore';
import { SidebarItem } from './SidebarItem';

const useStyles = makeStyles((theme: Theme) =>
    createStyles({
        root: {
            padding: theme.spacing(0),
            backgroundColor: theme.palette.background.paper,
        },
        nested: {
            paddingLeft: theme.spacing(4),
        },
    }),
);

export interface SidebarProps {
    services: {
        [fullName: string]: {
            fullName: string,
            name: string,
            namespace: string,
            description: string,
            inPort: number,
            outPort: number,
        }
    },
    namespaces: {
        [namespace: string]: string[],
    }
}

export const Sidebar = (props: SidebarProps) => {
    const classes = useStyles();
    const [open, setOpen] = React.useState<{ [namespace: string]: boolean }>({});

    const handleClick = (e: React.MouseEvent<HTMLDivElement, MouseEvent>) => {
        const namespace = e.currentTarget.dataset.namespace;
        setOpen((prev) =>
            ({
                ...prev,
                [namespace]: !prev[namespace]
            })
        );
    };

    return (
        <List
            className={classes.root}
            component="nav"
            aria-labelledby="nested-list-subheader"
            subheader={
                <ListSubheader component="div" id="nested-list-subheader">
                    Services
                </ListSubheader>
            }>
            {
                Object.entries(props.namespaces).map(([namespace, entry]) => {
                    return (
                        <React.Fragment key={namespace}>
                            <ListItem key={`header-${namespace}`} data-namespace={namespace} button onClick={handleClick}>
                                <ListItemText primary={namespace} />
                                {open[namespace] ? <ExpandLess /> : <ExpandMore />}
                            </ListItem>
                            <Collapse key={`collapse-${namespace}`} in={open[namespace]} timeout="auto" unmountOnExit>
                                <List key={`list-${namespace}`} component="div" disablePadding>
                                    {
                                        entry.map(x => {
                                            const s = props.services[x];
                                            return (
                                                < ListItem className={classes.nested} key={s.fullName}>
                                                    <SidebarItem {...s} />
                                                </ListItem>
                                            );
                                        })
                                    }
                                </List>
                            </Collapse>
                        </React.Fragment>
                    );
                })
            }
        </List>
    );
};

import * as React from 'react';
import { createStyles, makeStyles, Theme } from '@material-ui/core/styles';
import Chip from '@material-ui/core/Chip';
import Paper from '@material-ui/core/Paper';

export interface LabelsProps {
    labels: { key: string, display: string, }[],
}

const useStyles = makeStyles((theme: Theme) =>
    createStyles({
        root: {
            display: 'flex',
            justifyContent: 'left',
            flexWrap: 'wrap',
            listStyle: 'none',
            padding: theme.spacing(0.5),
            margin: 0,
            backgroundColor: 'inherit'
        },
        chip: {
            margin: theme.spacing(0.5),
        },
    }),
);

export const LabelChips = (props: LabelsProps) => {
    const classes = useStyles();
    return (
        <Paper component="ul" className={classes.root} elevation={0}>
            {props.labels.map((data) => {
                return (
                    <li key={data.key}>
                        <Chip
                            label={data.display}
                            className={classes.chip}
                        />
                    </li>
                );
            })}
        </Paper>
    );
}

import * as React from 'react';
import { createStyles, makeStyles, Theme } from '@material-ui/core/styles';
import Chip from '@material-ui/core/Chip';
import Paper from '@material-ui/core/Paper';

export interface Label {
    label: string;
}

export interface LabelsProps {
    labels: Label[],
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

export default (props: LabelsProps) => {
    const classes = useStyles();
    const [chipData] = React.useState(props);

    return (
        <Paper component="ul" className={classes.root} elevation={0}>
            {chipData.labels.map((data) => {
                return (
                    <li key={data.label}>
                        <Chip
                            label={data.label}
                            className={classes.chip}
                        />
                    </li>
                );
            })}
        </Paper>
    );
}
